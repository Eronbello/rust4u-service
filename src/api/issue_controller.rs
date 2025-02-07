use crate::application::usecases::issue_usecases::IssueUsecases;
use crate::domain::errors::domain_error::DomainError;
use crate::infra::db::issue_repository_sql::IssueRepositorySql;
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
struct CreateIssuePayload {
    project_id: Uuid,
    title: String,
    description: Option<String>,
    bounty_value: f64,
}

#[derive(Deserialize)]
struct UpdateIssuePayload {
    title: Option<String>,
    description: Option<String>,
    bounty_value: Option<f64>,
    status: Option<String>,
}

#[derive(Serialize)]
struct IssueResponse {
    id: Uuid,
    project_id: Uuid,
    title: String,
    description: Option<String>,
    bounty_value: f64,
    status: String,
}

impl IssueResponse {
    fn from_entity(issue: crate::domain::entities::issue::Issue) -> Self {
        Self {
            id: issue.id,
            project_id: issue.project_id,
            title: issue.title,
            description: issue.description,
            bounty_value: issue.bounty_value,
            status: issue.status.to_string(),
        }
    }
}

pub fn routes(pool: Pool<Postgres>) -> Router<Pool<Postgres>> {
    Router::new()
        .route("/", post(create_issue).get(list_issues))
        .route(
            "/:id",
            get(get_issue).put(update_issue).delete(delete_issue),
        )
        .with_state(pool)
}

async fn create_issue(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<CreateIssuePayload>,
) -> Result<Json<IssueResponse>, StatusCode> {
    let repo = IssueRepositorySql::new(pool);
    let usecases = IssueUsecases::new(repo);
    let issue_entity = usecases
        .create_issue(
            payload.project_id,
            payload.title,
            payload.description,
            payload.bounty_value,
        )
        .await
        .map_err(map_domain_error)?;
    Ok(Json(IssueResponse::from_entity(issue_entity)))
}

async fn get_issue(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
) -> Result<Json<IssueResponse>, StatusCode> {
    let repo = IssueRepositorySql::new(pool);
    let usecases = IssueUsecases::new(repo);
    let issue_entity = usecases.get_issue(id).await.map_err(map_domain_error)?;
    Ok(Json(IssueResponse::from_entity(issue_entity)))
}

async fn update_issue(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateIssuePayload>,
) -> Result<Json<IssueResponse>, StatusCode> {
    let repo = IssueRepositorySql::new(pool);
    let usecases = IssueUsecases::new(repo);
    let issue_entity = usecases
        .update_issue(
            id,
            payload.title,
            payload.description,
            payload.bounty_value,
            None,
        )
        .await
        .map_err(map_domain_error)?;
    Ok(Json(IssueResponse::from_entity(issue_entity)))
}

async fn delete_issue(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let repo = IssueRepositorySql::new(pool);
    let usecases = IssueUsecases::new(repo);
    usecases.delete_issue(id).await.map_err(map_domain_error)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn list_issues(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<IssueResponse>>, StatusCode> {
    let repo = IssueRepositorySql::new(pool);
    let usecases = IssueUsecases::new(repo);
    let issues = usecases.list_issues().await.map_err(map_domain_error)?;
    let response = issues.into_iter().map(IssueResponse::from_entity).collect();
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
