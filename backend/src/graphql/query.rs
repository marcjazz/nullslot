use async_graphql::{Context, Object, Result};
use uuid::Uuid;
use crate::models::{User, Resource};
use crate::service::{UserService, ResourceService};

pub struct Query;

#[Object]
impl Query {
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let service = ctx.data::<UserService>()?;
        Ok(service.get_all_users().await?)
    }

    async fn user(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<User>> {
        let service = ctx.data::<UserService>()?;
        Ok(service.get_user(id).await?)
    }

    async fn resources(&self, ctx: &Context<'_>) -> Result<Vec<Resource>> {
        let service = ctx.data::<ResourceService>()?;
        Ok(service.get_all_resources().await?)
    }

    async fn resource(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Resource>> {
        let service = ctx.data::<ResourceService>()?;
        Ok(service.get_resource(id).await?)
    }
}
