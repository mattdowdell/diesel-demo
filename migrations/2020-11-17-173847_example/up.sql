-- Your SQL goes here

CREATE TABLE users (
    id UUID PRIMARY KEY,
    name VARCHAR(254) NOT NULL
);

CREATE TABLE objects (
    id UUID PRIMARY KEY,
    created_by UUID NOT NULL,
    deleted_by UUID,

    CONSTRAINT fk_created_by FOREIGN KEY (created_by) REFERENCES users(id),
    CONSTRAINT fk_deleted_by FOREIGN KEY (deleted_by) REFERENCES users(id)
);
