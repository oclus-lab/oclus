CREATE TABLE users
(
    id                UUID UNIQUE    NOT NULL,
    email             VARCHAR UNIQUE NOT NULL,
    username          VARCHAR        NOT NULL,
    password          VARCHAR        NOT NULL,
    refresh_token     VARCHAR,
    registration_date TIMESTAMP      NOT NULL,
    PRIMARY KEY (id)
);


CREATE TABLE groups
(
    id       UUID UNIQUE NOT NULL,
    name     VARCHAR     NOT NULL,
    owner_id UUID        NOT NULL REFERENCES users (id),
    PRIMARY KEY (id)
);
