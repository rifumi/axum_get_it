use axum::{
    routing::{
        get
    }
    , Router
    , response::{
        Html
    }
};
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

async fn root() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}

#[cfg(test)]
mod tests {
    use super::root;
    #[tokio::test]
    async fn test_root() {
        let s1 = root().await;
        let s = s1.0.to_owned();
        assert_eq!(s, "<h1>Hello, world!</h1>");
    }
}