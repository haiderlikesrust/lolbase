-- Add migration script here
CREATE TABLE rows(for_column uuid NOT NULL, value text NOT NULL, value_type text NOT NULL, row_id uuid NOT NULL);