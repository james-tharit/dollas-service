use axum::{
    Json, Router,
    http::{Method, StatusCode, header::CONTENT_TYPE},
    routing::{get, post},
};

use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

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

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    // our router
    let app = Router::new()
        .layer(ServiceBuilder::new().layer(cors))
        .route("/", get(health_check))
        .route("/authenticate", post(validate_password))
        .route("/ollama", post(ollama_init));

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
