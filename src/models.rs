use super::schema::{games, national_teams, reservations, users};
use diesel::{Identifiable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize)]
#[primary_key(user_id)]
#[table_name = "users"]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub name: String,
    pub surname: String,
    pub identity_number: String,
    pub email: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub name: String,
    pub surname: String,
    pub identity_number: String,
    pub email: String,
}

#[derive(Identifiable, Queryable, Serialize)]
#[primary_key(national_team_id)]
#[table_name = "national_teams"]
pub struct NationalTeam {
    pub national_team_id: i32,
    pub name: String,
}

#[derive(Identifiable, Queryable, Serialize)]
#[primary_key(national_team_id)]
#[table_name = "national_teams"]
pub struct HomeTeam {
    pub national_team_id: i32,
    pub name: String,
}

#[derive(Identifiable, Queryable, Serialize)]
#[primary_key(national_team_id)]
#[table_name = "national_teams"]
pub struct GuestTeam {
    pub national_team_id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "national_teams"]
pub struct NewNationalTeam {
    pub name: String,
}

#[derive(Identifiable, Queryable, Serialize, Associations)]
#[primary_key(game_id)]
#[belongs_to(HomeTeam, foreign_key = "home_team_id")]
#[belongs_to(GuestTeam, foreign_key = "guest_team_id")]
#[table_name = "games"]
pub struct Game {
    pub game_id: i32,
    pub home_team_id: i32,
    pub guest_team_id: i32,
    pub date: String,
    pub basic_tickets: i32,
    pub vip_tickets: i32,
}

#[derive(Insertable, Deserialize)]
#[table_name = "games"]
pub struct UpdateGame {
    pub basic_tickets: i32,
    pub vip_tickets: i32,
}

#[derive(Identifiable, Queryable, Serialize, Associations)]
#[primary_key(reservation_id)]
#[belongs_to(User, foreign_key = "user_id")]
#[belongs_to(Game, foreign_key = "game_id")]
#[table_name = "reservations"]
pub struct Reservation {
    pub reservation_id: i32,
    pub user_id: i32,
    pub game_id: i32,
    pub basic_tickets: i32,
    pub vip_tickets: i32,
}

#[derive(Insertable, Deserialize)]
#[table_name = "reservations"]
pub struct NewReservation {
    pub user_id: i32,
    pub game_id: i32,
    pub basic_tickets: i32,
    pub vip_tickets: i32,
}
