CREATE TABLE groups
(
    id         BIGSERIAL UNIQUE NOT NULL,
    name       VARCHAR          NOT NULL,
    owner_id   BIGSERIAL        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    created_on TIMESTAMP        NOT NULL,
    PRIMARY KEY (id)
);
