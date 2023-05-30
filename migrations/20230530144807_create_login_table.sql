-- Add migration script here
CREATE TABLE logins (
   id uuid NOT NULL,
   PRIMARY KEY (id),
   email TEXT NOT NULL UNIQUE,
   name TEXT NOT NULL,
   loggedin_at timestamptz NOT NULL
);
