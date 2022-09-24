-- Add migration script here
CREATE TABLE columns(name text NOT NULL, column_id uuid NOT NULL PRIMARY KEY, created_on timestamp NOT NULL, for_record uuid NOT NULL)