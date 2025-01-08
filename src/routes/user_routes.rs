
use rocket::{get, post, put, delete, http::Status, serde::json::Json, State};
use crate::repositories::user_repository::UserRepository;
use crate::models::user::{NewUser, User};
use crate::db::Pool;

#[get("/users")]
pub fn get_users(pool: &State<Pool>) -> Result<Json<Vec<User>>, Status> {
    UserRepository::get_all_users(pool.inner())
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/users/<id>")]
pub fn get_user_by_id(pool: &State<Pool>, id: i32) -> Result<Json<User>, Status> {
    UserRepository::get_user_by_id(pool.inner(), id)
        .map(Json)
        .map_err(|_| Status::NotFound)
}

#[post("/users", format = "json", data = "<new_user>")]
pub fn create_user(pool: &State<Pool>, new_user: Json<NewUser>) -> Result<Json<User>, Status> {
    UserRepository::create_user(pool.inner(), new_user.into_inner())
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[put("/users/<id>", format = "json", data = "<updated_user>")]
pub fn update_user(pool: &State<Pool>, id: i32, updated_user: Json<NewUser>) -> Result<Json<User>, Status> {
    UserRepository::update_user(pool.inner(), id, updated_user.into_inner())
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[delete("/users/<id>")]
pub fn delete_user(pool: &State<Pool>, id: i32) -> Result<Status, Status> {
    match UserRepository::delete_user(pool.inner(), id) {
        Ok(num_deleted) if num_deleted > 0 => Ok(Status::NoContent),
        Ok(_) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}
        