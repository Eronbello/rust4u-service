use crate::domain::entities::issue::{Issue, IssueStatus};
use crate::domain::errors::domain_error::DomainError;
use crate::domain::repositories::issue_repository::IssueRepository;
use chrono::Utc;
use uuid::Uuid;

pub struct IssueUsecases<R: IssueRepository> {
    repository: R,
}

impl<R: IssueRepository> IssueUsecases<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_issue(
        &self,
        project_id: Uuid,
        title: String,
        description: Option<String>,
        bounty_value: f64,
    ) -> Result<Issue, DomainError> {
        if title.is_empty() {
            return Err(DomainError::InvalidData(
                "Issue title cannot be empty".to_string(),
            ));
        }

        let issue = Issue {
            id: Uuid::new_v4(),
            project_id,
            title,
            description,
            bounty_value,
            status: IssueStatus::Open,
            created_at: Utc::now(),
            updated_at: None,
        };

        self.repository.create_issue(&issue).await?;
        Ok(issue)
    }

    pub async fn get_issue(&self, issue_id: Uuid) -> Result<Issue, DomainError> {
        if let Some(issue) = self.repository.get_issue_by_id(issue_id).await? {
            Ok(issue)
        } else {
            Err(DomainError::NotFound("Issue not found".to_string()))
        }
    }

    pub async fn get_issues_by_project(&self, project_id: Uuid) -> Result<Vec<Issue>, DomainError> {
        self.repository.get_issues_by_project(project_id).await
    }

    pub async fn update_issue(
        &self,
        issue_id: Uuid,
        new_title: Option<String>,
        new_description: Option<String>,
        new_bounty_value: Option<f64>,
        new_status: Option<IssueStatus>,
    ) -> Result<Issue, DomainError> {
        let mut issue = self.get_issue(issue_id).await?;

        if let Some(title) = new_title {
            if !title.is_empty() {
                issue.title = title;
            }
        }
        if let Some(description) = new_description {
            issue.description = Some(description);
        }
        if let Some(bounty_value) = new_bounty_value {
            issue.bounty_value = bounty_value;
        }
        if let Some(status) = new_status {
            issue.status = status;
        }

        issue.updated_at = Some(Utc::now());
        self.repository.update_issue(&issue).await?;
        Ok(issue)
    }

    pub async fn update_issue_status(
        &self,
        issue_id: Uuid,
        status: IssueStatus,
    ) -> Result<(), DomainError> {
        self.repository.update_issue_status(issue_id, status).await
    }

    pub async fn delete_issue(&self, issue_id: Uuid) -> Result<(), DomainError> {
        self.repository.delete_issue(issue_id).await
    }

    pub async fn list_issues(&self) -> Result<Vec<Issue>, DomainError> {
        self.repository.list_issues().await
    }
}
