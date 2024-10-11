#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use once_cell::sync::Lazy;
use rocket::http::Status;
use rocket::serde::uuid::Uuid;
use rocket::serde::json::{Json, json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::Request;
use rocket_db_pools::{deadpool_redis, mongodb, Connection, Database};
use redis::AsyncCommands;
use mongodb::bson;
use mongodb::bson::doc;
use rocket_prometheus::{
    prometheus::{opts, IntCounterVec},
    PrometheusMetrics,
};

const EXPIRE: usize = 7200;

#[derive(Database)]
#[database("redis")]
struct Cache(deadpool_redis::Pool);

#[derive(Database)]
#[database("mongo")]
struct Store(mongodb::Client);

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message<'r> {
    data: &'r str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Text {
    _id: Uuid,
    data: String,
}

static TERM_COUNTER: Lazy<IntCounterVec> = Lazy::new(|| {
    IntCounterVec::new(opts!("search_term_counter", "Count of times a term was searched for"), &["term"])
        .expect("Could not create lazy IntCounterVec")
});
static CACHE_COUNTER: Lazy<IntCounterVec> = Lazy::new(|| {
    IntCounterVec::new(opts!("cache_counter", "Count hits and misses on cache"), &["type"])
        .expect("Could not create lazy IntCounterVec")
});

#[post("/texts", format = "json", data = "<msg>")]
async fn store_text(mongo: Connection<Store>, mut cache: Connection<Cache>, msg: Json<Message<'_>>) -> (Status, Value) {
    let collection = mongo.database("erfa").collection::<Text>("texts");
    let id = Uuid::new_v4();
    let text = Text { _id: id, data: msg.data.to_owned() };
    let _: redis::RedisResult<String> = cache.set_ex(id.to_string(), msg.data, EXPIRE).await;
    match collection.insert_one(text, None).await {
        Err(error) => (Status::InternalServerError, json!({
            "error": format!("failed to write to DB: {}", error)
        })),
        Ok(_) => (Status::Created, json!({
            "id": id.as_hyphenated().to_string()
        })),
    }
}

#[get("/texts/<uuid>")]
async fn get_text(mongo: Connection<Store>, cache: Connection<Cache>, uuid: Uuid) -> (Status, Value) {
    let (status, val) = get_val(mongo, cache, uuid).await;
    match status.code {
        200 => (status, json!({ "data": val })),
        _ => (status, json!({ "error": val })),
    }
}

#[get("/texts/<uuid>/search?<term>")]
async fn search_text(mongo: Connection<Store>, cache: Connection<Cache>, uuid: Uuid, term: &str) -> (Status, Value) {
    TERM_COUNTER.with_label_values(&[term]).inc();
    let (status, val) = get_val(mongo, cache, uuid).await;
    match status.code {
        200 => (status, json!({ "found": val.split_whitespace().any(|x| x == term) })),
        _ => (status, json!({ "error": val })),
    }
}

async fn get_val(mongo: Connection<Store>, mut cache: Connection<Cache>, uuid: Uuid) -> (Status, String) {
    if let Ok(data) = cache.get::<String, String>(uuid.to_string()).await {
        CACHE_COUNTER.with_label_values(&["hit"]).inc();
        return (Status::Ok, data);
    }
    CACHE_COUNTER.with_label_values(&["miss"]).inc();
    let collection = mongo.database("erfa").collection::<Text>("texts");
    match collection.find_one(doc! { "_id": uuid_to_bson(&uuid) }, None).await {
        Err(error) => (Status::InternalServerError, format!("failed to get DB: {}", error)),
        Ok(Some(res)) => {
            let _: redis::RedisResult<String> = cache.set_ex(uuid.to_string(), &res.data, EXPIRE).await;
            (Status::Ok, res.data)
        },
        Ok(None) => (Status::NotFound, "text not found".to_owned()),
    }
}

#[delete("/texts/<uuid>")]
async fn delete_text(mongo: Connection<Store>, mut cache: Connection<Cache>, uuid: Uuid) -> (Status, Value) {
    let _ = cache.del::<String, String>(uuid.to_string()).await;
    let collection = mongo.database("erfa").collection::<Text>("texts");
    match collection.delete_one(doc! { "_id": uuid_to_bson(&uuid) }, None).await {
        Err(error) => (Status::InternalServerError, json!({
            "error": format!("failed to delete from DB: {}", error)
        })),
        Ok(res) => {
            if res.deleted_count > 0 {
                (Status::NoContent, Value::default())
            } else {
                (Status::Gone, Value::default())
            }
        }
    }
}

fn uuid_to_bson(uuid: &Uuid) -> bson::Bson {
    let options = bson::ser::SerializerOptions::builder().human_readable(false).build();
    bson::to_bson_with_options(&uuid, options).unwrap()
}

#[catch(500)]
fn internal_error() -> Value {
    json!({
        "error": "internal error"
    })
}

#[catch(404)]
fn not_found(req: &Request) -> Value {
    json!({
        "error": format!(
            "{} {} is not a valid operation",
            req.method(),
            req.uri().path()
        )
    })
}

#[launch]
fn rocket() -> _ {
    let prometheus = PrometheusMetrics::new();
    prometheus
        .registry()
        .register(Box::new(TERM_COUNTER.clone()))
        .unwrap();
    prometheus
        .registry()
        .register(Box::new(CACHE_COUNTER.clone()))
        .unwrap();
    rocket::build()
        .attach(prometheus.clone())
        .attach(Cache::init())
        .attach(Store::init())
        .register("/", catchers![internal_error, not_found])
        .mount("/", routes![store_text, delete_text, get_text, search_text])
        .mount("/metrics", prometheus)
}
