-- Your SQL goes here
CREATE TABLE round1_questions (
                             stageno INTEGER UNIQUE NOT NULL UNIQUE,
    question TEXT,
                             answer TEXT,
    comment TEXT,
                             PRIMARY KEY (stageno)
);