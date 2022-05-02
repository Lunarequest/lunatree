#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;
use rocket_dyn_templates::Template;
mod auth;
mod db;
mod schema;
mod users;

const BRANDING: &'static str = "lunatree";

#[get("/")]
fn index() -> &'static str {
	"Hello, world!"
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.attach(Template::fairing())
		.attach(db::stage())
		.attach(auth::stage())
		.mount("/", routes![index])
}
