#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;
mod db;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
