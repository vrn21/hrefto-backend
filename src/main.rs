use std::net::TcpListener;
use axum::{routing::get, Json, Router};
use axum::handler::Handler; // Import the Handler trait
use tokio::net::TcpListener as TokioTcpListener; // Remove duplicate import

#[tokio::main]

async fn main(){
    let app = Router::new().route("/api/hello", get(handle_api));

    let listener = TokioTcpListener::bind("127.0.0.1:7878").await.unwrap(); // Use tokio::net::TcpListener::bind

    println!("Server started at port 7878");

    axum::serve(listener, app.into_make_service()).await.unwrap();
}

pub async fn handle_api() -> Json<serde_json::Value> { // Add async keyword and return type
    const message:&str = "Hello world";

    let json_resp = serde_json::json!({
        "status": "OK",
        "Message":message,
    });

    Json(json_resp)
}