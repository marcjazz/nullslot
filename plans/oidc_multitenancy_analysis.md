# OIDC/SSO Multitenancy Analysis

## Conclusion
The current OIDC/SSO implementation **does not support multitenancy**. It is designed for a single-tenant deployment.

## Detailed Findings

### 1. Configuration (`backend/src/config.rs`)
*   OIDC configuration (Client ID, Secret, Issuer, Redirect URI) is loaded directly from static environment variables:
    *   `OIDC_CLIENT_ID`
    *   `OIDC_CLIENT_SECRET`
    *   `OIDC_ISSUER_URL`
    *   `OIDC_REDIRECT_URI`
*   There is no mechanism to load dynamic configurations or multiple sets of credentials based on a tenant identifier.

### 2. OIDC Client (`backend/src/oidc.rs`)
*   A single `OidcClient` (instance of `CoreClient`) is constructed during application startup using the static configuration.
*   This single client is stored in the application state and used for all requests.

### 3. Database Schema (`backend/migrations`)
*   The `users` table (`20260110000000_create_users.sql`) defines the user schema.
*   **Missing Field**: There is no `tenant_id`, `organization_id`, or similar foreign key to associate a user with a specific tenant.
*   Users are globally unique by `email` and `username` across the entire database instance.

### 4. Authentication Logic (`backend/src/api/auth.rs`)
*   `oidc_login_handler`:
    *   Uses the single `state.oidc_client` to generate the authorization URL.
    *   Does not accept any path parameters (e.g., `/auth/{tenant}/login`) or query parameters to select an Identity Provider.
*   `oidc_callback_handler`:
    *   Processes the callback using the same single client.
    *   Assumes all users belong to the same context.

## Recommendations for Multitenancy
To support multitenancy, the following changes would be required:
1.  **Database**: Add a `tenants` table and a `tenant_id` column to the `users` table.
2.  **Configuration**: Move OIDC config from env vars to the `tenants` table (encrypted).
3.  **Dynamic Client Registry**: Implement a mechanism to instantiate or retrieve `OidcClient` instances dynamically based on the target tenant (e.g., a map of `tenant_id -> OidcClient` in memory, or lazy creation).
4.  **API Routes**: Update auth routes to include tenant context (e.g., `GET /api/v1/{tenant_id}/auth/login`).
