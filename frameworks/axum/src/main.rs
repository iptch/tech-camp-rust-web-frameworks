use axum::extract::Path;
use axum::extract::Query;
use axum::extract::Request;
use axum::extract::State;
use axum::routing::get;
use axum::{async_trait, Json};
use axum::{extract::FromRequest, http::StatusCode, routing::post, Router};
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use tokio::fs;
use tokio::io;

mod config;

// payloads

#[derive(Deserialize)]
struct TextPayload {
    data: String,
}
#[derive(Deserialize)]
struct SearchParams {
    term: String,
}

#[derive(Serialize)]
struct SearchResponse {
    found: bool,
}

#[derive(Serialize)]
struct ErrorPayload {
    error: String,
}

#[async_trait]
impl<S> FromRequest<S> for TextPayload
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<ErrorPayload>);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<TextPayload>::from_request(req, state).await {
            Err(rejection) => Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorPayload {
                    error: rejection.body_text(),
                }),
            )),
            Ok(Json(text_payload)) => Ok(text_payload),
        }
    }
}

// main

struct AppState {
    _client: mongodb::Client,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let config_text = fs::read_to_string("config.toml").await?;
    let config: config::Config = toml::from_str(&config_text).unwrap();
    println!("{:?}", config);

    let client = mongodb::Client::with_uri_str(config.mongodb.host)
        .await
        .unwrap();

    let shared_state = std::sync::Arc::new(AppState { _client: client });

    // build our application with a single route
    let app = Router::new()
        .route("/texts", post(post_text))
        .route("/texts/:text_id", get(get_text).delete(delete_text))
        .route("/texts/:text_id/search", get(search_text))
        .with_state(shared_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn post_text(State(_state): State<Arc<AppState>>, text_payload: TextPayload) {
    println!("post \"{}\"", text_payload.data);
}
async fn get_text(State(_state): State<Arc<AppState>>, Path(text_id): Path<String>) {
    println!("get {}", text_id);
}
async fn delete_text(State(_state): State<Arc<AppState>>, Path(text_id): Path<String>) {
    println!("delete {}", text_id);
}

async fn search_text(
    State(_state): State<Arc<AppState>>,
    Path(text_id): Path<String>,
    params: Query<SearchParams>,
) -> Result<Json<SearchResponse>, StatusCode> {
    println!("search {} for {}", text_id, params.term);
    Err(StatusCode::BAD_REQUEST)
}
