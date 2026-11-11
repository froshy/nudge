use axum::{
    Json, Router,
    response::Html,
    routing::{get, post},
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Credentials {
    key_id: String,
    secret: String,
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

async fn account(Json(creds): Json<Credentials>) -> String {
    format!(
        "Received key ID {} (secret is {} characters)",
        creds.key_id,
        creds.secret.len()
    )
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/api/account", post(account));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("Listening on http://127.0.0.1:3001");
    axum::serve(listener, app).await.unwrap();
}
