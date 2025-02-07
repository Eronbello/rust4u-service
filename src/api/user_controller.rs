use crate::application::usecases::user_usecases::UserUsecases;
use crate::domain::errors::domain_error::DomainError;
use crate::infra::db::user_repository_sql::UserRepositorySql;
use crate::infra::jwt::{generate_jwt, Claims};
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Deserialize)]
struct RegisterPayload {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginPayload {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct UpdatePayload {
    username: Option<String>,
    password: Option<String>,
}

#[derive(Serialize)]
struct UserResponse {
    id: Uuid,
    username: String,
    email: String,
    token: Option<String>,
}

impl UserResponse {
    fn from_entity(user: crate::domain::entities::user::User, token: Option<String>) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            token,
        }
    }
}

pub fn routes(pool: Pool<Postgres>) -> Router<Pool<Postgres>> {
    Router::new()
        .route("/", post(register_user).get(list_users))
        .route("/login", post(login_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
        .with_state(pool)
}

// ------------------------
// Handlers

async fn register_user(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<UserResponse>, StatusCode> {
    let repo = UserRepositorySql::new(pool);
    let usecases = UserUsecases::new(repo);
    let user_entity = usecases
        .register_user(payload.username, payload.email, payload.password)
        .await
        .map_err(map_domain_error)?;
    // Gera token JWT imediatamente após registro, se quiser
    let token = generate_jwt(user_entity.id).ok();
    Ok(Json(UserResponse::from_entity(user_entity, token)))
}

async fn login_user(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<UserResponse>, StatusCode> {
    let repo = UserRepositorySql::new(pool);
    let usecases = UserUsecases::new(repo);
    let user_entity = usecases
        .login_user(payload.email, payload.password)
        .await
        .map_err(map_domain_error)?;
    let token = generate_jwt(user_entity.id).ok();
    Ok(Json(UserResponse::from_entity(user_entity, token)))
}

async fn get_user(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<Json<UserResponse>, StatusCode> {
    // Validar JWT
    let claims = check_auth(&headers)?;

    let repo = UserRepositorySql::new(pool);
    let usecases = UserUsecases::new(repo);
    let user_entity = usecases.get_user(id).await.map_err(map_domain_error)?;

    // Verifica se quem requisita é o mesmo user ou algo do tipo (opcional)
    if claims.sub != id {
        // Decida se quer permitir que outra pessoa veja o user
        // Por simplicidade, vamos permitir
    }

    Ok(Json(UserResponse::from_entity(user_entity, None)))
}

async fn update_user(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<UpdatePayload>,
) -> Result<Json<UserResponse>, StatusCode> {
    let claims = check_auth(&headers)?;

    // Checar se o id do token bate com o user que está sendo atualizado, ou se é admin, etc.
    if claims.sub != id {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let repo = UserRepositorySql::new(pool);
    let usecases = UserUsecases::new(repo);
    let user_entity = usecases
        .update_user(id, payload.username.clone(), payload.password.clone())
        .await
        .map_err(map_domain_error)?;

    Ok(Json(UserResponse::from_entity(user_entity, None)))
}

async fn delete_user(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<StatusCode, StatusCode> {
    let claims = check_auth(&headers)?;

    if claims.sub != id {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let repo = UserRepositorySql::new(pool);
    let usecases = UserUsecases::new(repo);

    usecases.delete_user(id).await.map_err(map_domain_error)?;

    Ok(StatusCode::NO_CONTENT)
}

async fn list_users(
    State(pool): State<Pool<Postgres>>,
    headers: HeaderMap,
) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    // Em tese, só usuários logados podem ver a lista
    let _claims = check_auth(&headers)?;

    let repo = UserRepositorySql::new(pool);
    let usecases = UserUsecases::new(repo);

    let users = usecases.list_users().await.map_err(map_domain_error)?;
    let resp = users
        .into_iter()
        .map(|u| UserResponse::from_entity(u, None))
        .collect();
    Ok(Json(resp))
}

// ------------------------
// Aux Functions

fn map_domain_error(err: DomainError) -> StatusCode {
    match err {
        DomainError::InvalidData(_) => StatusCode::BAD_REQUEST,
        DomainError::Conflict(_) => StatusCode::CONFLICT,
        DomainError::NotFound(_) => StatusCode::NOT_FOUND,
        DomainError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        DomainError::Infra(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn check_auth(headers: &HeaderMap) -> Result<Claims, StatusCode> {
    if let Some(auth_header) = headers.get("Authorization") {
        let auth_str = auth_header.to_str().unwrap_or("");
        if auth_str.starts_with("Bearer ") {
            let token = auth_str.trim_start_matches("Bearer ").trim();
            let claims =
                crate::infra::jwt::validate_jwt(token).map_err(|_| StatusCode::UNAUTHORIZED)?;
            return Ok(claims);
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}
