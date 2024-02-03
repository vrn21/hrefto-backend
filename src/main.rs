use std::net::TcpListener;
use axum::{routing::get, Json, Router};

#[tokio::main]

async fn main(){
    let app = Router::new().route("/api/hello", get(handle_api));

    let listner = TcpListener::bind("127.0.0.1.7878");

    println!("Server started at port 7878");

    axum::serve(listner, app.into_make_service()).await.unwrap();
}

pub fn handle_api(){
    const message:&str = "Hello world";

    let json_resp = serde_json::json!({
        "status": "OK",
        "Message":message,
    });

    Json(json_resp);
}