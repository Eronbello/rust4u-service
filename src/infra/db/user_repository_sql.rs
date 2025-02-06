use crate::domain::entities::user::User;
use crate::domain::errors::domain_error::DomainError;
use crate::domain::repositories::user_repository::UserRepository;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct UserRepositorySql {
    pub pool: Pool<Postgres>,
}

impl UserRepositorySql {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositorySql {
    async fn create_user(&self, user: &User) -> Result<(), DomainError> {
        let query = r#"
            INSERT INTO users (id, username, email, password_hash, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
        "#;
        sqlx::query(query)
            .bind(user.id)
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.password_hash)
            .bind(user.created_at)
            .bind(user.updated_at)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Infra(format!("DB error: {:?}", e)))?;
        Ok(())
    }

    async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, DomainError> {
        let query = r#"
            SELECT id, username, email, password_hash, created_at, updated_at
            FROM users
            WHERE id = $1
        "#;
        let row = sqlx::query_as::<_, User>(query)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DomainError::Infra(format!("DB error: {:?}", e)))?;
        Ok(row)
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        let query = r#"
            SELECT id, username, email, password_hash, created_at, updated_at
            FROM users
            WHERE email = $1
        "#;
        let row = sqlx::query_as::<_, User>(query)
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DomainError::Infra(format!("DB error: {:?}", e)))?;
        Ok(row)
    }

    async fn update_user(&self, user: &User) -> Result<(), DomainError> {
        let query = r#"
            UPDATE users
            SET username = $1,
                email = $2,
                password_hash = $3,
                updated_at = $4
            WHERE id = $5
        "#;
        sqlx::query(query)
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.password_hash)
            .bind(user.updated_at)
            .bind(user.id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Infra(format!("DB error: {:?}", e)))?;
        Ok(())
    }

    async fn delete_user(&self, user_id: Uuid) -> Result<(), DomainError> {
        let query = r#"
            DELETE FROM users
            WHERE id = $1
        "#;
        sqlx::query(query)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Infra(format!("DB error: {:?}", e)))?;
        Ok(())
    }

    async fn list_users(&self) -> Result<Vec<User>, DomainError> {
        let query = r#"
            SELECT id, username, email, password_hash, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
        "#;
        let rows = sqlx::query_as::<_, User>(query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DomainError::Infra(format!("DB error: {:?}", e)))?;
        Ok(rows)
    }
}
