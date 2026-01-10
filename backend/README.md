# Nullslot Backend

This is the Rust backend for the Nullslot project.

## Database Setup

The backend uses PostgreSQL as its primary database.

### 1. Start PostgreSQL

You can start a PostgreSQL instance using Docker Compose:

```bash
docker-compose up -d
```

This will start a PostgreSQL 16 instance on port `5432`.

### 2. Environment Variables

The backend requires the `DATABASE_URL` environment variable to connect to the database. Create a `.env` file in the `backend` directory:

```env
DATABASE_URL=postgres://postgres:password@localhost:5432/nullslot
```

### 3. Database Migrations

Migrations are automatically run when the backend starts. They are located in the `migrations` directory.

To manually manage migrations, you can use the `sqlx-cli`:

```bash
cargo install sqlx-cli
sqlx migrate run --database-url postgres://postgres:password@localhost:5432/nullslot
```

## Running the Backend

```bash
cargo run
```

The server will be available at `http://127.0.0.1:8080`.
GraphQL Playground is available at `http://127.0.0.1:8080/graphql`.
