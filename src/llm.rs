pub mod llm {
    use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};

    pub async fn start_ollama() {
        let ollama = Ollama::new("http://localhost".to_string(), 11434);
        let model = "llama2:latest".to_string();
        let prompt = "Why is the sky blue?".to_string();
        let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

        if let Ok(res) = res {
            println!("{}", res.response);
        }
    }

    pub fn print() {
        println!("HELLO WORLD");
    }
}
