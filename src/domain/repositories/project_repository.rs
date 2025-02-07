use crate::domain::entities::project::Project;
use crate::domain::errors::domain_error::DomainError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait ProjectRepository {
    async fn create_project(&self, project: &Project) -> Result<(), DomainError>;
    async fn get_project_by_id(&self, project_id: Uuid) -> Result<Option<Project>, DomainError>;
    async fn get_projects_by_owner(&self, owner_id: Uuid) -> Result<Vec<Project>, DomainError>;
    async fn update_project(&self, project: &Project) -> Result<(), DomainError>;
    async fn delete_project(&self, project_id: Uuid) -> Result<(), DomainError>;
    async fn list_projects(&self) -> Result<Vec<Project>, DomainError>;
}
