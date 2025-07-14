use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

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
    // our router
    let app = Router::new()
        .route("/", get(health_check))
        .route("/authenticate", post(validate_password));

    // which calls one of these handlers
    async fn health_check() -> &'static str {
        "healthy"
    }

    async fn validate_password(
        Json(payload): Json<AuthRequest>,
    ) -> Result<Json<AuthResponse>, StatusCode> {
        println!("{:?}", payload);
        if payload.secret_keyword.to_uppercase() == SECRET_KEYWORD {
            Ok(Json(AuthResponse { token: true }))
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
