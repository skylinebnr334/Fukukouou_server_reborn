-- Your SQL goes here

CREATE TABLE round2_info (
                             id INTEGER UNIQUE NOT NULL UNIQUE,
                             current_num INTEGER NOT NULL DEFAULT 0,
                             PRIMARY KEY (id)
);
CREATE TABLE round2_data (
                             team_id INTEGER NOT NULL UNIQUE,
                             current_phase INTEGER NOT NULL DEFAULT 0,
                             latest_down_num INTEGER NOT NULL DEFAULT -1,
                             miss_timing INTEGER NOT NULL DEFAULT -1,
                             PRIMARY KEY (team_id)
);