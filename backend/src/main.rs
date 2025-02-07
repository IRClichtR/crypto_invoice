use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio;
use tokio::net::TcpListener;
use tokio::signal;


#[tokio::main]
async fn main() {
    // main route
    let app = Router::new().route("/", get(hello_world));

    // build server address and tcp_listener
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let tcp_listener = TcpListener::bind(&addr)
        .await
        .unwrap();
    println!("Server started up listening on http://{}", addr);

    axum::serve(tcp_listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

// handle graceful shutting down the server
async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
    println!("Shutdown signal received");
}

async fn hello_world() -> &'static str {
    "Hello Backend!"
}
