use axum::{routing::get, Router};
use std::net::{SocketAddr, Ipv4Addr};
use std::env;

#[tokio::main]
async fn main() {
    // logging
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    // container port:3000にリクエストを受信した時root()を呼び出す
    let app = Router::new().route("/", get(root));
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, world!\n"
}

#[cfg(test)]
mod tests {
    use super::root;
    #[tokio::test]
    async fn test_root() {
        let s = root().await;
        assert_eq!(s.to_string(), "Hello, world!\n");
    }
}