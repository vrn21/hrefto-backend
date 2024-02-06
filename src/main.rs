use std::net::TcpListener;
use axum::{routing::get, Json, Router};
 // Import the Handler trait
use tokio::net::TcpListener as TokioTcpListener; // Remove duplicate import

mod actions;
mod handler;

#[tokio::main]

async fn main(){
    let app = Router::new().route("/api/hello", get(handler::handle_api));

    let listener = TokioTcpListener::bind("127.0.0.1:7878").await.unwrap(); // Use tokio::net::TcpListener::bind

    println!("Server started at port 7878");

    axum::serve(listener, app.into_make_service()).await.unwrap();
}

