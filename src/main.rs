use axum::{
    Error, Json, Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    http::StatusCode,
    response::Response,
    routing::{any, get, post},
};
use ollama_rs::Ollama;
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
        .allow_methods(Any)
        .allow_headers(Any);

    // our router
    let app = Router::new()
        .route("/", get(health_check))
        .route("/authenticate", post(validate_password))
        .route("/chat", any(chat_handler))
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

    async fn chat_handler(ws: WebSocketUpgrade) -> Response {
        println!("Initiate webSocket connection");
        let ollama = crate::llm::llm::init("http://localhost".to_string(), 11434);
        ws.on_upgrade(|soc| chat_socket(soc, ollama))
    }

    async fn chat_socket(mut socket: WebSocket, ollama: Ollama) {
        while let Some(msg) = socket.recv().await {
            let response = if let Ok(msg) = msg {
                println!("{:?}", msg.to_text());
                let request: Result<String, Error> = msg.to_text().map(|s| s.to_string());

                let res = crate::llm::llm::generate_response(
                    &ollama,
                    None,
                    match request {
                        Ok(str) => str,
                        Err(e) => format!("error: {}", e),
                    },
                )
                .await;
                match res {
                    Ok(res_message) => {
                        println!("SUCCESS!, message is {:?}", res_message);
                        Ok(res_message)
                    }
                    Err(error) => {
                        println!("{:?}", error);
                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
            } else {
                // client disconnected
                return;
            };

            if socket
                .send(Message::text(match response {
                    Ok(str) => str,
                    Err(e) => format!("error: {}", e),
                }))
                .await
                .is_err()
            {
                // client disconnected
                return;
            }
        }
    }

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
