#[macro_use]
extern crate rocket;

mod routes;

use routes::*;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_text, post_text, delete_text, get_search])
}
