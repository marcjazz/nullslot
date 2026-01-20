use backend::{api, config::Config, graphql, oidc, ws::Broadcaster, AppState};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    let config = Arc::new(Config::from_env());

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Database connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    tracing::info!("Database migrations completed successfully");

    // Create Broadcaster for WebSockets
    let broadcaster = Arc::new(Broadcaster::new(1024));

    // Discover OIDC client
    let oidc_client = Arc::new(oidc::discover_oidc_client(&config).await?);

    // Create GraphQL schema
    let schema = graphql::create_schema(pool, broadcaster.clone(), config.clone(), oidc_client.clone());

    // Create AppState
    let state = AppState {
        broadcaster: broadcaster.clone(),
        config: config.clone(),
        oidc_client,
    };

    // Setup router
    let app = api::router(schema, state);

    // Define address
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);

    // Run server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
