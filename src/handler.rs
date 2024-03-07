use axum::handler::Handler;
use axum::{routing::get, Json};
pub async fn handle_api() -> Json<serde_json::Value> { // Add async keyword and return type
    const message:&str = "Hello world";

    let json_resp = serde_json::json!({
        "status": "OK",
        "Message":message,
    });

    Json(json_resp)
}