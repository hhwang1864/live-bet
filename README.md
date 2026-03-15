# live-bet

A real estate property listing and live auction web app built in Rust.

Users can browse properties, filter by bedroom count, place bids, and manage listings after signing up.

## Tech Stack

- **[Axum](https://github.com/tokio-rs/axum)** — async web framework
- **[SQLx](https://github.com/launchbay/sqlx)** — async PostgreSQL queries
- **[Tera](https://keats.github.io/tera/)** — HTML templating
- **[bcrypt](https://docs.rs/bcrypt)** — password hashing
- **[axum-extra](https://docs.rs/axum-extra)** — signed cookie sessions
- **Bootstrap 5** — frontend styling

## Features

- User sign up / log in / log out
- Browse all property listings
- Filter properties by bedroom count (1, 2, 3+)
- Add, edit, and delete properties (authenticated users only)
- Live auction — place bids on any property

## Database Schema

```sql
CREATE TABLE properties (
  id         SERIAL PRIMARY KEY,
  name       TEXT,
  image_url  TEXT,
  region     TEXT,
  bedroom_no INTEGER,
  price      INTEGER
);

CREATE TABLE users (
  id              SERIAL PRIMARY KEY,
  first_name      TEXT,
  last_name       TEXT,
  email           TEXT,
  password_digest TEXT
);
```

## Getting Started

### Prerequisites

- Rust (stable) — [rustup.rs](https://rustup.rs)
- PostgreSQL running locally

### Setup

```bash
# 1. Create the database and tables
psql -c "CREATE DATABASE real_estate_db;"

# 2. Copy and configure environment variables
cp .env.example .env
# Edit .env — set DATABASE_URL and SECRET_KEY

# 3. Run
cargo run
```

Server starts on **http://localhost:8080** by default.

## Project Structure

```
├── Cargo.toml
├── .env.example
├── src/
│   ├── main.rs              — app setup, router, state
│   ├── db.rs                — database connection pool
│   ├── models/
│   │   ├── property.rs      — property DB queries
│   │   └── user.rs          — user DB queries + bcrypt
│   └── handlers/
│       ├── properties.rs    — property route handlers
│       ├── sessions.rs      — login / logout
│       └── users.rs         — sign up
├── templates/               — Tera HTML templates (Bootstrap 5)
│   ├── layout.html
│   ├── properties/
│   ├── sessions/
│   └── users/
└── public/
    └── stylesheets/
```

## Environment Variables

| Variable | Description |
|---|---|
| `DATABASE_URL` | PostgreSQL connection string e.g. `postgres://localhost/real_estate_db` |
| `SECRET_KEY` | Secret for signing session cookies (min 64 bytes) |
| `PORT` | Port to listen on (default: `8080`) |
