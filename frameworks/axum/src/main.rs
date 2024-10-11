use axum::extract::Request;
use axum::{async_trait, Json};
use axum::{
    extract::FromRequest,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
struct TextPayload {
    data: String,
}

#[derive(Deserialize, Serialize)]
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

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/texts", post(post_text));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn post_text(text_payload: TextPayload) {
    println!("{}", text_payload.data);
}
