# Backend Authentication Implementation Plan

This document outlines the implementation strategy for the backend authentication system, supporting both Magic Link and OIDC (SSO) flows, as well as JWT management and middleware integration.

## 1. Database Schema Updates

We need to store magic link tokens. We will create a new table `magic_links`.

### Migration: `create_magic_links`

```sql
CREATE TABLE IF NOT EXISTS magic_links (
    token TEXT PRIMARY KEY,
    user_email TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    used BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for faster lookups by email if needed for rate limiting or cleanup
CREATE INDEX idx_magic_links_email ON magic_links(user_email);
```

*Note: We store `user_email` instead of `user_id` to allow sending magic links to users who might not exist yet (if we wanted to support invite flows), though strictly for login, they should exist. Linking to email allows decoupling the token from the user ID until verification.*

## 2. Magic Link Authentication

### Repository Layer (`backend/src/repository/auth.rs`)

We need functions to manage the lifecycle of a magic link token.

-   `create_magic_link(token: String, email: String, expires_at: DateTime<Utc>) -> Result<()>`: Inserts a new token.
-   `find_magic_link(token: &str) -> Result<Option<MagicLink>>`: Retrieves a token if it exists and is not expired.
-   `mark_magic_link_used(token: &str) -> Result<()>`: Sets `used = true` to prevent replay attacks.

### Service Layer (`backend/src/service/auth.rs`)

-   `request_magic_link(email: String) -> Result<()>`:
    1.  Check if a user with this `email` exists in the `users` table. If not, return an error (or fail silently for security).
    2.  Generate a secure random token (e.g., using `uuid` or `rand` crate, URL-safe string).
    3.  Calculate expiration (e.g., 15 minutes from now).
    4.  Call `repository.create_magic_link(...)`.
    5.  **Mock Email Sending**: For now, log the link `http://localhost:5173/auth/verify?token=<token>` to stdout. In production, this would use an email client.

-   `login_with_magic_link(token: String) -> Result<AuthResponse>`:
    1.  Call `repository.find_magic_link(token)`.
    2.  Validate:
        -   Token exists.
        -   `expires_at` > `now`.
        -   `used` is `false`.
    3.  If valid, call `repository.mark_magic_link_used(token)`.
    4.  Find the user by `email` (from the token record).
    5.  Generate a JWT for this user (see Section 4).
    6.  Return the JWT (and refresh token if implemented).

### API Layer

-   **GraphQL Mutation**: `requestMagicLink(email: String!) -> Boolean`
    -   Calls `service.request_magic_link(email)`.
-   **GraphQL Mutation**: `loginWithMagicLink(token: String!) -> AuthPayload`
    -   Calls `service.login_with_magic_link(token)`.
    -   Returns `{ token: String, user: User }`.

## 3. OIDC/SSO Authentication

### Configuration

Environment variables required in `.env`:

```env
OIDC_CLIENT_ID=...
OIDC_CLIENT_SECRET=...
OIDC_ISSUER_URL=... (e.g., https://accounts.google.com or university SSO)
OIDC_REDIRECT_URL=http://localhost:3000/auth/oidc/callback
```

### Library Suggestion

-   **Crate**: [`openidconnect`](https://crates.io/crates/openidconnect)
    -   It is the standard, robust library for OIDC in Rust.
    -   Supports discovery, authorization code flow, and token validation.

### REST Endpoints (`backend/src/api/auth.rs`)

Since OIDC requires browser redirects, we will use standard REST endpoints (Axum handlers) instead of GraphQL for the initial handshake.

-   `GET /auth/oidc/login`:
    1.  Initialize the `CoreClient` from `openidconnect`.
    2.  Generate an authorization URL (with CSRF token and nonce).
    3.  (Optional) Store CSRF token in a cookie or cache for validation.
    4.  Redirect the user (`302 Found`) to the provider's authorization URL.

-   `GET /auth/oidc/callback`:
    1.  Extract `code` and `state` (CSRF) from query parameters.
    2.  Validate CSRF token.
    3.  Exchange `code` for an ID Token and Access Token using the `CoreClient`.
    4.  Validate the ID Token (signature, issuer, audience, nonce).
    5.  Extract claims (specifically `email` and `sub`).
    6.  **User Sync**:
        -   Check if user exists by `email`.
        -   If yes, update any details if needed.
        -   If no, we might auto-create the user OR reject depending on policy (APP_FLOW implies "Identity comes from SSO, Authority comes from database", suggesting users might need to be pre-provisioned or auto-provisioned with a default role). *Assumption: We will auto-provision or match existing users by email.*
    7.  Generate a local JWT.
    8.  Redirect the user back to the frontend (e.g., `http://localhost:5173/auth/callback?token=<jwt>`).

## 4. JWT Management & Middleware

### Token Generation (`backend/src/service/auth.rs`)

-   **Crate**: `jsonwebtoken` (already in Cargo.toml).
-   **Claims Struct**:
    ```rust
    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        sub: Uuid,       // User ID
        role: UserRole,  // "Admin", "Teacher", etc.
        exp: usize,      // Expiration timestamp
        iat: usize,      // Issued at
    }
    ```
-   **Signing**: Use `HS256` with a `JWT_SECRET` env var.

### Middleware (`backend/src/middleware/auth.rs`)

We need an Axum middleware that runs before the GraphQL handler.

1.  **Extract Header**: Look for `Authorization: Bearer <token>`.
2.  **Validate**:
    -   Decode the token using `jsonwebtoken`.
    -   Verify signature and expiration.
3.  **Context Injection**:
    -   If valid, insert the `Claims` (or full `User` object) into the Axum request extensions (`req.extensions_mut().insert(claims)`).
    -   If invalid or missing, do *not* fail immediately (unless the route is strictly protected). Instead, leave the extension empty. This allows the GraphQL schema to handle public vs. private fields, or we can enforce it strictly for the `/graphql` endpoint if the entire API is private.
    -   *Recommendation*: For this system, most endpoints are protected. We can have a `AuthGuard` in GraphQL or enforce it at the handler level.

### GraphQL Context (`backend/src/graphql/mod.rs`)

-   In the `graphql_handler`, extract the `Claims` from the request extensions.
-   Pass it into the `async_graphql::Context` data.
-   This allows resolvers to access `ctx.data::<Claims>()` to check permissions (e.g., `if claims.role == UserRole::Admin`).

## 5. Summary of Work

1.  **Migrations**: Add `magic_links` table.
2.  **Dependencies**: Add `openidconnect`, `reqwest` (if not present for OIDC client).
3.  **Repository**: Implement `MagicLink` operations.
4.  **Service**: Implement `request_magic_link`, `login_with_magic_link`, `generate_jwt`.
5.  **API (REST)**: Implement OIDC `login` and `callback` handlers.
6.  **API (GraphQL)**: Implement Magic Link mutations.
7.  **Middleware**: Implement JWT validation and context injection.
