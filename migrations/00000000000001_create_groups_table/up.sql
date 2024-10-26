CREATE TABLE groups
(
    id       UUID UNIQUE NOT NULL,
    name     VARCHAR     NOT NULL,
    owner_id UUID        NOT NULL REFERENCES users (id),
    PRIMARY KEY (id)
);
