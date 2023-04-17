use axum::{
    routing::{
        get
        , post
    }
    , Router
    , response::{
        Html
        ,IntoResponse
    }
    , http::StatusCode
    , Json
};
use serde::{ Deserialize, Serialize };
use std::net::{ SocketAddr, Ipv4Addr };
use std::env;

#[tokio::main]
async fn main() {
    // logging
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    // container port:3000にリクエストを受信した時のrouting設定
    let app = create_app();
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_app() -> Router {
    return Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));
}

async fn root() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct User {
    id: u64,
    username: String
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_root() {
        let s1 = root().await;
        let s = s1.0.to_owned();
        assert_eq!(s, "<h1>Hello, world!</h1>");
    }
    use axum::response::{Json, IntoResponse};
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_create_user() {
        let response = create_user(Json(CreateUser{ username: "Takeshi".to_string() }))
            .await
            .into_response();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body();
        let bytes = hyper::body::to_bytes(body).await.unwrap();
        let user: User = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(user, User { id: 1337, username: "Takeshi".to_string() });
    }

    use axum::{
        body::Body
        ,http::{
            header
            ,Method
            ,Request
        }
    };
    use tower::ServiceExt;
    #[tokio::test]
    async fn should_return_hello_world() {
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app().oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert_eq!(body, "<h1>Hello, world!</h1>");
    }
    #[tokio::test]
    async fn should_return_user_data() {
        let req = Request::builder().uri("/users")
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(r#"{ "username": "Munetaka Murakami" }"#)).unwrap();
        let res = create_app().oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        let user: User = serde_json::from_str(&body).expect("cannot convert User instance.");
        assert_eq!(user, User { id: 1337, username: "Munetaka Murakami".to_string()});
    }
}