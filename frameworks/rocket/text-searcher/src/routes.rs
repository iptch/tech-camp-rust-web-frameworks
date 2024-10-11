use rocket::http::Status;
use rocket::serde::uuid::Uuid;
use rocket::serde::{
    json::{json, Json, Value},
    Deserialize, Serialize,
};
use rocket_db_pools::{mongodb, Connection, Database};

#[derive(Database)]
#[database("texts")]
pub struct TextsDatabase(mongodb::Client);

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Message<'m> {
    pub data: &'m str,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Text {
    id: Uuid,
    text: String,
}

#[post("/texts", format = "application/json", data = "<msg>")]
pub async fn post_text(db: Connection<TextsDatabase>, msg: Json<Message<'_>>) -> (Status, Value) {
    let id = Uuid::new_v4();
    let collection = db.database("techcamp").collection::<Text>("texts");
    let new_text = Text {
        id,
        text: msg.data.to_string(),
    };

    match collection.insert_one(new_text, None).await {
        Ok(_) => (Status::Created, json!(id)),
        Err(e) => (
            Status::InternalServerError,
            json!({"error": format!("failed to insert text into database: {e}")}),
        ),
    }
}

#[delete("/texts/<uuid>")]
pub async fn delete_text(uuid: Uuid) -> Status {
    println!("got uuid: {uuid}");
    // Success: 204 No Content
    // Invalid UUID -> 400 Bad Request
    // UUID does not exist -> 404 Not Found
    // Server-side error 500 Internal Server Error
    Status::NoContent
}

#[get("/texts/<uuid>")]
pub async fn get_text(uuid: Uuid) -> (Status, Value) {
    // Success: 200 OK
    // Invalid UUID: 400 Bad Request
    // UUID does not exist: 404 Not Found
    // Server side error 500
    (Status::Ok, json!({"data": uuid}))
}

#[get("/texts/<uuid>/search?<term>")]
pub async fn get_search(uuid: String, term: &str) -> (Status, Value) {
    // Success: 200 OK
    // Invalid UUID or term -> 400 Bad Request
    // UUID does not exist -> 404 Not Found
    // Server side error 500
    (
        Status::Ok,
        json!({"found": true, "uuid": uuid, "term": term}),
    )
}
