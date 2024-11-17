-- Add migration script here

CREATE TABLE tokens (
    id BIGSERIAL PRIMARY KEY,
    account_id CHAR(16) NOT NULL,
    token CHAR(50) NOT NULL,
    expires_at TIMESTAMP NOT NULL
);