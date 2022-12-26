use std::sync::Arc;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

use crate::fianchetto::Fianchetto;

pub mod game_controller;
pub mod reservation_controller;
pub mod team_controller;
pub mod user_controller;

pub trait Controller {
    fn routes(app: &mut Fianchetto, conn_pool: Arc<Pool<ConnectionManager<PgConnection>>>);
}
