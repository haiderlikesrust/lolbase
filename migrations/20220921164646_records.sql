-- Add migration script here
CREATE TABLE records(name text NOT NULL, record_id uuid NOT NULL PRIMARY KEY, created_on timestamp NOT NULL);