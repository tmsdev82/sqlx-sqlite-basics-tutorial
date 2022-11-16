CREATE TABLE IF NOT EXISTS users
(
    id          INTEGER PRIMARY KEY NOT NULL,
    name        VARCHAR(250)        NOT NULL,
    active      BOOLEAN             NOT NULL DEFAULT 0
);

