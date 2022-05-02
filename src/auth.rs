use super::users::*;
use super::schema::users;
use super::db::LunaTreeDbConn;
use super::BRANDING;
use diesel::prelude::*;
use rocket::form::Form;
use rocket::{fairing::AdHoc, response::Redirect};
use rocket_dyn_templates::{context, Template};

#[get("/login")]
async fn login() -> Template {
	Template::render(
		"login",
		context! {
			 brand: BRANDING
		},
	)
}

#[get("/register")]
async fn register() -> Template {
	Template::render(
		"register",
		context! {
			brand: BRANDING
		},
	)
}

#[post("/register", data = "<user>")]
async fn register_post(conn:LunaTreeDbConn, user: Form<UserForm<'_>>, ) {
	if user.passwd1 == user.passwd2 {
		if user.username.chars().count() <= 50 {
			let dbuser = NewUser::new(
				user.username.to_string(),
				user.passwd1.to_string(),
				user.email.to_string(),
			);
            match conn.run(move |c|  {
                diesel::insert_into(users::table)
                    .values(&dbuser)
                    .execute(c) 
            }
            ).await {
                Ok(_) => {},
                Err(e) => {}
            };
		}
	}
}

pub fn stage() -> AdHoc {
	AdHoc::on_ignite("User Authentication", |rocket| async {
		rocket.mount("/", routes![login, register, register_post])
	})
}
