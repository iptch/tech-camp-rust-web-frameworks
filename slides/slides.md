---
title: TechCamp 2024/10 Rust Web Frameworks
author: Selim Kälin & Zak Cook & Jan Kleine
institute: Innovation Process Technology AG
date: 12.10.2024
theme: moon
revealjs-url: "https://unpkg.com/reveal.js@5.1.0"
progress: false
controls: false
hash: true
highlightjs: true
---

# Actix

## Ergonomics / Hands-On Feel

<pre data-id="code-animation"><code data-trim data-line-numbers="|6,27|25|8,20,21,26" rust><script type="text/template">
#[derive(Debug, Deserialize, Serialize)]
struct Query {
    term: String,
}

#[get("/texts/{uuid}/search")]
async fn my_endpoint(
    client: web::Data<Client>,
    uuid: web::Path<Uuid>,
    term: web::Query<Query>,
) -> impl Responder {

    // .. do something

    HttpResponse::Ok().json(response)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let uri = env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await.expect("failed to connect");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(client.clone()))
            .service(my_endpoint)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
</script></code></pre>

## Benchmark

# Axum

## Ergonomics / Hands-On Feel

<pre data-id="code-animation"><code data-trim data-line-numbers="|2-4|9|10" rust><script type="text/template">
    let app = Router::new()
        .route("/texts", post(post_text))
        .route("/texts/:text_id", get(get_text).delete(delete_text))
        .route("/texts/:text_id/search", get(search_text))
        .layer(TraceLayer::new_for_http())
        .with_state(shared_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
</script></code></pre>

## {data-auto-animate=true}

<pre data-id="code-animation"><code data-trim data-line-numbers="|2|3|4|19-22" rust><script type="text/template">
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
</script></code></pre>

## Pros/Cons

- + lightweight
- + no macro black magic
- - more verbose (e.g. for state)

## Benchmark

# Rocket

## Ergonomics {data-auto-animate=true}

<pre data-id="code-animation"><code data-trim data-line-numbers="|1,4" rust>
#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
}
</code></pre>

## {data-auto-animate=true}

<pre data-id="code-animation"><code data-trim data-line-numbers="|5-7,12" rust>
#[macro_use]
extern crate rocket;
use rocket_db_pools::{Database, mongodb};

#[derive(Database)]
#[database("my-database-name")]
pub struct MyDatabase(mongodb::Client);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(MyDatabase::init())
}
</code></pre>

## {data-auto-animate=true}

<pre data-id="code-animation"><code data-trim data-line-numbers="|12|12-24|30|2-6" rust><script type="text/template">
#[macro_use]
extern crate rocket;
use rocket_db_pools::{Database, Connection, mongodb};
use rocket::serde::uuid::Uuid;
use rocket::http::Status;
use rocket::serde::json::{json, Value};

#[derive(Database)]
#[database("my-database-name")]
pub struct MyDatabase(mongodb::Client);

#[get("/texts/<uuid>")]
pub async fn get(db: Connection<MyDatabase>, uuid: Uuid) -> (Status, Value) {
    match get_from_database(db, uuid).await {
        Err(e) => (
            Status::InternalServerError,
            json!({"error": format!("error searching database: {e}")}),
        ),
        Ok(result) => (
            Status::Ok,
            json!({"data": text.text.to_owned()}),
        ),
    },
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(MyDatabase::init())
        .mount("/", routes![get])
}
</script></code></pre>

## Benchmark

Will update this when available ...

## Immutability {data-auto-animate=true}

<pre data-id="code-animation"><code data-trim data-line-numbers="2,4" rust>
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
</code></pre>

::: notes

- typing is still respected, cannot change type of variable, even with `mut`
:::

## Everything is Owned

```{.rust data-line-numbers=""}
let x = String::from("hello");
let y = x;
println!("{}", x); // invalid
```
