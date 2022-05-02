use super::schema::users;
use argon2::Config;
use diesel::{Insertable, Queryable};
use rand::Rng;

#[derive(Queryable)]
pub struct User {
	pub userid: i32,
	pub username: String,
	pub password: String,
	pub email: String,
	pub active: bool,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
	pub username: String,
	pub password: String,
	pub email: String,
}

#[derive(FromForm, Debug)]
pub struct UserForm<'r> {
	pub username: &'r str,
	pub email: &'r str,
	pub passwd1: &'r str,
	pub passwd2: &'r str,
}

impl NewUser {
	pub fn new(username: String, password: String, email: String) -> Self {
		let salt = rand::thread_rng().gen::<u8>();
		let config = Config::default();
		let hashed_password = argon2::hash_encoded(password.as_bytes(), &[salt], &config)
			.expect("failed to hash password");
		NewUser {
			username: username,
			password: hashed_password,
			email: email,
		}
	}
}
