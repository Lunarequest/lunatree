use diesel_migrations::embed_migrations;
use rocket::{Build, Rocket, fairing::AdHoc};
use rocket_sync_db_pools::{database, diesel::PgConnection};

#[database("lunatreedb")]
pub struct LunaTreeDbConn(PgConnection);

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    // This macro from `diesel_migrations` defines an `embedded_migrations`
    // module containing a function named `run` that runs the migrations in the
    // specified directory, initializing the database.
    embed_migrations!("./migrations");

    let conn = LunaTreeDbConn::get_one(&rocket)
        .await
        .expect("database connection");
    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("diesel migrations");

    rocket
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("setup database", |rocket| async {
        rocket.attach(LunaTreeDbConn::fairing())
              .attach(AdHoc::on_ignite("run migrations on db", run_migrations))
     }
  )
}
