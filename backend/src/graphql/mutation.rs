use async_graphql::{Context, Object, InputObject, Result};
use uuid::Uuid;
use crate::models::{Resource, Token};
use crate::service::{UserService, ResourceService};

pub struct Mutation;

#[derive(InputObject)]
pub struct RegisterInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(InputObject)]
pub struct CreateResourceInput {
    pub owner_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub metadata: serde_json::Value,
}

#[Object]
impl Mutation {
    async fn register(&self, ctx: &Context<'_>, input: RegisterInput) -> Result<Token> {
        let service = ctx.data::<UserService>()?;
        let _user = service.create_user(input.username, input.email, input.password).await?;
        
        // Mock token for now as per original implementation
        Ok(Token {
            access_token: "mock_access_token".to_string(),
            refresh_token: "mock_refresh_token".to_string(),
            expires_in: 3600,
        })
    }

    async fn create_resource(&self, ctx: &Context<'_>, input: CreateResourceInput) -> Result<Resource> {
        let service = ctx.data::<ResourceService>()?;
        
        let resource = service.create_resource(
            input.owner_id,
            input.name,
            input.description,
            input.metadata,
        ).await?;
        
        Ok(resource)
    }
}
