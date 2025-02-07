use crate::application::usecases::project_usecases::ProjectUsecases;
use crate::domain::errors::domain_error::DomainError;
use crate::infra::db::project_repository_sql::ProjectRepositorySql;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Deserialize)]
struct CreateProjectPayload {
    owner_id: Uuid,
    name: String,
    description: Option<String>,
    github_link: Option<String>,
    tags: Vec<String>,
}

#[derive(Deserialize)]
struct UpdateProjectPayload {
    name: Option<String>,
    description: Option<String>,
    github_link: Option<String>,
    tags: Option<Vec<String>>,
}

#[derive(Serialize)]
struct ProjectResponse {
    id: Uuid,
    owner_id: Uuid,
    name: String,
    description: Option<String>,
    github_link: Option<String>,
    tags: Vec<String>,
}

impl ProjectResponse {
    fn from_entity(project: crate::domain::entities::project::Project) -> Self {
        Self {
            id: project.id,
            owner_id: project.owner_id,
            name: project.name,
            description: project.description,
            github_link: project.github_link,
            tags: project.tags,
        }
    }
}

pub fn routes(pool: Pool<Postgres>) -> Router<Pool<Postgres>> {
    Router::new()
        .route("/", post(create_project).get(list_projects))
        .route(
            "/:id",
            get(get_project).put(update_project).delete(delete_project),
        )
        .with_state(pool)
}

async fn create_project(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<CreateProjectPayload>,
) -> Result<Json<ProjectResponse>, StatusCode> {
    let repo = ProjectRepositorySql::new(pool);
    let usecases = ProjectUsecases::new(repo);
    let project_entity = usecases
        .create_project(
            payload.owner_id,
            payload.name,
            payload.description,
            payload.github_link,
            payload.tags,
        )
        .await
        .map_err(map_domain_error)?;
    Ok(Json(ProjectResponse::from_entity(project_entity)))
}

async fn get_project(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ProjectResponse>, StatusCode> {
    let repo = ProjectRepositorySql::new(pool);
    let usecases = ProjectUsecases::new(repo);
    let project_entity = usecases.get_project(id).await.map_err(map_domain_error)?;
    Ok(Json(ProjectResponse::from_entity(project_entity)))
}

async fn update_project(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateProjectPayload>,
) -> Result<Json<ProjectResponse>, StatusCode> {
    let repo = ProjectRepositorySql::new(pool);
    let usecases = ProjectUsecases::new(repo);
    let project_entity = usecases
        .update_project(
            id,
            payload.name,
            payload.description,
            payload.github_link,
            payload.tags,
        )
        .await
        .map_err(map_domain_error)?;
    Ok(Json(ProjectResponse::from_entity(project_entity)))
}

async fn delete_project(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let repo = ProjectRepositorySql::new(pool);
    let usecases = ProjectUsecases::new(repo);
    usecases
        .delete_project(id)
        .await
        .map_err(map_domain_error)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn list_projects(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<ProjectResponse>>, StatusCode> {
    let repo = ProjectRepositorySql::new(pool);
    let usecases = ProjectUsecases::new(repo);
    let projects = usecases.list_projects().await.map_err(map_domain_error)?;
    let response = projects
        .into_iter()
        .map(ProjectResponse::from_entity)
        .collect();
    Ok(Json(response))
}

fn map_domain_error(err: DomainError) -> StatusCode {
    match err {
        DomainError::InvalidData(_) => StatusCode::BAD_REQUEST,
        DomainError::Conflict(_) => StatusCode::CONFLICT,
        DomainError::NotFound(_) => StatusCode::NOT_FOUND,
        DomainError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        DomainError::Infra(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
