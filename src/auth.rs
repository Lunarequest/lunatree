use super::db::LunaTreeDbConn;
use super::schema::users;

use super::users::*;
use super::BRANDING;
use diesel::prelude::*;
use rocket::fairing::AdHoc;
use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_dyn_templates::{context, Template};

#[get("/login")]
async fn login(messages: Option<FlashMessage<'_>>) -> Template {
	Template::render(
		"login",
		context! {
			 message: messages,
			 brand: BRANDING
		},
	)
}

#[post("/login", data = "<user>")]
async fn login_post(user: Form<LoginForm<'_>>) {

}

#[get("/register")]
async fn register(messages: Option<FlashMessage<'_>>) -> Template {
	Template::render(
		"register",
		context! {
			message: messages,
			brand: BRANDING
		},
	)
}

#[post("/register", data = "<user>")]
async fn register_post(
	conn: LunaTreeDbConn,
	user: Form<UserForm<'_>>,
) -> Result<Redirect, Flash<Redirect>> {
	if user.passwd1 == user.passwd2 {
		if user.username.chars().count() <= 50 {
			let dbuser = NewUser::new(
				user.username.to_string(),
				user.passwd1.to_string(),
				user.email.to_string(),
			);
			match dbuser {
				Ok(dbuser) => {
					match conn
						.run(move |c| diesel::insert_into(users::table).values(&dbuser).execute(c))
						.await
					{
						Ok(_) => {
							return Ok(Redirect::to(uri!("/")));
						}
						Err(e) => {
							return Err(Flash::error(Redirect::to(uri!(register)), format!("{e}")));
						}
					}
				}
				Err(e) => {
					return Err(Flash::error(
						Redirect::to(uri!(register)),
						format!("unkown error please report to admin tracedump: {}", e),
					));
				}
			};
		} else {
			return Err(Flash::error(
				Redirect::to(uri!(register)),
				"username too long",
			));
		}
	} else {
		return Err(Flash::error(
			Redirect::to(uri!(register)),
			"passwords do not match",
		));
	}
}

pub fn stage() -> AdHoc {
	AdHoc::on_ignite("User Authentication", |rocket| async {
		rocket.mount("/", routes![login, login_post, register, register_post])
	})
}
