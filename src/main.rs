use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

mod llm;

#[tokio::main]
async fn main() {
    static SECRET_KEYWORD: &'static str = "BACCANO";

    #[derive(Deserialize, Debug)]
    struct AuthRequest {
        secret_keyword: String,
    }

    #[derive(Serialize)]
    struct AuthResponse {
        token: bool,
    }

    #[derive(Serialize, Deserialize)]
    struct LLMResponse {
        message: String,
        model: Option<String>,
    }
    // Configure the CORS layer
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::exact("http://localhost:5173".parse().unwrap()))
        // Allow all standard methods (GET, POST, PUT, DELETE, etc.)
        .allow_methods(Any)
        // Allow all headers
        .allow_headers(Any);

    // our router
    let app = Router::new()
        .route("/", get(health_check))
        .route("/authenticate", post(validate_password))
        .route("/ollama", post(ollama_init))
        .layer(cors);

    // which calls one of these handlers
    async fn health_check() -> &'static str {
        "healthy"
    }

    async fn validate_password(
        Json(payload): Json<AuthRequest>,
    ) -> Result<Json<AuthResponse>, StatusCode> {
        println!("Request coming in");
        println!("{:?}", payload);
        if payload.secret_keyword.to_uppercase() == SECRET_KEYWORD {
            Ok(Json(AuthResponse { token: true }))
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }

    async fn ollama_init(
        Json(payload): Json<LLMResponse>,
    ) -> Result<Json<LLMResponse>, StatusCode> {
        println!("init Ollama");
        let ollama = crate::llm::llm::init("http://localhost".to_string(), 11434);
        let res = crate::llm::llm::generate_response(&ollama, payload.model, payload.message).await;

        match res {
            Ok(res_message) => {
                println!("SUCCESS!, message is {:?}", res_message);
                Ok(Json(LLMResponse {
                    message: res_message,
                    model: None,
                }))
            }
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
