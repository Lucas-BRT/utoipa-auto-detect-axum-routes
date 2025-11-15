use std::net::Ipv4Addr;

use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
    serve,
};
use tokio::net::TcpListener;

const PORT: u16 = 3000;

async fn hello() -> (StatusCode, String) {
    (StatusCode::OK, "Hello, world!".to_string())
}

fn main_router() -> Router {
    Router::new().route("/hello", get(hello))
}

async fn setup_tcp_listener() -> TcpListener {
    TcpListener::bind((Ipv4Addr::LOCALHOST, PORT))
        .await
        .expect("failed to bind to TCP port {PORT}")
}

async fn launch_server(listener: TcpListener, router: Router) {
    serve(listener, router).await.unwrap();
}

#[tokio::main]
async fn main() {
    let router = main_router();
    let listener = setup_tcp_listener().await;

    println!(
        "server launched at http://{}:{}/hello",
        Ipv4Addr::LOCALHOST,
        PORT
    );
    launch_server(listener, router).await;
}
