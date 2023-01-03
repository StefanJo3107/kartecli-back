use std::sync::Arc;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;
use pwhash::bcrypt;
use serde_json::json;

use super::super::fianchetto::response::Response;
use super::super::fianchetto::Fianchetto;
use super::super::schema::users;
use super::Controller;
use crate::models::NewUser;
use crate::models::User;
pub struct UserController;

impl Controller for UserController {
    fn routes(app: &mut Fianchetto, conn_pool: Arc<Pool<ConnectionManager<PgConnection>>>) {
        let conn = Arc::clone(&conn_pool);
        app.get("/user/:username/:password", move |_, params| {
            let username: String = params.find("username").unwrap().parse()?;
            let password: String = params.find("password").unwrap().parse()?;
            let result: User;
            match users::table
                .filter(users::dsl::username.eq(username))
                .first::<User>(&conn.get().unwrap())
            {
                Ok(res) => {
                    if bcrypt::verify(password, &res.password) {
                        result = res;
                    } else {
                        return Ok(Response::not_found(String::from("{\"err\":\"NotFound\"}")));
                    }
                }
                Err(err) => {
                    let err = err.to_string();
                    let err_json = json!({ "err": err });
                    return Ok(Response::not_found(serde_json::to_string(&err_json)?));
                }
            };

            let user_json = serde_json::to_string(&result)?;
            Ok(Response::ok(user_json))
        });

        let conn = Arc::clone(&conn_pool);
        app.get("/check/:username", move |_, params| {
            let username: String = params.find("username").unwrap().parse()?;

            match users::table
                .filter(users::dsl::username.eq(username))
                .first::<User>(&conn.get().unwrap())
            {
                Ok(_) => return Ok(Response::ok(String::from("{\"status\": true}"))),
                Err(_) => return Ok(Response::ok(String::from("{\"status\": false}"))),
            }
        });

        let conn = Arc::clone(&conn_pool);
        app.post("/user", move |req, _| {
            let mut new_user: NewUser = serde_json::from_value(req.content)?;
            let hashed = bcrypt::hash(new_user.password)?;
            new_user.password = hashed;
            let user: User = UserController::create_user(&conn.get().unwrap(), new_user)?;

            let user_json = serde_json::to_string(&user)?;
            Ok(Response::created(user_json))
        });
    }
}

impl UserController {
    fn create_user(conn: &PgConnection, user: NewUser) -> Result<User, Error> {
        diesel::insert_into(users::table)
            .values(&user)
            .get_result(conn)
    }
}
