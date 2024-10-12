mod routes;

#[macro_use]
extern crate rocket;

use rocket_db_pools::Database;
use routes::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(TextsDatabase::init())
        .mount("/", routes![get_text, post_text, delete_text, get_search])
}
