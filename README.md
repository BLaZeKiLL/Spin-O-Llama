# Spin-O-Llama
[![build](https://github.com/BLaZeKiLL/Spin-O-Llama/actions/workflows/build.yml/badge.svg)](https://github.com/BLaZeKiLL/Spin-O-Llama/actions/workflows/build.yml)

[Ollama api](https://github.com/jmorganca/ollama/blob/main/docs/api.md) implementation for spin

> :warning: **Proof of concept**: This project is not production ready

## Quick Start
- [Install spin](https://developer.fermyon.com/spin/v2/install)
- login to fermeyon cloud
    ```
    spin login
    ```
- clone this repository
    ```
    git clone https://github.com/BLaZeKiLL/Spin-O-Llama.git
    cd Spin-O-Llama
    ```
- build
    ```
    spin build
    ```
- deploy
    ```
    spin deploy
    ```

Routes implemented
- [/api/generate](https://github.com/jmorganca/ollama/blob/main/docs/api.md#generate-a-completion)

    supported request body
    ```json
    {
        "model": "<supported-model>",
        "prompt": "<input prompt>",
        "stream": false // streaming not supported, has no impact
        "options": { // options are optional
            "num_predict": 128,
            "temperature": 0.8,
            "top_p": 0.9,
            "repeat_penalty": 1.1
        } // default values provided above
    }
    ```

    response body
    ```json
    {
        "model": "<model-id>",
        "response": "<output>",
        "done": true
    }
    ```
- [/api/embeddings](https://github.com/jmorganca/ollama/blob/main/docs/api.md#generate-embeddings)

    supported request body
    ```json
    {
        "model": "<model-id>", // doesn't matter for now will always use all-minilm-l6-v2
        "prompt": "<input>"
    }
    ```

    response body
    ```json
    {
        "embedding": [<float array>]
    }
    ```

Model compatibility
- generate - llama2-chat, codellama-instruct
- embeddings - all-minilm-l6-v2

## Contributing
Contributions are welcome for further implementation of the Ollama api that is supported on the spin runtime.