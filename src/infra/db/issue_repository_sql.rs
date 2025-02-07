use crate::domain::entities::issue::{Issue, IssueStatus};
use crate::domain::errors::domain_error::DomainError;
use crate::domain::repositories::issue_repository::IssueRepository;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use std::fmt;
use uuid::Uuid;

impl fmt::Display for IssueStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct IssueRepositorySql {
    pub pool: Pool<Postgres>,
}

impl IssueRepositorySql {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IssueRepository for IssueRepositorySql {
    async fn create_issue(&self, issue: &Issue) -> Result<(), DomainError> {
        let query = r#"
            INSERT INTO issues (id, project_id, title, description, bounty_value, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#;
        sqlx::query(query)
            .bind(issue.id)
            .bind(issue.project_id)
            .bind(&issue.title)
            .bind(&issue.description)
            .bind(issue.bounty_value)
            .bind(issue.status.to_string())
            .bind(issue.created_at)
            .bind(issue.updated_at)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Infra(format!("DB error: {:?}", e)))?;
        Ok(())
    }

    async fn get_issue_by_id(&self, issue_id: Uuid) -> Result<Option<Issue>, DomainError> {
        let query = r#"
            SELECT id, project_id, title, description, bounty_value, status, created_at, updated_at
            FROM issues
            WHERE id = $1
        "#;
        let row = sqlx::query_as::<_, Issue>(query)
            .bind(issue_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DomainError::Infra(format!("DB error: {:?}", e)))?;
        Ok(row)
    }

    async fn get_issues_by_project(&self, project_id: Uuid) -> Result<Vec<Issue>, DomainError> {
        let query = r#"
            SELECT id, project_id, title, description, bounty_value, status, created_at, updated_at
            FROM issues
            WHERE project_id = $1
            ORDER BY created_at DESC
        "#;
        let rows = sqlx::query_as::<_, Issue>(query)
            .bind(project_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DomainError::Infra(format!("DB error: {:?}", e)))?;
        Ok(rows)
    }

    async fn update_issue(&self, issue: &Issue) -> Result<(), DomainError> {
        let query = r#"
            UPDATE issues
            SET title = $1,
                description = $2,
                bounty_value = $3,
                status = $4,
                updated_at = $5
            WHERE id = $6
        "#;
        sqlx::query(query)
            .bind(&issue.title)
            .bind(&issue.description)
            .bind(issue.bounty_value)
            .bind(issue.status.to_string())
            .bind(issue.updated_at)
            .bind(issue.id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Infra(format!("DB error: {:?}", e)))?;
        Ok(())
    }

    async fn update_issue_status(
        &self,
        issue_id: Uuid,
        status: IssueStatus,
    ) -> Result<(), DomainError> {
        let query = "UPDATE issues SET status = $1 WHERE id = $2";
        sqlx::query(query)
            .bind(status.to_string())
            .bind(issue_id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Infra(format!("DB error: {:?}", e)))?;
        Ok(())
    }

    async fn delete_issue(&self, issue_id: Uuid) -> Result<(), DomainError> {
        let query = "DELETE FROM issues WHERE id = $1";
        sqlx::query(query)
            .bind(issue_id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Infra(format!("DB error: {:?}", e)))?;
        Ok(())
    }

    async fn list_issues(&self) -> Result<Vec<Issue>, DomainError> {
        let query = r#"
            SELECT id, project_id, title, description, bounty_value, status, created_at, updated_at
            FROM issues
            ORDER BY created_at DESC
        "#;
        let rows = sqlx::query_as::<_, Issue>(query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DomainError::Infra(format!("DB error: {:?}", e)))?;
        Ok(rows)
    }
}
