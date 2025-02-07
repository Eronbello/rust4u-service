use crate::domain::entities::project::Project;
use crate::domain::errors::domain_error::DomainError;
use crate::domain::repositories::project_repository::ProjectRepository;
use chrono::Utc;
use uuid::Uuid;

pub struct ProjectUsecases<R: ProjectRepository> {
    repository: R,
}

impl<R: ProjectRepository> ProjectUsecases<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_project(
        &self,
        owner_id: Uuid,
        name: String,
        description: Option<String>,
        github_link: Option<String>,
        tags: Vec<String>,
    ) -> Result<Project, DomainError> {
        if name.is_empty() {
            return Err(DomainError::InvalidData(
                "Project name cannot be empty".to_string(),
            ));
        }

        let project = Project {
            id: Uuid::new_v4(),
            owner_id,
            name,
            description,
            github_link,
            tags,
            created_at: Utc::now(),
            updated_at: None,
        };

        self.repository.create_project(&project).await?;
        Ok(project)
    }

    pub async fn get_project(&self, project_id: Uuid) -> Result<Project, DomainError> {
        if let Some(project) = self.repository.get_project_by_id(project_id).await? {
            Ok(project)
        } else {
            Err(DomainError::NotFound("Project not found".to_string()))
        }
    }

    pub async fn get_projects_by_owner(&self, owner_id: Uuid) -> Result<Vec<Project>, DomainError> {
        self.repository.get_projects_by_owner(owner_id).await
    }

    pub async fn update_project(
        &self,
        project_id: Uuid,
        new_name: Option<String>,
        new_description: Option<String>,
        new_github_link: Option<String>,
        new_tags: Option<Vec<String>>,
    ) -> Result<Project, DomainError> {
        let mut project = self.get_project(project_id).await?;

        if let Some(name) = new_name {
            if !name.is_empty() {
                project.name = name;
            }
        }
        if let Some(description) = new_description {
            project.description = Some(description);
        }
        if let Some(github_link) = new_github_link {
            project.github_link = Some(github_link);
        }
        if let Some(tags) = new_tags {
            project.tags = tags;
        }

        project.updated_at = Some(Utc::now());
        self.repository.update_project(&project).await?;
        Ok(project)
    }

    pub async fn delete_project(&self, project_id: Uuid) -> Result<(), DomainError> {
        self.repository.delete_project(project_id).await
    }

    pub async fn list_projects(&self) -> Result<Vec<Project>, DomainError> {
        self.repository.list_projects().await
    }
}
