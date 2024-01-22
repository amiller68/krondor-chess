-- This started of as a Todo list template
-- we drop this table in the next migration
CREATE TABLE IF NOT EXISTS todos (
    id SERIAL PRIMARY KEY,
    description TEXT NOT NULL
);
