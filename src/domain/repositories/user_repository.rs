use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::entities::user::User;
use crate::domain::errors::domain_error::DomainError;

#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, user: &User) -> Result<(), DomainError>;
    async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, DomainError>;
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
    async fn update_user(&self, user: &User) -> Result<(), DomainError>;
    async fn delete_user(&self, user_id: Uuid) -> Result<(), DomainError>;
    async fn list_users(&self) -> Result<Vec<User>, DomainError>;
}
