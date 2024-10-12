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

# Axum

## Ergonomics / Hands-On Feel

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


