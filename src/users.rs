use super::schema::users;
use diesel::{Insertable, Queryable};

#[derive(Queryable)]
pub struct User {
	pub userid: i32,
	pub username: String,
	pub password: String,
	pub email: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
	pub username: String,
	pub password: String,
	pub email: String,
}
