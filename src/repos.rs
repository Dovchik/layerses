use crate::entities::User;
use async_trait::async_trait;
use mockall::automock;
use sqlx::PgPool;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("data store error")]
    RepoError(#[from] sqlx::Error),
}

// ! Important should appear before async_trait https://docs.rs/mockall/latest/mockall/#async-traits
#[automock]
#[async_trait]
pub trait IUserRepo {
    async fn insert(&self, user: User) -> Result<(), Error>;
    async fn list(&self) -> Result<Vec<User>, Error>;
}

pub struct UserRepo {
    pool: PgPool,
}

impl UserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IUserRepo for UserRepo {
    async fn insert(&self, user: User) -> Result<(), Error> {
        let mut conn = self.pool.acquire().await?;
        sqlx::query!(
            r#"
            INSERT INTO users (username)
            VALUES ($1)
            "#,
            user.username()
        )
        .execute(&mut conn)
        .await?;
        Ok(())
    }

    async fn list(&self) -> Result<Vec<User>, Error> {
        let mut conn = self.pool.acquire().await?;
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM users
            "#
        )
        .fetch_all(&mut conn)
        .await?;
        Ok(users)
    }
}
