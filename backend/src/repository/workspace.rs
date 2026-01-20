use sqlx::PgPool;
use uuid::Uuid;
use crate::models::workspace::{Workspace, WorkspaceInvite, WorkspaceRole};
use crate::error::AppResult;

#[derive(Clone)]
pub struct WorkspaceRepository {
    pool: PgPool,
}

impl WorkspaceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, workspace: Workspace) -> AppResult<Workspace> {
        sqlx::query!(
            r#"
            INSERT INTO workspaces (id, name, domain_restriction, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            workspace.id,
            workspace.name,
            workspace.domain_restriction,
            workspace.created_at,
            workspace.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(workspace)
    }

    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Workspace>> {
        let workspace = sqlx::query_as!(
            Workspace,
            r#"
            SELECT id, name, domain_restriction, created_at, updated_at
            FROM workspaces
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(workspace)
    }

    pub async fn find_by_user_id(&self, user_id: Uuid) -> AppResult<Vec<Workspace>> {
        let workspaces = sqlx::query_as!(
            Workspace,
            r#"
            SELECT w.id, w.name, w.domain_restriction, w.created_at, w.updated_at
            FROM workspaces w
            JOIN workspace_members wm ON w.id = wm.workspace_id
            WHERE wm.user_id = $1
            ORDER BY w.name ASC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(workspaces)
    }

    pub async fn add_member(&self, workspace_id: Uuid, user_id: Uuid, role: WorkspaceRole) -> AppResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO workspace_members (workspace_id, user_id, role)
            VALUES ($1, $2, $3)
            ON CONFLICT (workspace_id, user_id) DO UPDATE SET role = EXCLUDED.role
            "#,
            workspace_id,
            user_id,
            role as WorkspaceRole
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn check_membership(&self, workspace_id: Uuid, user_id: Uuid) -> AppResult<Option<WorkspaceRole>> {
        let member = sqlx::query!(
            r#"
            SELECT role as "role: WorkspaceRole"
            FROM workspace_members
            WHERE workspace_id = $1 AND user_id = $2
            "#,
            workspace_id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(member.map(|m| m.role))
    }

    pub async fn create_invite(&self, invite: WorkspaceInvite) -> AppResult<WorkspaceInvite> {
        sqlx::query!(
            r#"
            INSERT INTO workspace_invites (token_hash, workspace_id, email, role, expires_at, created_by, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            invite.token_hash,
            invite.workspace_id,
            invite.email,
            invite.role as WorkspaceRole,
            invite.expires_at,
            invite.created_by,
            invite.created_at
        )
        .execute(&self.pool)
        .await?;

        Ok(invite)
    }

    pub async fn find_invite(&self, token_hash: &str) -> AppResult<Option<WorkspaceInvite>> {
        let invite = sqlx::query_as!(
            WorkspaceInvite,
            r#"
            SELECT token_hash, workspace_id, email, role as "role: WorkspaceRole", expires_at, created_by, created_at
            FROM workspace_invites
            WHERE token_hash = $1
            "#,
            token_hash
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(invite)
    }

    pub async fn delete_invite(&self, token_hash: &str) -> AppResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM workspace_invites
            WHERE token_hash = $1
            "#,
            token_hash
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
