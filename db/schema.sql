-- live-bet database schema
-- Requires: PostgreSQL with pgvector extension

CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS properties (
    id         SERIAL PRIMARY KEY,
    name       TEXT,
    image_url  TEXT,
    region     TEXT,
    bedroom_no INTEGER,
    price      INTEGER,
    embedding  vector(128)
);

CREATE TABLE IF NOT EXISTS users (
    id              SERIAL PRIMARY KEY,
    first_name      TEXT,
    last_name       TEXT,
    email           TEXT UNIQUE,
    password_digest TEXT
);

-- Seed data
INSERT INTO properties (name, image_url, region, bedroom_no, price) VALUES
    ('Shiny house on the coast',
     'https://images.unsplash.com/photo-1551524164-687a55dd1126?auto=format&fit=crop&w=1025&q=80',
     'Melbourne', 3, 1000000),
    ('House on the sky',
     'https://plus.unsplash.com/premium_photo-1665657351423-1914cf8f4117?auto=format&fit=crop&w=500&q=60',
     'Sydney', 1, 600000),
    ('Opera house',
     'https://images.unsplash.com/photo-1596428025491-662bbaf50351?auto=format&fit=crop&w=500&q=60',
     'Sydney', 20, 4000000)
ON CONFLICT DO NOTHING;
