CREATE TABLE users
(
    id            BIGSERIAL NOT NULL UNIQUE,
    email         VARCHAR   NOT NULL UNIQUE,
    username      VARCHAR   NOT NULL,
    registered_on TIMESTAMP NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE auth_infos
(
    user_id       BIGSERIAL NOT NULL REFERENCES users (id),
    user_email    VARCHAR   NOT NULL REFERENCES users (email),
    password_hash VARCHAR   NOT NULL,
    refresh_token VARCHAR,
    last_login_on TIMESTAMP,
    PRIMARY KEY (user_id)
);

-- pending registration with unverified email
CREATE TABLE registration_requests
(
    id      BIGSERIAL     NOT NULL UNIQUE,
    email   VARCHAR       NOT NULL,
    totp    VARCHAR       NOT NULL,
    trials  INT DEFAULT 0 NOT NULL,
    sent_on TIMESTAMP     NOT NULL,
    PRIMARY KEY (id)
);
