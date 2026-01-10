use async_graphql::{EmptySubscription, Schema};
use crate::graphql::query::Query;
use crate::graphql::mutation::Mutation;
use crate::repository::{UserRepository, ResourceRepository};
use crate::service::{UserService, ResourceService};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema(pool: sqlx::PgPool) -> AppSchema {
    let user_repo = UserRepository::new(pool.clone());
    let resource_repo = ResourceRepository::new(pool.clone());
    
    let user_service = UserService::new(user_repo);
    let resource_service = ResourceService::new(resource_repo);

    Schema::build(Query, Mutation, EmptySubscription)
        .data(pool)
        .data(user_service)
        .data(resource_service)
        .finish()
}
