use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct TextPayload {
    pub data: String,
}
#[derive(Deserialize)]
pub struct SearchParams {
    pub term: String,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub found: bool,
}

#[derive(Serialize)]
pub struct InsertedResponse {
    #[serde(with = "uuid::serde::simple")]
    pub id: uuid::Uuid,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
