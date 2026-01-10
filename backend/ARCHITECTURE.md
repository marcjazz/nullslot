# Backend Architecture Design (GraphQL)

## 1. Overview

The backend is a Rust service designed with a clean modular architecture to support a GraphQL API. It emphasizes a clear separation of concerns between API resolvers, domain logic, persistence, and infrastructure.

- Language: Rust
- Web framework: Axum
- API: GraphQL (async-graphql)
- Database: PostgreSQL
- ORM/Query: sqlx
- Authentication: JWT (HS256)
- Observability: tracing, structured logging

## 2. API Endpoints

### Health and Status (REST)
- GET /api/v1/health
- GET /api/v1/status

### GraphQL API
- POST /graphql: The main entry point for all GraphQL queries and mutations.
- GET /graphql: Serves the GraphQL Playground (in development).

## 3. GraphQL Schema (Simplified)

### Queries
- `users`: Returns a list of users.
- `user(id: UUID)`: Returns a specific user.
- `resources`: Returns a list of resources.
- `resource(id: UUID)`: Returns a specific resource.

### Mutations
- `register(input: RegisterInput)`: Registers a new user and returns tokens.
- `login(input: LoginInput)`: Authenticates a user and returns tokens.
- `createResource(input: CreateResourceInput)`: Creates a new resource.

## 4. Data Models

The internal data structures use `async-graphql` derives to expose them to the schema.

### User
```rust
struct User {
  id: uuid::Uuid,
  username: String,
  email: String,
  role: UserRole,
  created_at: chrono::DateTime<chrono::Utc>,
}
```

## 5. Error Handling Strategy

- Centralized `AppError` type.
- GraphQL resolvers return `Result<T, async_graphql::Error>`.
- Internal errors are logged and obscured from the client if necessary.

## 6. Persistence and Domain Layer

- PostgreSQL database with `sqlx`.
- Repository pattern for DB access.
- Service layer for domain logic.
