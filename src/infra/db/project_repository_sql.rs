use crate::domain::entities::project::Project;
use crate::domain::errors::domain_error::DomainError;
use crate::domain::repositories::project_repository::ProjectRepository;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct ProjectRepositorySql {
    pub pool: Pool<Postgres>,
}

impl ProjectRepositorySql {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProjectRepository for ProjectRepositorySql {
    async fn create_project(&self, project: &Project) -> Result<(), DomainError> {
        let query = r#"
            INSERT INTO projects (id, owner_id, name, description, github_link, tags, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#;
        sqlx::query(query)
            .bind(project.id)
            .bind(project.owner_id)
            .bind(&project.name)
            .bind(&project.description)
            .bind(&project.github_link)
            .bind(&project.tags)
            .bind(project.created_at)
            .bind(project.updated_at)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Infra(format!("DB error: {:?}", e)))?;
        Ok(())
    }

    async fn get_project_by_id(&self, project_id: Uuid) -> Result<Option<Project>, DomainError> {
        let query = r#"
            SELECT id, owner_id, name, description, github_link, tags, created_at, updated_at
            FROM projects
            WHERE id = $1
        "#;
        let row = sqlx::query_as::<_, Project>(query)
            .bind(project_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DomainError::Infra(format!("DB error: {:?}", e)))?;
        Ok(row)
    }

    async fn get_projects_by_owner(&self, owner_id: Uuid) -> Result<Vec<Project>, DomainError> {
        let query = r#"
            SELECT id, owner_id, name, description, github_link, tags, created_at, updated_at
            FROM projects
            WHERE owner_id = $1
            ORDER BY created_at DESC
        "#;
        let rows = sqlx::query_as::<_, Project>(query)
            .bind(owner_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DomainError::Infra(format!("DB error: {:?}", e)))?;
        Ok(rows)
    }

    async fn update_project(&self, project: &Project) -> Result<(), DomainError> {
        let query = r#"
            UPDATE projects
            SET name = $1,
                description = $2,
                github_link = $3,
                tags = $4,
                updated_at = $5
            WHERE id = $6
        "#;
        sqlx::query(query)
            .bind(&project.name)
            .bind(&project.description)
            .bind(&project.github_link)
            .bind(&project.tags)
            .bind(project.updated_at)
            .bind(project.id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Infra(format!("DB error: {:?}", e)))?;
        Ok(())
    }

    async fn delete_project(&self, project_id: Uuid) -> Result<(), DomainError> {
        let query = "DELETE FROM projects WHERE id = $1";
        sqlx::query(query)
            .bind(project_id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Infra(format!("DB error: {:?}", e)))?;
        Ok(())
    }

    async fn list_projects(&self) -> Result<Vec<Project>, DomainError> {
        let query = r#"
            SELECT id, owner_id, name, description, github_link, tags, created_at, updated_at
            FROM projects
            ORDER BY created_at DESC
        "#;
        let rows = sqlx::query_as::<_, Project>(query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DomainError::Infra(format!("DB error: {:?}", e)))?;
        Ok(rows)
    }
}
