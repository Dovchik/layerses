use repos::UserRepo;
use service::UserService;
use sqlx::postgres::PgPoolOptions;
use thiserror::Error;

mod entities;
mod error;
mod repos;
mod service;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error")]
    DatabaseError(#[from] sqlx::Error),

    #[error("repository error")]
    RepositoryError(#[from] repos::Error),

    #[error("service error")]
    ServiceError(#[from] service::Error),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://test:tester@localhost/layerses")
        .await?;
    let user_repo = UserRepo::new(pool.clone());
    let user_service = UserService::new(Box::new(user_repo));
    user_service.inser_new("HOMER").await?;
    dbg!(user_service.list().await?);
    Ok(())
}
