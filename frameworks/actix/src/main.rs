use actix_web::{
    delete, get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder,
};
use log::info;
use mongodb::bson::doc;
use mongodb::Client;
use mongodb::{bson, results::DeleteResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const DB_NAME: &str = "SearchApp";
const COLL_NAME: &str = "texts";

#[derive(Debug, Deserialize, Serialize)]
struct MongoText {
    #[serde(rename = "_id")]
    id: Uuid,
    data: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct MongoID {
    #[serde(rename = "_id")]
    id: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
struct TextResponse {
    data: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct UUIDResponse {
    id: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
struct FoundResponse {
    found: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Query {
    term: String,
}

#[post("/texts")]
async fn save_text(client: web::Data<Client>, payload: web::Json<TextResponse>) -> impl Responder {
    let text = MongoText {
        id: Uuid::new_v4(),
        data: payload.data.to_owned(),
    };

    let collection = client.database(DB_NAME).collection::<MongoText>(COLL_NAME);

    match collection.insert_one(&text).await {
        Err(err) => {
            let response = ErrorResponse {
                error: format!("Failed to write to DB: {err}"),
            };
            HttpResponse::InternalServerError().json(response)
        }
        Ok(_) => HttpResponse::Created().json(UUIDResponse { id: text.id }),
    }
}

#[delete("/texts/{uuid}")]
async fn delete_text(client: web::Data<Client>, uuid: web::Path<Uuid>) -> impl Responder {
    let collection = client.database(DB_NAME).collection::<MongoText>(COLL_NAME);

    let delete_one = collection.delete_one(doc! { "_id": uuid_to_bson(&uuid) });
    match delete_one.await {
        Err(err) => {
            let response = ErrorResponse {
                error: format!("Failed to delete from DB: {err}"),
            };
            HttpResponse::InternalServerError().json(response)
        }
        Ok(DeleteResult {
            deleted_count: 0, ..
        }) => HttpResponse::NotFound().json(ErrorResponse {
            error: "UUID does not exist".to_owned(),
        }),
        Ok(_) => HttpResponse::NoContent().finish(),
    }
}

#[get("/texts/{uuid}")]
async fn get_text(client: web::Data<Client>, uuid: web::Path<Uuid>) -> impl Responder {
    let collection = client.database(DB_NAME).collection::<MongoText>(COLL_NAME);
    let find_one = collection.find_one(doc! { "_id": uuid_to_bson(&uuid)});
    match find_one.await {
        Err(err) => {
            let response = ErrorResponse {
                error: format!("Failed to search DB: {err}"),
            };
            HttpResponse::InternalServerError().json(response)
        }
        Ok(None) => {
            let response = ErrorResponse {
                error: "UUID does not exist".to_string(),
            };
            HttpResponse::NotFound().json(response)
        }
        Ok(Some(mongo_text)) => {
            let response = TextResponse {
                data: mongo_text.data,
            };
            HttpResponse::Ok().json(response)
        }
    }
}

#[get("/texts/{uuid}/search")]
async fn search_text(
    client: web::Data<Client>,
    uuid: web::Path<Uuid>,
    term: web::Query<Query>,
) -> impl Responder {
    let term = term.term.to_owned();
    if term.contains(char::is_whitespace) {
        let response = ErrorResponse {
            error: format!("term is not allowed to contain whitspaces: {term}"),
        };
        return HttpResponse::BadRequest().json(response);
    }

    let collection = client.database(DB_NAME).collection::<MongoText>(COLL_NAME);
    let find_one = collection.find_one(doc! { "_id": uuid_to_bson(&uuid)});
    match find_one.await {
        Err(err) => {
            let response = ErrorResponse {
                error: format!("Failed to search DB: {err}"),
            };
            HttpResponse::InternalServerError().json(response)
        }
        Ok(None) => {
            let response = ErrorResponse {
                error: "UUID does not exist".to_string(),
            };
            HttpResponse::NotFound().json(response)
        }
        Ok(Some(mongo_text)) => {
            let response = FoundResponse {
                found: mongo_text.data.contains(&term),
            };
            HttpResponse::Ok().json(response)
        }
    }
}

fn uuid_to_bson(uuid: &Uuid) -> bson::Bson {
    let options = bson::ser::SerializerOptions::builder()
        .human_readable(false)
        .build();
    bson::to_bson_with_options(&uuid, options).unwrap()
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    info!("connecting to mongodb: {uri}");

    let client = Client::with_uri_str(uri).await.expect("failed to connect");

    info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(client.clone()))
            .service(save_text)
            .service(delete_text)
            .service(get_text)
            .service(search_text)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
