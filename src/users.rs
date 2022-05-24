use super::schema::users;
use argon2::{
	password_hash::{
		rand_core::OsRng, Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
	},
	Argon2,
};
use diesel::{Insertable, Queryable};

#[derive(Queryable)]
pub struct User {
	pub userid: i32,
	pub username: String,
	pub password: String,
	pub email: String,
	pub active: bool,
}

#[derive(Queryable, Insertable)]
#[table_name = "users"]
pub struct NewUser {
	pub username: String,
	pub password: String,
	pub email: String,
}

#[derive(FromForm, Debug, Clone)]
pub struct UserForm<'r> {
	pub username: &'r str,
	pub email: &'r str,
	pub passwd1: &'r str,
	pub passwd2: &'r str,
}

impl NewUser {
	pub fn new(username: String, password: String, email: String) -> Result<Self, Error> {
		let argon2 = Argon2::default();
		let salt = SaltString::generate(&mut OsRng);
		let password_hash = match argon2.hash_password(password.as_bytes(), &salt) {
			Ok(password) => password.to_string(),
			Err(e) => return Err(e),
		};
		Ok(NewUser {
			username,
			password: password_hash,
			email,
		})
	}
	pub async fn verify_password(self, password: String) -> bool {
		let parsed_hash = PasswordHash::new(&self.password);
		match parsed_hash {
			Ok(hash) => {
				return Argon2::default()
					.verify_password(password.as_bytes(), &hash)
					.is_ok();
			}
			Err(_) => false,
		}
	}
}
