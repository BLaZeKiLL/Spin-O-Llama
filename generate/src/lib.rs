use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
    llm::{infer_with_options, InferencingModel, InferencingParams},
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct OllamaOptions {
    pub num_predict: u32,
    pub temperature: f32,
    pub top_p: f32,
    pub repeat_penalty: f32,
}

impl Default for OllamaOptions {
    fn default() -> Self {
        Self {
            num_predict: 128,
            temperature: 0.8,
            top_p: 0.9,
            repeat_penalty: 1.1,
        }
    }
}

#[derive(Deserialize)]
pub struct OllamaGenerateRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
    pub system: Option<String>,
    pub options: Option<OllamaOptions>,
}

// Theres other stuff also look at - https://github.com/jmorganca/ollama/blob/main/docs/api.md#response-1
#[derive(Serialize)]
pub struct OllamaGenerateResponse {
    pub model: String,
    pub response: String,
    // context
    pub done: bool,
}

const PROMPT: &str = r#"
<SYS>
{SYSTEM}
</SYS>

User: {USER}
"#;

/// A simple Spin HTTP component.
#[http_component]
fn handle_generate(req: Request) -> Result<Response> {
    println!("Handling request to {:?}", req.header("spin-full-url"));

    let request: OllamaGenerateRequest = serde_json::from_slice(req.body())?;

    let options = request.options.unwrap_or_default();

    let model = match request.model.as_str() {
        "codellama-instruct" => InferencingModel::CodellamaInstruct,
        "llama2-chat" => InferencingModel::Llama2Chat,
        _ => panic!("Unsupported model")
    };

    let prompt = match request.system {
        Some(sys) => PROMPT
            .replace("{SYSTEM}", sys.as_str())
            .replace("{USER}", request.prompt.as_str()),
        None => request.prompt,
    };

    println!("PROMPT: {:?}", prompt.as_str());

    let result = infer_with_options(
        model,
        prompt.as_str(),
        InferencingParams {
            max_tokens: options.num_predict,
            repeat_penalty: options.repeat_penalty,
            repeat_penalty_last_n_token_count: 64,
            temperature: options.temperature,
            top_k: 40,
            top_p: options.top_p,
        },
    );

    return match result {
        Ok(r) => {
            println!("RESULT: {:?}", &r.text);

            let response = OllamaGenerateResponse {
                model: String::from("llama2-chat:13b"),
                response: r.text,
                done: true,
            };

            Ok(spin_sdk::http::Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(Some(serde_json::to_string(&response)?))
                .build())
        }
        Err(_) => {
            println!("LLM ERROR");

            Ok(spin_sdk::http::Response::builder()
                .status(500)
                .header("content-type", "application/json")
                .build())
        }
    };
}
