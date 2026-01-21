# 🏗️ NullSlot Architecture

This document provides a comprehensive overview of the NullSlot application's architecture, from the high-level concepts down to the backend and frontend implementation details.

---

## 1. Core Concepts

### 1.1. Workspace-centric Multitenancy

NullSlot operates on a multi-tenant model where each tenant is a **Workspace**. All resources, such as timetables, users, and availability, belong to a workspace. This ensures data isolation and allows users to be part of multiple distinct environments.

- **Users** can be members of multiple workspaces.
- **Roles** are defined on a per-workspace basis (`Owner`, `Editor`, `Viewer`).
- Access to any resource requires the user to have the appropriate role within the resource's workspace.

### 1.2. Authentication Flow

Authentication is designed to be simple and secure, supporting both SSO and passwordless methods.

1.  **Initiation**: The user logs in via:
    *   **Google OAuth2 (OIDC)**: For users with a Google account.
    *   **Magic Link**: For passwordless email-based login.

2.  **Verification**: The backend verifies the user's identity with the respective provider (Google or by validating the magic link token).

3.  **JWT Issuance**: Upon successful verification, the backend issues a JSON Web Token (JWT) to the frontend. This token contains the user's identity but is **agnostic of any specific workspace**.

4.  **Workspace Context**:
    *   After receiving the JWT, the frontend queries for the user's available workspaces (`myWorkspaces`).
    *   The user selects a workspace.
    *   All subsequent API requests from the frontend must include the `X-Workspace-ID` header.

5.  **Authorization**: The backend's auth middleware validates the JWT *and* uses the `X-Workspace-ID` header to check the user's permissions for the requested resource within that specific workspace.

```
               ┌───────────────┐        ┌───────────────┐
               │   Frontend    │        │    Backend    │
               └───────┬───────┘        └───────┬───────┘
                       │                        │
                       │─── Login (Google/Magic Link) ───▶│
                       │                        │
                       │◀────────── JWT ───────────│
                       │                        │
                       │── Get My Workspaces ──▶│
                       │  (with JWT)           │
                       │                        │
                       │◀── List of Workspaces ──│
                       │                        │
           (User Selects Workspace)             │
                       │                        │
                       │─ API Request (GraphQL) ─▶│
                       │  - Authorization: JWT   │
                       │  - X-Workspace-ID: uuid │
                       │                        │
                       │                        │─── 1. Validate JWT
                       │                        │─── 2. Check Workspace Membership
                       │                        │─── 3. Process Request
                       │                        │
                       │◀─── API Response ────│
                       │                        │
```

---

## 2. Technology Stack

| Component | Technology | Description |
| :--- | :--- | :--- |
| **Backend** | Rust (Axum Framework) | Provides a high-performance, type-safe foundation. |
| **API** | GraphQL (async-graphql) | Offers a flexible and strongly-typed API for the frontend. |
| **Database** | PostgreSQL | The primary relational database for all persistent data. |
| **DB Access** | SQLx | Provides compile-time checked queries against the database. |
| **Frontend** | React (Vite, TypeScript) | A modern, fast, and type-safe single-page application. |
| **Deployment**| Docker | Containerizes the backend and frontend for consistent environments. |

---

## 3. Backend Design

The backend follows a standard layered architecture to ensure separation of concerns.

-   **GraphQL Layer (`graphql/`)**: Defines the GraphQL schema, queries, mutations, and resolvers. This is the primary entry point for the API.
-   **Service Layer (`service/`)**: Contains the core business logic. It orchestrates operations, enforces permissions, and interacts with the repository layer.
-   **Repository Layer (`repository/`)**: Handles all database interactions using `sqlx`. It abstracts the raw SQL queries from the service layer.
-   **Models (`models/`)**: Defines the core data structures (e.g., `Workspace`, `User`, `Timetable`) that are used across all layers.

---

## 4. Application Flow

The system is coordinator-driven, with a clear distinction between drafting and publishing timetables.

1.  **Availability Collection**: Teachers log in and submit their availability for the upcoming period. This data is saved in a draft state.
2.  **Timetable Drafting & Conflict Resolution**: A coordinator reviews teacher availability, resolves scheduling conflicts, and builds a draft timetable.
3.  **Publishing**: Once the draft is finalized, the coordinator publishes it. This action is a hard boundary:
    *   The published timetable becomes the official, immutable schedule.
    *   Notifications (e.g., calendar invites) are sent to relevant staff.
4.  **Consumption**: The published timetable is now viewable by all users. The frontend caches this data (e.g., in IndexedDB) to provide read-only offline access.
