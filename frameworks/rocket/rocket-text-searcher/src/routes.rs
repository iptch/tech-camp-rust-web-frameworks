use mongodb::bson::{doc, SerializerOptions};
use rocket::form::validate::Contains;
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
fn uuid_to_bson(uuid: &Uuid) -> mongodb::bson::Bson {
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
        Ok(_) => (Status::Created, json!({"id": id})),
        Err(e) => (
            Status::InternalServerError,
            json!({"error": format!("failed to insert text into database: {e}")}),
        ),
    }
}

#[delete("/texts/<uuid>")]
pub async fn delete_text(db: Connection<TextsDatabase>, uuid: Uuid) -> (Status, Value) {
    let collection = db.database("techcamp").collection::<Text>("texts");
    match collection
        .delete_one(doc! { "_id": uuid_to_bson(&uuid)}, None)
        .await
    {
        Err(e) => (
            Status::InternalServerError,
            json!({"error": format!("error deleting from database: {e}")}),
        ),
        Ok(result) => match result.deleted_count {
            0 => (Status::NotFound, json!({"error": "text not found"})),
            1 => (Status::NoContent, Value::default()),
            _ => (Status::Gone, Value::default()),
        },
    }
}

async fn get_from_database(
    db: Connection<TextsDatabase>,
    uuid: Uuid,
) -> mongodb::error::Result<Option<Text>> {
    let collection = db.database("techcamp").collection::<Text>("texts");
    collection
        .find_one(doc! { "_id": uuid_to_bson(&uuid)}, None)
        .await
}

#[get("/texts/<uuid>")]
pub async fn get_text(db: Connection<TextsDatabase>, uuid: Uuid) -> (Status, Value) {
    match get_from_database(db, uuid).await {
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
pub async fn get_search(db: Connection<TextsDatabase>, uuid: Uuid, term: &str) -> (Status, Value) {
    match get_from_database(db, uuid).await {
        Err(e) => (
            Status::InternalServerError,
            json!({"error": format!("error searching database: {e}")}),
        ),
        Ok(result) => match result {
            None => (
                Status::NotFound,
                json!({"error": "text not found".to_owned()}),
            ),
            Some(text) => {
                let found = text.text.contains(term);
                (Status::Ok, json!({"found": found}))
            }
        },
    }
}
