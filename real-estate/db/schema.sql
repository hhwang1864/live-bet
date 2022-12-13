CREATE DATABASE real_estate_db;
\c real_estate_db

CREATE TABLE properties(
  id SERIAL PRIMARY KEY,
  name TEXT,
  image_url TEXT,
  region TEXT,
  bedroom_no INTEGER,
  price INTEGER
);

INSERT INTO properties(name,image_url, region, bedroom_no, price)
VALUES
('Shiny house on the coast', 'https://images.unsplash.com/photo-1551524164-687a55dd1126?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1025&q=80', 'Melbourne', '3', '1000000'),
('House on the sky',
'https://plus.unsplash.com/premium_photo-1665657351423-1914cf8f4117?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxzZWFyY2h8MXx8aG91c2UlMjBvbiUyMHRoZSUyMGhpbGx8ZW58MHx8MHx8&auto=format&fit=crop&w=500&q=60', 'Sydney', '1', '600000'),
('opera house','https://images.unsplash.com/photo-1596428025491-662bbaf50351?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxzZWFyY2h8MTV8fG9wZXJhJTIwaG91c2V8ZW58MHx8MHx8&auto=format&fit=crop&w=500&q=60','Sydney', '20', '4000000');

CREATE TABLE users(
  id SERIAL PRIMARY KEY,
  first_name TEXT,
  last_name TEXT,
  email TEXT
);

ALTER TABLE users ADD COLUMN password_digest TEXT;

