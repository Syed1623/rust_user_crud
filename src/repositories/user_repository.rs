
use diesel::prelude::*;
use crate::db::Pool;
use crate::models::user::{NewUser, User};
use crate::schema::users;

pub struct UserRepository;

impl UserRepository {
    pub fn create_user(pool: &Pool, new_user: NewUser) -> QueryResult<User> {
        let mut conn = pool.get().expect("Couldn't get db connection from pool");
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&mut conn)
    }

    pub fn get_all_users(pool: &Pool) -> QueryResult<Vec<User>> {
        let mut conn = pool.get().expect("Couldn't get db connection from pool");
        users::table.load::<User>(&mut conn)
    }

    pub fn get_user_by_id(pool: &Pool, user_id: i32) -> QueryResult<Option<User>> {
        let mut conn = pool.get().expect("Couldn't get db connection from pool");
        users::table.find(user_id).get_result(&mut conn).optional()
    }

    pub fn update_user(pool: &Pool, user_id: i32, updated_user: NewUser) -> QueryResult<User> {
        let mut conn = pool.get().expect("Couldn't get db connection from pool");
        diesel::update(users::table.find(user_id))
            .set((
                users::email.eq(updated_user.email),
                users::name.eq(updated_user.name),
            ))
            .get_result(&mut conn)
    }

    pub fn delete_user(pool: &Pool, user_id: i32) -> QueryResult<usize> {
        let mut conn = pool.get().expect("Couldn't get db connection from pool");
        diesel::delete(users::table.find(user_id)).execute(&mut conn)
    }
}
        