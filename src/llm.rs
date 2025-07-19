pub mod llm {

    use std::error::Error;

    use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};

    pub fn init(host: String, port: u16) -> Ollama {
        Ollama::new(host, port)
    }

    pub async fn generate_response(
        ollama: &Ollama,
        model: Option<String>,
        prompt: String,
    ) -> Result<String, Box<dyn Error>> {
        let model = model.unwrap_or_else(|| "llama2:latest".to_string());
        let request = GenerationRequest::new(model, prompt);
        let res = ollama.generate(request).await?; // Use '?' for error propagation

        Ok(res.response)
    }
}
