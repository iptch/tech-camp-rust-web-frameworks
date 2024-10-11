use rocket::http::Status;
use rocket::serde::uuid::Uuid;
use rocket::serde::{
    json::{json, Json, Value},
    Deserialize, Serialize,
};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Message<'m> {
    pub data: &'m str,
}

#[post("/texts", format = "application/json", data = "<msg>")]
pub async fn post_text(msg: Json<Message<'_>>) -> (Status, Value) {
    let id = Uuid::new_v4();

    println!("got the message: {}", msg.data);
    // Payload up to 10Mib
    // Success: 201 with UUID of text id in database
    // 400 Bad Request for invalid payload
    // 500 Internal Server Error for server-side errors
    (Status::Created, json!(id))
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
