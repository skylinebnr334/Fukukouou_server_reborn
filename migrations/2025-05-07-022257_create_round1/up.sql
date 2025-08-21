-- Your SQL goes here
CREATE TABLE round1_info (
    id INTEGER UNIQUE NOT NULL UNIQUE,
    current_stage INTEGER NOT NULL DEFAULT 0,
    current_question INTEGER NOT NULL DEFAULT -13,
    PRIMARY KEY (id)
);
CREATE TABLE round1_data (
    id INTEGER NOT NULL UNIQUE,
    team1 INTEGER NOT NULL DEFAULT 0,
    team2 INTEGER NOT NULL DEFAULT 0,
    team3 INTEGER NOT NULL DEFAULT 0,
    team4 INTEGER NOT NULL DEFAULT 0,
    team5 INTEGER NOT NULL DEFAULT 0,
    team6 INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (id)
);
CREATE TABLE round1_tokutendt
(
    id INTEGER NOT NULL DEFAULT 0 UNIQUE,
    correct INTEGER NOT NULL DEFAULT 1,
    miss      INTEGER NOT NULL DEFAULT -1,
    ask_throw INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (id)
);
CREATE TABLE round1_used_question (
                             id INTEGER UNIQUE NOT NULL UNIQUE,
    PRIMARY KEY (id)
                                  );