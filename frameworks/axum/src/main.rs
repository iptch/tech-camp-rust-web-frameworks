use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::routing::get;
use axum::Json;
use axum::{http::StatusCode, routing::post, Router};
use entries::TextSearchEntry;
use tower_http::trace::TraceLayer;
use std::env;
use std::sync::Arc;

mod entries;
mod payloads;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mongodb_host = env::var("MONGODB_HOST")?;

    tracing_subscriber::fmt::init();

    let client = mongodb::Client::with_uri_str(mongodb_host).await.unwrap();

    let shared_state = std::sync::Arc::new(state::MongoAppState::new(client));

    // build our application with a single route
    let app = Router::new()
        .route("/texts", post(post_text))
        .route("/texts/:text_id", get(get_text).delete(delete_text))
        .route("/texts/:text_id/search", get(search_text))
        .layer(TraceLayer::new_for_http())
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

    let id = uuid::Uuid::new_v4();
    let entry = entries::TextEntry {
        id,
        data: text_payload.data,
    };
    match state.client().insert_one(entry).await {
        Ok(_) => Ok((StatusCode::CREATED, Json(payloads::InsertedResponse { id }))),
        Err(_error) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(payloads::ErrorResponse {
                    error: "error inserting into mongodb",
                }),
            ))
        }
    }
}
async fn get_text(
    State(state): State<Arc<state::MongoAppState>>,
    Path(text_id): Path<String>,
) -> Result<Json<payloads::TextPayload>, (StatusCode, Json<payloads::ErrorResponse>)> {
    let Ok(id) = uuid::Uuid::try_parse(&text_id) else {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(payloads::ErrorResponse {
                error: "invalid uuid",
            }),
        ));
    };
    match state
        .client()
        .find_one(bson::to_document(&TextSearchEntry { id }).unwrap())
        .await
    {
        Ok(Some(result)) => Ok(Json(payloads::TextPayload { data: result.data })),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(payloads::ErrorResponse { error: "not found" }),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(payloads::ErrorResponse {
                error: "error with mongodb",
            }),
        )),
    }
}
async fn delete_text(
    State(state): State<Arc<state::MongoAppState>>,
    Path(text_id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<payloads::ErrorResponse>)> {
    let Ok(id) = uuid::Uuid::try_parse(&text_id) else {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(payloads::ErrorResponse {
                error: "invalid uuid",
            }),
        ));
    };
    match state
        .client()
        .delete_one(bson::to_document(&TextSearchEntry { id }).unwrap())
        .await
    {
        Ok(result) => {
            if result.deleted_count == 0 {
                Err((
                    StatusCode::NOT_FOUND,
                    Json(payloads::ErrorResponse { error: "not found" }),
                ))
            } else {
                // we could probably check to make sure the count is 1 here
                Ok(StatusCode::NO_CONTENT)
            }
        }
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(payloads::ErrorResponse {
                error: "error with mongodb",
            }),
        )),
    }
}

async fn search_text(
    State(state): State<Arc<state::MongoAppState>>,
    Path(text_id): Path<String>,
    Query(params): Query<payloads::SearchParams>,
) -> Result<Json<payloads::SearchResponse>, (StatusCode, Json<payloads::ErrorResponse>)> {
    let Ok(id) = uuid::Uuid::try_parse(&text_id) else {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(payloads::ErrorResponse {
                error: "invalid uuid",
            }),
        ));
    };
    match state
        .client()
        .find_one(bson::to_document(&TextSearchEntry { id }).unwrap())
        .await
    {
        Ok(Some(result)) => {
            let found = result.data.contains(&params.term);
            Ok(Json(payloads::SearchResponse { found }))
        }
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(payloads::ErrorResponse { error: "not found" }),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(payloads::ErrorResponse {
                error: "error with mongodb",
            }),
        )),
    }
}
