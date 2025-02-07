use crate::domain::entities::issue::{Issue, IssueStatus};
use crate::domain::errors::domain_error::DomainError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait IssueRepository {
    async fn create_issue(&self, issue: &Issue) -> Result<(), DomainError>;
    async fn get_issue_by_id(&self, issue_id: Uuid) -> Result<Option<Issue>, DomainError>;
    async fn get_issues_by_project(&self, project_id: Uuid) -> Result<Vec<Issue>, DomainError>;
    async fn update_issue(&self, issue: &Issue) -> Result<(), DomainError>;
    async fn update_issue_status(
        &self,
        issue_id: Uuid,
        status: IssueStatus,
    ) -> Result<(), DomainError>;
    async fn delete_issue(&self, issue_id: Uuid) -> Result<(), DomainError>;
    async fn list_issues(&self) -> Result<Vec<Issue>, DomainError>;
}
