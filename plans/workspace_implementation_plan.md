# Workspace Implementation Plan

## 1. Overview
This document outlines the plan to introduce a "Workspace" strategy into the NullSlot application. This moves the system from a single-tenant view (User owns resources directly) to a multi-tenant view (User belongs to Workspaces; Workspaces own resources).

**Key Features:**
*   **Simple Auth**: Continue using Google Login & Magic Links.
*   **Workspace Model**: Many-to-Many User-Workspace relationship.
*   **Roles**: Owner, Editor, Viewer (per workspace).
*   **Invites**: Email/Link based invites.
*   **Security**: Domain restriction policies.

## 2. Database Design

We will introduce three new tables.

### 2.1. `workspaces`
Represents the tenant.
```sql
CREATE TABLE IF NOT EXISTS workspaces (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    domain_restriction TEXT, -- e.g., "company.com", nullable (no restriction)
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### 2.2. `workspace_members`
Links users to workspaces with a specific role.
```sql
CREATE TYPE workspace_role AS ENUM ('Owner', 'Editor', 'Viewer');

CREATE TABLE IF NOT EXISTS workspace_members (
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role workspace_role NOT NULL DEFAULT 'Viewer',
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (workspace_id, user_id)
);
```

### 2.3. `workspace_invites`
Stores pending invites.
```sql
CREATE TABLE IF NOT EXISTS workspace_invites (
    token_hash TEXT PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    email TEXT NOT NULL,
    role workspace_role NOT NULL DEFAULT 'Viewer',
    expires_at TIMESTAMPTZ NOT NULL,
    created_by UUID NOT NULL REFERENCES users(id), -- Who sent the invite
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## 3. Rust Models

We will define these in `backend/src/models/workspace.rs`.

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use async_graphql::{Enum, SimpleObject};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Enum, sqlx::Type)]
#[sqlx(type_name = "workspace_role")]
pub enum WorkspaceRole {
    Owner,
    Editor,
    Viewer,
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Workspace {
    pub id: Uuid,
    pub name: String,
    pub domain_restriction: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Computed fields for GraphQL can be added in the Resolver
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct WorkspaceMember {
    pub workspace_id: Uuid,
    pub user_id: Uuid,
    pub role: WorkspaceRole,
    pub joined_at: DateTime<Utc>,
}
```

## 4. API Design (GraphQL)

### 4.1. Queries

*   `myWorkspaces`: Returns list of workspaces the current user belongs to.
*   `workspace(id: UUID)`: Returns details of a specific workspace (permission check required).

### 4.2. Mutations

*   `createWorkspace(name: String, domainRestriction: Option<String>)`: Creates a new workspace and makes the creator the Owner.
*   `inviteUserToWorkspace(workspaceId: UUID, email: String, role: WorkspaceRole)`:
    *   **Logic**:
        1. Check if requester is Owner/Editor of `workspaceId`.
        2. Check `domainRestriction` against `email`.
        3. Create magic link token.
        4. Store in `workspace_invites`.
        5. (Mock) Send Email.
*   `joinWorkspace(token: String)`:
    *   **Logic**:
        1. Validate token from `workspace_invites`.
        2. Check expiration.
        3. Check `domainRestriction` again (safety).
        4. Add user to `workspace_members`.
        5. Delete invite.
*   `removeMember(workspaceId: UUID, userId: UUID)`:
    *   **Logic**: Owner can remove anyone. User can leave (remove themselves).

## 5. Auth Flow Updates

### 5.1. User Registration (First Login)
When a user logs in for the first time (via Google or Magic Link) and has **no workspaces**:
1.  System creates a "Default Workspace" for them (e.g., "Marco's Workspace").
2.  Adds them as `Owner`.

### 5.2. Login Response
The `login` mutation (or OIDC callback) currently returns a JWT.
The Frontend will immediately query `myWorkspaces` to decide where to land the user.
*   If 1 Workspace -> Redirect to Dashboard.
*   If >1 Workspaces -> Show "Select Workspace" screen or default to last used (stored in local storage).

## 6. Security & Permissions

*   **Middleware**: The existing `auth` middleware provides the `User` identity.
*   **Service Layer**: New `WorkspaceService` will handle permission checks.
    *   `verify_membership(user_id, workspace_id) -> Result<Role>`
    *   `verify_role(user_id, workspace_id, required_role) -> Result<()>`

## 7. Migration Strategy (Existing Data)
*   **Users**: Existing users will need to be backfilled.
*   **Migration Script**:
    *   Iterate all users.
    *   Create a workspace for each user.
    *   Move ownership of their resources (`timetables`, etc.) to this new workspace (Future Step, not immediate scope).
    *   *Note*: For this specific task, we are only setting up the Auth/Structure. Resource migration is a separate task.

## 8. Frontend Pages
1.  **Workspace Selector**: "Switch Workspace" dropdown.
2.  **Create Workspace Modal**: Name & Domain rules.
3.  **Workspace Settings**:
    *   List Members.
    *   Invite User form.
    *   Pending Invites list.
4.  **Invite Landing Page**: `app.nullslot.com/join?token=...`

## 9. Implementation Steps
1.  Create Migration files (`workspaces`, `members`, `invites`).
2.  Implement `Workspace` models & repository.
3.  Implement `WorkspaceService` (CRUD + Invites).
4.  Expose GraphQL Resolvers.
5.  Update Frontend to handle Workspace selection.
