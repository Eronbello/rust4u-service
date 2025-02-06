use dotenv::dotenv;
use std::env;
use tracing_subscriber;
use rust4u_backend::api::create_routes;
use rust4u_backend::infra::db::create_db_pool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    // Create DB pool
    let pool = create_db_pool(&database_url).await?;

    // Build our application with routes
    let app = create_routes(pool);

    let addr = "0.0.0.0:3000".parse().unwrap();
    tracing::info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
