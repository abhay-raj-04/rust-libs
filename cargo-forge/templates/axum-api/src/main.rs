use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, instrument};
use tracing_subscriber;

#[derive(Clone)]
struct AppState {
    // Add your app state here (database connections, etc.)
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    service: String,
}

#[derive(Deserialize)]
struct CreateItemRequest {
    name: String,
}

#[derive(Serialize)]
struct CreateItemResponse {
    id: u32,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    info!("Starting {{ project_name }} server...");

    // Initialize app state
    let state = AppState {};

    // Build our application with routes
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/items", post(create_item))
        .with_state(Arc::new(state));

    // Start server
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    info!("Server listening on http://127.0.0.1:3000");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn root() -> &'static str {
    "Welcome to {{ project_name }} API!"
}

#[instrument]
async fn health(State(_state): State<Arc<AppState>>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "{{ project_name }}".to_string(),
    })
}

#[instrument]
async fn create_item(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<CreateItemRequest>,
) -> Result<Json<CreateItemResponse>, StatusCode> {
    // TODO: Implement actual item creation logic
    let item = CreateItemResponse {
        id: 1,
        name: payload.name,
    };
    
    Ok(Json(item))
}