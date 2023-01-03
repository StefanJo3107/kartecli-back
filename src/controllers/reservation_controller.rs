use std::sync::Arc;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;
use pwhash::bcrypt;
use serde_json::json;

use super::game_controller::GameController;
use super::Controller;
use crate::fianchetto::response::Response;
use crate::fianchetto::Fianchetto;
use crate::models::{NewReservation, Reservation, UpdateGame, User};
use crate::schema::{reservations, users};

pub struct ReservationController;

impl Controller for ReservationController {
    fn routes(app: &mut Fianchetto, conn_pool: Arc<Pool<ConnectionManager<PgConnection>>>) {
        let conn = Arc::clone(&conn_pool);
        app.get("/reservations/:username/:password", move |_, params| {
            let username: String = params.find("username").unwrap().parse()?;
            let password: String = params.find("password").unwrap().parse()?;
            let user: User;
            match users::table
                .filter(users::dsl::username.eq(username))
                .first::<User>(&conn.get().unwrap())
            {
                Ok(res) => {
                    if bcrypt::verify(password, &res.password) {
                        user = res;
                    } else {
                        return Ok(Response::not_found(String::from(
                            "{\"err\":\"Unauthorized\"}",
                        )));
                    }
                }
                Err(err) => {
                    let err = err.to_string();
                    let err_json = json!({ "err": err });
                    return Ok(Response::not_found(serde_json::to_string(&err_json)?));
                }
            };

            let reservation: Vec<Reservation> = reservations::table
                .filter(reservations::dsl::user_id.eq(user.user_id))
                .load(&conn.get().unwrap())?;
            let reservation_json = serde_json::to_string(&reservation)?;
            Ok(Response::ok(reservation_json))
        });

        let conn = Arc::clone(&conn_pool);
        app.post("/reservations/:username/:password", move |req, params| {
            let username: String = params.find("username").unwrap().parse()?;
            let password: String = params.find("password").unwrap().parse()?;

            match users::table
                .filter(users::dsl::username.eq(username))
                .first::<User>(&conn.get().unwrap())
            {
                Ok(res) => {
                    if !bcrypt::verify(password, &res.password) {
                        return Ok(Response::not_found(String::from(
                            "{\"err\":\"Unauthorized\"}",
                        )));
                    }
                }
                Err(err) => {
                    let err = err.to_string();
                    let err_json = json!({ "err": err });
                    return Ok(Response::not_found(serde_json::to_string(&err_json)?));
                }
            };

            let new_res: NewReservation = serde_json::from_value(req.content)?;
            let reservation: Reservation =
                ReservationController::create_reservation(&conn.get().unwrap(), new_res)?;
            GameController::update_game(
                &conn.get().unwrap(),
                reservation.game_id,
                UpdateGame {
                    basic_tickets: reservation.basic_tickets,
                    vip_tickets: reservation.vip_tickets,
                },
            )?;
            let reservation_json = serde_json::to_string(&reservation)?;
            Ok(Response::created(reservation_json))
        });

        let conn = Arc::clone(&conn_pool);
        app.delete("/reservations/:username/:password/:id", move |_, params| {
            let username: String = params.find("username").unwrap().parse()?;
            let password: String = params.find("password").unwrap().parse()?;

            match users::table
                .filter(users::dsl::username.eq(username))
                .first::<User>(&conn.get().unwrap())
            {
                Ok(res) => {
                    if !bcrypt::verify(password, &res.password) {
                        return Ok(Response::not_found(String::from(
                            "{\"err\":\"Unauthorized\"}",
                        )));
                    }
                }
                Err(err) => {
                    let err = err.to_string();
                    let err_json = json!({ "err": err });
                    return Ok(Response::not_found(serde_json::to_string(&err_json)?));
                }
            };

            let reservation_id: i32 = params.find("id").unwrap().parse()?;
            let reservation: Reservation = reservations::table
                .filter(reservations::dsl::reservation_id.eq(reservation_id))
                .first(&conn.get().unwrap())?;
            GameController::update_game(
                &conn.get().unwrap(),
                reservation.game_id,
                UpdateGame {
                    basic_tickets: -reservation.basic_tickets,
                    vip_tickets: -reservation.vip_tickets,
                },
            )?;
            match ReservationController::delete_reservation(&conn.get().unwrap(), reservation_id) {
                Ok(()) => Ok(Response::ok(String::from(""))),
                Err(err) => {
                    return Ok(Response::bad_request_body(serde_json::to_string(
                        &json!({ "err": err.to_string() }),
                    )?))
                }
            }
        });
    }
}

impl ReservationController {
    fn create_reservation(
        conn: &PgConnection,
        reservation: NewReservation,
    ) -> Result<Reservation, Error> {
        diesel::insert_into(reservations::table)
            .values(&reservation)
            .get_result(conn)
    }

    fn delete_reservation(conn: &PgConnection, id: i32) -> Result<(), Error> {
        diesel::delete(reservations::table.filter(reservations::dsl::reservation_id.eq(id)))
            .execute(conn)?;
        Ok(())
    }
}
