use std::sync::Arc;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;
use serde_json::json;

use super::Controller;
use crate::fianchetto::response::Response;
use crate::fianchetto::Fianchetto;
use crate::models::{NewReservation, Reservation};
use crate::schema::reservations;

pub struct ReservationController;

impl Controller for ReservationController {
    fn routes(app: &mut Fianchetto, conn_pool: Arc<Pool<ConnectionManager<PgConnection>>>) {
        let conn = Arc::clone(&conn_pool);
        app.get("/reservations", move |_, _| {
            let reservation_vec: Vec<Reservation>;
            match reservations::table.load(&conn.get().unwrap()) {
                Ok(r) => reservation_vec = r,
                Err(err) => {
                    let err = err.to_string();
                    let err_json = json!({ "err": err });
                    return Ok(Response::not_found(serde_json::to_string(&err_json)?));
                }
            }

            let reservation_json = serde_json::to_string(&reservation_vec)?;
            Ok(Response::ok(reservation_json))
        });

        let conn = Arc::clone(&conn_pool);
        app.get("/reservations/:id", move |_, params| {
            let res_id: i32 = params.find("id").unwrap().parse()?;
            let reservation: Reservation = reservations::table
                .filter(reservations::dsl::reservation_id.eq(res_id))
                .first(&conn.get().unwrap())?;

            let reservation_json = serde_json::to_string(&reservation)?;
            Ok(Response::ok(reservation_json))
        });

        let conn = Arc::clone(&conn_pool);
        app.post("/reservation", move |req, _| {
            let new_res: NewReservation = serde_json::from_value(req.content)?;
            let reservation: Reservation =
                ReservationController::create_reservation(&conn.get().unwrap(), new_res)?;

            let reservation_json = serde_json::to_string(&reservation)?;
            Ok(Response::created(reservation_json))
        });

        let conn = Arc::clone(&conn_pool);
        app.delete("/reservation/:id", move |_, params| {
            let reservation_id: i32 = params.find("id").unwrap().parse()?;
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
