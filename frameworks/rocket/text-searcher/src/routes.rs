use bson::{doc, SerializerOptions};
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
    _id: Uuid,
    text: String,
}

#[allow(deprecated)]
fn uuid_to_bson(uuid: &Uuid) -> bson::Bson {
    // Despite this being deprecated it is currently necessary to make the webserver work,
    // otherwise UUIDs won't be matched when searching or deleting
    let options = SerializerOptions::builder().human_readable(false).build();
    bson::to_bson_with_options(uuid, options).unwrap()
}

#[post("/texts", format = "application/json", data = "<msg>")]
pub async fn post_text(db: Connection<TextsDatabase>, msg: Json<Message<'_>>) -> (Status, Value) {
    let id = Uuid::new_v4();
    let collection = db.database("techcamp").collection::<Text>("texts");
    let new_text = Text {
        _id: id,
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
pub async fn get_text(db: Connection<TextsDatabase>, uuid: Uuid) -> (Status, Value) {
    let collection = db.database("techcamp").collection::<Text>("texts");
    match collection
        .find_one(doc! {"_id": uuid_to_bson(&uuid)}, None)
        .await
    {
        Err(e) => (
            Status::InternalServerError,
            json!({"error": format!("error searching database: {e}")}),
        ),
        Ok(result) => match result {
            None => (
                Status::NotFound,
                json!({"error": "text not found".to_owned()}),
            ),
            Some(text) => (Status::Ok, json!({"data": text.text.to_owned()})),
        },
    }
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
