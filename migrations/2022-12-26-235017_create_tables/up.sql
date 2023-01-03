CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    surname VARCHAR NOT NULL,
    identity_number VARCHAR NOT NULL,
    email VARCHAR NOT NULL
);

CREATE TABLE national_teams(
    national_team_id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE games(
    game_id SERIAL PRIMARY KEY,
    home_team_id INT NOT NULL,
    guest_team_id INT NOT NULL,
    basic_tickets INT NOT NULL,
    vip_tickets INT NOT NULL,
    date VARCHAR NOT NULL,
    CONSTRAINT fk_home
    FOREIGN KEY(home_team_id)
    REFERENCES national_teams(national_team_id)
    ON DELETE CASCADE,
    CONSTRAINT fk_guest
    FOREIGN KEY(guest_team_id)
    REFERENCES national_teams(national_team_id)
    ON DELETE CASCADE
);

CREATE TABLE reservations(
    reservation_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    game_id INT NOT NULL,
    basic_tickets INT NOT NULL,
    vip_tickets INT NOT NULL,
    CONSTRAINT fk_user
    FOREIGN KEY(user_id)
    REFERENCES users(user_id)
    ON DELETE CASCADE,
    CONSTRAINT fk_game
    FOREIGN KEY(game_id)
    REFERENCES games(game_id) 
    ON DELETE CASCADE
);