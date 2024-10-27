CREATE TABLE groups
(
    id         UUID UNIQUE NOT NULL,
    name       VARCHAR     NOT NULL,
    owner_id   UUID        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    created_on TIMESTAMP   NOT NULL,
    PRIMARY KEY (id)
);
