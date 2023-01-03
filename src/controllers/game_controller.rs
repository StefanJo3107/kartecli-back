use std::sync::Arc;

use crate::models::{Game, UpdateGame};

use super::Controller;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;
use serde_json::json;

use crate::fianchetto::response::Response;
use crate::fianchetto::Fianchetto;
use crate::schema::games;
pub struct GameController;

impl Controller for GameController {
    fn routes(app: &mut Fianchetto, conn_pool: Arc<Pool<ConnectionManager<PgConnection>>>) {
        let conn = Arc::clone(&conn_pool);
        app.get("/games", move |_, _| {
            let games_vec: Vec<Game>;
            match games::table.load(&conn.get().unwrap()) {
                Ok(g) => games_vec = g,
                Err(err) => {
                    let err = err.to_string();
                    let err_json = json!({ "err": err });
                    return Ok(Response::not_found(serde_json::to_string(&err_json)?));
                }
            }

            let games_json = serde_json::to_string(&games_vec)?;
            Ok(Response::ok(games_json))
        });

        let conn = Arc::clone(&conn_pool);
        app.get("/games/:id", move |_, params| {
            let game_id: i32 = params.find("id").unwrap().parse()?;
            let game: Game;
            match games::table
                .filter(games::dsl::game_id.eq(game_id))
                .first(&conn.get().unwrap())
            {
                Ok(g) => game = g,
                Err(err) => {
                    let err = err.to_string();
                    let err_json = json!({ "err": err });
                    return Ok(Response::not_found(serde_json::to_string(&err_json)?));
                }
            }

            let game_json = serde_json::to_string(&game)?;
            Ok(Response::ok(game_json))
        });

        let conn = Arc::clone(&conn_pool);
        app.put("/games/:id", move |req, params| {
            let game_id: i32 = params.find("id").unwrap().parse()?;
            let upd_game: UpdateGame = serde_json::from_value(req.content)?;
            let game: Game = GameController::update_game(&conn.get().unwrap(), game_id, upd_game)?;

            let game_json = serde_json::to_string(&game)?;
            Ok(Response::ok(game_json))
        });
    }
}

impl GameController {
    pub fn update_game(conn: &PgConnection, id: i32, game: UpdateGame) -> Result<Game, Error> {
        diesel::update(games::table.find(id))
            .set((
                games::dsl::vip_tickets.eq(games::dsl::vip_tickets - game.vip_tickets),
                games::dsl::basic_tickets.eq(games::dsl::basic_tickets - game.basic_tickets),
            ))
            .get_result(conn)
    }
}
