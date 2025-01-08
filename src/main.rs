
#[macro_use]
extern crate rocket;

use dotenvy::dotenv;

mod db;
mod models;
mod repositories;
mod routes;
mod schema;

use crate::db::stage_pool;
use crate::routes::user_routes::{create_user, delete_user, get_user_by_id, get_users, update_user};

#[launch]
fn rocket() -> _ {
    // Load environment variables from .env
    dotenv().ok();

    rocket::build()
        .attach(stage_pool())
        .mount(
            "/api",
            routes![
                get_users,
                get_user_by_id,
                create_user,
                update_user,
                delete_user
            ],
        )
}
        