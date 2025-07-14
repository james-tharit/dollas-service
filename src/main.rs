use axum::{
    Json, Router,
    http::{Method, StatusCode, header::CONTENT_TYPE},
    routing::{get, post},
};

use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

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

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    // our router
    let app = Router::new()
        .route("/", get(health_check))
        .route("/authenticate", post(validate_password))
        .layer(ServiceBuilder::new().layer(cors));

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

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
