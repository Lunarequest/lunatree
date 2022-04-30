use super::users::*;
use rocket::fairing::AdHoc;
use rocket_dyn_templates::{context, Template};

#[get("/login")]
async fn login() -> Template {
	Template::render(
		"login",
		context! {
			 hello: "World"
		},
	)
}

pub fn stage() -> AdHoc {
	AdHoc::on_ignite("User Authentication", |rocket| async {
		rocket.mount("/", routes![login])
	})
}
