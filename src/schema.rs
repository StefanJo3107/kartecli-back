// @generated automatically by Diesel CLI.

diesel::table! {
    games (game_id) {
        game_id -> Int4,
        home_team_id -> Int4,
        guest_team_id -> Int4,
        date -> Varchar,
        basic_tickets -> Int4,
        vip_tickets -> Int4,
    }
}

diesel::table! {
    national_teams (national_team_id) {
        national_team_id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    reservations (reservation_id) {
        reservation_id -> Int4,
        user_id -> Int4,
        game_id -> Int4,
        basic_tickets -> Int4,
        vip_tickets -> Int4,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        username -> Varchar,
        password -> Varchar,
        name -> Varchar,
        surname -> Varchar,
        identity_number -> Varchar,
        email -> Varchar,
    }
}

diesel::joinable!(reservations -> games (game_id));
diesel::joinable!(reservations -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(games, national_teams, reservations, users,);
