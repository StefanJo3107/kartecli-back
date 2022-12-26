use std::sync::Arc;

use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use diesel::r2d2::{ConnectionManager, Pool};
use serde_json::json;

use super::Controller;
use crate::fianchetto::response::Response;
use crate::fianchetto::Fianchetto;
use crate::models::NationalTeam;
use crate::schema::national_teams::dsl;

pub struct TeamController;

impl Controller for TeamController {
    fn routes(app: &mut Fianchetto, conn_pool: Arc<Pool<ConnectionManager<PgConnection>>>) {
        let conn = Arc::clone(&conn_pool);
        app.get("/teams", move |_, _| {
            let teams: Vec<NationalTeam>;
            match dsl::national_teams.load(&conn.get().unwrap()) {
                Ok(t) => teams = t,
                Err(err) => {
                    let err = err.to_string();
                    let err_json = json!({ "err": err });
                    return Ok(Response::not_found(serde_json::to_string(&err_json)?));
                }
            }

            let team_json = serde_json::to_string(&teams)?;
            Ok(Response::ok(team_json))
        });

        let conn = Arc::clone(&conn_pool);
        app.get("/teams/:id", move |_, params| {
            let team_id: i32 = params.find("id").unwrap().parse()?;
            let team: NationalTeam;
            match dsl::national_teams
                .filter(dsl::national_team_id.eq(team_id))
                .first(&conn.get().unwrap())
            {
                Ok(t) => team = t,
                Err(err) => {
                    let err = err.to_string();
                    let err_json = json!({ "err": err });
                    return Ok(Response::not_found(serde_json::to_string(&err_json)?));
                }
            }

            let team_json = serde_json::to_string(&team)?;
            Ok(Response::ok(team_json))
        });
    }
}

impl TeamController {}
