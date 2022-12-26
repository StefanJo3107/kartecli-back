use micro_backend_framework::controllers::game_controller::GameController;
use micro_backend_framework::controllers::reservation_controller::ReservationController;
use micro_backend_framework::controllers::team_controller::TeamController;
use micro_backend_framework::controllers::user_controller::UserController;
use micro_backend_framework::controllers::Controller;
use micro_backend_framework::establish_connection;
use micro_backend_framework::fianchetto::Fianchetto;
use std::sync::Arc;

fn main() {
    let mut app: Fianchetto = Fianchetto::new("127.0.0.1:1207", 4);
    let db_conn = Arc::new(establish_connection());

    ReservationController::routes(&mut app, Arc::clone(&db_conn));
    GameController::routes(&mut app, Arc::clone(&db_conn));
    UserController::routes(&mut app, Arc::clone(&db_conn));
    TeamController::routes(&mut app, Arc::clone(&db_conn));

    app.listen();
}
