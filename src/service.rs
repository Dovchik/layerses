use crate::{
    entities::User,
    repos::{self, IUserRepo},
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("User Name not allowed")]
    NotAllowed,

    #[error(transparent)]
    RepositoryError(#[from] repos::Error),
}

pub struct UserService {
    user_repo: Box<dyn IUserRepo>,
}

impl UserService {
    pub fn new(user_repo: Box<dyn IUserRepo>) -> Self {
        Self { user_repo }
    }

    pub async fn inser_new(&self, username: &str) -> Result<(), Error> {
        if username == "LOH" {
            return Err(Error::NotAllowed);
        }

        self.user_repo
            .insert(User {
                id: Default::default(),
                username: username.to_owned(),
            })
            .await?;

        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<User>, Error> {
        let users = self.user_repo.list().await?;
        Ok(users)
    }
}

#[cfg(test)]
mod tests {
    use super::UserService;
    use crate::repos::*;

    #[tokio::test]
    async fn should_insert_user() {
        // arrange
        let mut mock = MockIUserRepo::new();
        mock.expect_insert().times(1).returning(|_| Ok(()));
        let user_service = UserService::new(Box::new(mock));

        // act
        let result = user_service.inser_new("HOMER").await;

        // assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_return_error_if_name_is_loh() {
        // arrange
        let mut mock = MockIUserRepo::new();
        mock.expect_insert().times(0).returning(|_| Ok(()));
        let user_service = UserService::new(Box::new(mock));

        // act
        let result = user_service.inser_new("LOH").await;

        // assert
        assert!(matches!(result.unwrap_err(), super::Error::NotAllowed));
    }
}
