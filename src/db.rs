
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use rocket::fairing::AdHoc;
use rocket::figment::Figment;
use rocket::{Rocket, Build};
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url: &str) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub struct Db(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl Db {
    // Shortcut for retrieving a single connection
    pub fn get_one(pool: &Pool) -> Self {
        Db(pool.get().expect("Failed to get DB connection from pool"))
    }
}

// Attach the DB pool as a Rocket fairing
pub fn stage_pool() -> AdHoc {
    AdHoc::on_ignite("Database Pool", |rocket| async {
        let figment: &Figment = rocket.figment();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in .env or environment");

        let pool = init_pool(&database_url);
        rocket.manage(pool)
    })
}
        