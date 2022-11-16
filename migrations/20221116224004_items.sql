CREATE TABLE IF NOT EXISTS items
(
    id          INTEGER PRIMARY KEY NOT NULL,
    name        VARCHAR(250)        NOT NULL,
    price       FLOAT               NOT NULL DEFAULT 0
);

