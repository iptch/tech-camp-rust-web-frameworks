use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::routing::get;
use axum::Json;
use axum::{http::StatusCode, routing::post, Router};
use entries::TextSearchEntry;
use std::sync::Arc;
use tokio::fs;
use tokio::io;

mod config;
mod entries;
mod payloads;
mod state;

// payloads

// main
#[tokio::main]
async fn main() -> io::Result<()> {
    let config_text = fs::read_to_string("config.toml").await?;
    let config: config::Config = toml::from_str(&config_text).unwrap();
    println!("{:?}", config);

    let client = mongodb::Client::with_uri_str(config.mongodb.host)
        .await
        .unwrap();

    let shared_state = std::sync::Arc::new(state::MongoAppState::new(
        client,
        config.mongodb.database,
        config.mongodb.collection,
    ));

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

async fn post_text(
    State(state): State<Arc<state::MongoAppState>>,
    Json(text_payload): Json<payloads::TextPayload>,
) -> Result<
    (StatusCode, Json<payloads::InsertedResponse>),
    (StatusCode, Json<payloads::ErrorResponse>),
> {
    println!("post \"{}\"", text_payload.data);

    let client = &state.client();
    let id = uuid::Uuid::new_v4();
    let entry = entries::TextEntry {
        id,
        data: text_payload.data,
    };
    match client.insert_one(entry).await {
        mongodb::error::Result::Ok(_) => {
            Ok((StatusCode::CREATED, Json(payloads::InsertedResponse { id })))
        }
        mongodb::error::Result::Err(error) => {
            println!("{:?}", error);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(payloads::ErrorResponse {
                    error: "error inserting into mongodb".to_string(),
                }),
            ))
        }
    }
}
async fn get_text(
    State(_state): State<Arc<state::MongoAppState>>,
    Path(text_id): Path<String>,
) -> Result<Json<payloads::TextPayload>, (StatusCode, Json<payloads::ErrorResponse>)> {
    println!("get {}", text_id);
    let client = &_state.client();
    let Ok(id) = uuid::Uuid::try_parse(&text_id) else {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(payloads::ErrorResponse {
                error: "invalid uuid".to_string(),
            }),
        ));
    };
    match client
        .find_one(bson::to_document(&TextSearchEntry { id }).unwrap())
        .await
    {
        mongodb::error::Result::Ok(maybe_result) => match maybe_result {
            Option::Some(result) => Ok(Json(payloads::TextPayload { data: result.data })),
            Option::None => Err((
                StatusCode::NOT_FOUND,
                Json(payloads::ErrorResponse {
                    error: "not found".to_string(),
                }),
            )),
        },
        mongodb::error::Result::Err(_error) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(payloads::ErrorResponse {
                error: "error with mongodb".to_string(),
            }),
        )),
    }
}
async fn delete_text(State(_state): State<Arc<state::MongoAppState>>, Path(text_id): Path<String>) {
    println!("delete {}", text_id);
}

async fn search_text(
    State(_state): State<Arc<state::MongoAppState>>,
    Path(text_id): Path<String>,
    params: Query<payloads::SearchParams>,
) -> Result<Json<payloads::SearchResponse>, StatusCode> {
    println!("search {} for {}", text_id, params.term);
    Err(StatusCode::BAD_REQUEST)
}
