CREATE TABLE users
(
    id            UUID UNIQUE    NOT NULL,
    email         VARCHAR UNIQUE NOT NULL,
    username      VARCHAR        NOT NULL,
    password      VARCHAR        NOT NULL,
    refresh_token VARCHAR,
    registered_on TIMESTAMP      NOT NULL,
    PRIMARY KEY (id)
);
