-- Your SQL goes here
CREATE TABLE round1_questions (
                             stageno INTEGER UNIQUE NOT NULL UNIQUE,
    question TEXT NOT NULL,
                             answer TEXT NOT NULL,
    comment TEXT NOT NULL,
                             PRIMARY KEY (stageno)
);