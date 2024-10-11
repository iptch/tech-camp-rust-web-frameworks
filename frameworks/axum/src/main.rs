use axum::{
    routing::{get, post}, Router
};
use axum::extract::Json;
use serde::Deserialize;

#[derive(Deserialize)]
struct TextPayload {
    data: String
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" })).route("/texts", post(post_text));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn post_text(Json(text_payload): Json<TextPayload>) {
    println!("{}", text_payload.data);
}
