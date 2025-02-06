use crate::domain::entities::user::User;
use crate::domain::errors::domain_error::DomainError;
use crate::domain::repositories::user_repository::UserRepository;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use uuid::Uuid;
pub struct UserUsecases<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> UserUsecases<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn register_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<User, DomainError> {
        // Basic validations
        if username.is_empty() || email.is_empty() || password.is_empty() {
            return Err(DomainError::InvalidData(
                "Fields cannot be empty".to_string(),
            ));
        }

        // Check if email is already taken
        if let Some(_) = self.repository.get_user_by_email(&email).await? {
            return Err(DomainError::Conflict("Email already in use".to_string()));
        }

        let hashed_password = hash(password, DEFAULT_COST)
            .map_err(|e| DomainError::Infra(format!("Error hashing password: {:?}", e)))?;

        let user = User {
            id: Uuid::new_v4(),
            username,
            email,
            password_hash: hashed_password,
            created_at: Utc::now(),
            updated_at: None,
        };

        self.repository.create_user(&user).await?;
        Ok(user)
    }

    pub async fn login_user(&self, email: String, password: String) -> Result<User, DomainError> {
        if let Some(user) = self.repository.get_user_by_email(&email).await? {
            let is_valid = verify(password, &user.password_hash)
                .map_err(|e| DomainError::Infra(format!("Error verifying password: {:?}", e)))?;
            if is_valid {
                Ok(user)
            } else {
                Err(DomainError::Unauthorized("Invalid credentials".to_string()))
            }
        } else {
            Err(DomainError::NotFound("User not found".to_string()))
        }
    }

    pub async fn get_user(&self, user_id: Uuid) -> Result<User, DomainError> {
        if let Some(user) = self.repository.get_user_by_id(user_id).await? {
            Ok(user)
        } else {
            Err(DomainError::NotFound("User not found".to_string()))
        }
    }

    pub async fn update_user(
        &self,
        user_id: Uuid,
        new_username: Option<String>,
        new_password: Option<String>,
    ) -> Result<User, DomainError> {
        let mut user = self.get_user(user_id).await?;
        if let Some(u) = new_username {
            if !u.is_empty() {
                user.username = u;
            }
        }
        if let Some(p) = new_password {
            if !p.is_empty() {
                user.password_hash = hash(p, DEFAULT_COST)
                    .map_err(|e| DomainError::Infra(format!("Error hashing password: {:?}", e)))?;
            }
        }
        user.updated_at = Some(Utc::now());
        self.repository.update_user(&user).await?;
        Ok(user)
    }

    pub async fn delete_user(&self, user_id: Uuid) -> Result<(), DomainError> {
        self.repository.delete_user(user_id).await
    }

    pub async fn list_users(&self) -> Result<Vec<User>, DomainError> {
        self.repository.list_users().await
    }
}
