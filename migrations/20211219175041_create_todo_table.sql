-- Add migration script here
CREATE TABLE todo(
    id uuid NOT NULL,
    task TEXT NOT NULL,
    complete BOOLEAN NOT NULL,
    PRIMARY KEY (id)
)