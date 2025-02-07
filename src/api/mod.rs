use axum::Router;
use sqlx::Pool;
use sqlx::Postgres;

mod project_controller;
mod user_controller;

pub fn create_routes(pool: Pool<Postgres>) -> Router {
    Router::new()
        .nest("/users", user_controller::routes(pool.clone()))
        .nest("/projects", project_controller::routes(pool.clone()))
        .with_state(pool)
}
