use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct TextEntry {
    #[serde(rename = "_id", with = "uuid::serde::simple")]
    pub id: uuid::Uuid,
    pub data: String,
}

#[derive(Serialize)]
pub struct TextSearchEntry {
    #[serde(rename = "_id", with = "uuid::serde::simple")]
    pub id: uuid::Uuid,
}

