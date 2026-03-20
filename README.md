# live-bet

A real estate property listing and live auction platform built in **Rust**, featuring **vector database search** powered by pgvector.

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white)
![Bootstrap](https://img.shields.io/badge/Bootstrap-563D7C?style=for-the-badge&logo=bootstrap&logoColor=white)

---

## What It Does

live-bet is a full-stack web application where users can **browse**, **search**, **list**, and **bid** on properties in real time.

### Browse & Filter

> Visitors can explore all listed properties and filter by bedroom count (1 bed, 2 bed, 3+ bed). Each listing shows the property image, region, bedroom count, and price.

![Browse Properties](https://images.unsplash.com/photo-1560518883-ce09059eeffa?auto=format&fit=crop&w=900&q=80)

### Vector Search (pgvector)

> Type natural language queries like **"luxury mansion Sydney"** or **"budget studio near the coast"** and the app finds the most semantically similar properties — not just keyword matches. Powered by PostgreSQL's pgvector extension.

**How it works:**
1. Each property is converted to a rich text description (name + region + bedroom type + price range)
2. The text is embedded into a 128-dimensional vector using deterministic feature hashing
3. Search queries are embedded the same way
4. pgvector's cosine distance operator (`<=>`) finds the nearest matches
5. Results are ranked by similarity percentage

```
"beach house"  →  Shiny house on the coast (87% match)
                   House on the sky (42% match)
                   Opera house (31% match)
```

### Live Auction

> Any visitor can place bids on properties through the auction page. The current price updates in real time as bids come in.

![Auction](https://images.unsplash.com/photo-1560520031-3a4dc4e9de0c?auto=format&fit=crop&w=900&q=80)

### Property Management (Authenticated)

> Signed-in users can **add**, **edit**, and **delete** property listings. Authentication uses bcrypt-hashed passwords with signed cookie sessions.

![Dashboard](https://images.unsplash.com/photo-1600596542815-ffad4c1539a9?auto=format&fit=crop&w=900&q=80)

---

## Tech Stack

| Layer | Technology |
|---|---|
| **Web framework** | [Axum 0.7](https://github.com/tokio-rs/axum) — async, tower-based |
| **Database** | PostgreSQL + [pgvector](https://github.com/pgvector/pgvector) |
| **ORM / queries** | [SQLx 0.8](https://github.com/launchbadge/sqlx) — async, compile-time safe |
| **Vector search** | [pgvector-rust 0.4](https://github.com/pgvector/pgvector-rust) |
| **Templates** | [Tera](https://keats.github.io/tera/) (Jinja2-like) |
| **Auth** | bcrypt + signed cookies (axum-extra) |
| **Frontend** | Bootstrap 5 + Font Awesome |
| **Runtime** | Tokio |

---

## Features

- **Vector search** — semantic property search via pgvector cosine similarity
- **User auth** — sign up, log in, log out with bcrypt-hashed passwords
- **Property CRUD** — create, read, update, delete listings (auth required)
- **Bedroom filters** — browse by 1-bed, 2-bed, or 3+ bed properties
- **Live auction** — place bids on any property
- **Auto-backfill** — on startup, generates embeddings for any properties missing one
- **Responsive UI** — Bootstrap 5, works on mobile and desktop

---

## Project Structure

```
live-bet/
├── Cargo.toml                  # Rust dependencies
├── .env.example                # Environment variable template
├── db/
│   └── schema.sql              # Database schema + seed data + pgvector
├── src/
│   ├── main.rs                 # Router, app state, startup
│   ├── db.rs                   # PostgreSQL connection pool
│   ├── models/
│   │   ├── property.rs         # Property CRUD queries + embedding storage
│   │   ├── user.rs             # User auth queries + bcrypt
│   │   └── search.rs           # Vector embedding + pgvector similarity search
│   └── handlers/
│       ├── properties.rs       # Property route handlers
│       ├── search.rs           # GET /search — vector search handler
│       ├── sessions.rs         # Login / logout
│       └── users.rs            # Sign up
├── templates/                  # Tera HTML templates
│   ├── layout.html             # Base layout (Bootstrap 5)
│   ├── properties/
│   │   ├── index.html          # Home — listings + search bar
│   │   ├── search.html         # Vector search results
│   │   ├── new.html            # Add property form
│   │   ├── edit.html           # Edit property form
│   │   ├── auction.html        # Live bidding page
│   │   ├── house.html          # Contact page
│   │   ├── one_bedroom.html    # 1-bed filter
│   │   ├── two_bedroom.html    # 2-bed filter
│   │   └── three_bedroom.html  # 3+ bed filter
│   ├── sessions/
│   │   └── new.html            # Login form
│   └── users/
│       └── new.html            # Signup form
└── public/
    └── stylesheets/
        └── main.css
```

---

## Getting Started

### Prerequisites

- **Rust** (stable) — [rustup.rs](https://rustup.rs)
- **PostgreSQL** with the **pgvector** extension

Install pgvector:
```bash
# macOS (Homebrew)
brew install pgvector

# Ubuntu/Debian
sudo apt install postgresql-16-pgvector

# Or build from source: https://github.com/pgvector/pgvector#installation
```

### Setup

```bash
# 1. Create database and load schema
createdb real_estate_db
psql real_estate_db < db/schema.sql

# 2. Configure environment
cp .env.example .env
# Edit .env — set DATABASE_URL and SECRET_KEY

# 3. Run
cargo run
```

Server starts at **http://localhost:8080**.

### Running Tests

```bash
cargo test
```

Tests cover:
- Vector embedding generation (dimensionality, normalization, determinism)
- Semantic similarity (similar queries produce closer vectors)
- Property text enrichment (price ranges, bedroom descriptions)
- Bcrypt password verification
- Model serialization / deserialization
- Template rendering

---

## Environment Variables

| Variable | Description | Default |
|---|---|---|
| `DATABASE_URL` | PostgreSQL connection string | *(required)* |
| `SECRET_KEY` | Cookie signing secret (min 64 bytes) | `"0" * 64` |
| `PORT` | Listen port | `8080` |

---

## API Routes

| Method | Path | Auth | Description |
|---|---|---|---|
| GET | `/` | No | Home page — property listings + search bar |
| GET | `/search?q=...` | No | Vector similarity search |
| GET | `/auction` | No | Live auction — place bids |
| GET | `/one_bedroom` | No | Filter: 1-bedroom properties |
| GET | `/two_bedroom` | No | Filter: 2-bedroom properties |
| GET | `/three_bedroom` | No | Filter: 3+ bedroom properties |
| GET | `/house` | No | Contact page |
| GET | `/users/new` | No | Sign up form |
| POST | `/users` | No | Create account |
| GET | `/sessions/new` | No | Log in form |
| POST | `/sessions` | No | Log in / log out |
| GET | `/properties/new` | Yes | New property form |
| POST | `/properties/` | Yes | Create property |
| GET | `/properties/:id` | Yes | Edit property form |
| POST | `/properties/:id` | Yes | Update or delete property |

---

## Vector Search — How It Works

```
User query: "luxury beach house"
         │
         ▼
┌─────────────────────┐
│  text_to_embedding() │  ← Deterministic feature hashing
│  128-dim vector      │     (3 hash positions per word, L2-normalized)
└────────┬────────────┘
         │
         ▼
┌─────────────────────┐
│  pgvector            │  ← PostgreSQL extension
│  cosine distance <=> │     Compares query vector vs all property embeddings
└────────┬────────────┘
         │
         ▼
┌─────────────────────┐
│  Ranked results      │  ← Sorted by similarity (closest first)
│  with % match score  │
└─────────────────────┘
```

The current implementation uses **deterministic feature hashing** for embeddings, which works well for keyword overlap. To upgrade to full semantic understanding, swap `text_to_embedding()` for an API call to OpenAI, Cohere, or a local sentence-transformers model.

---

## License

MIT
