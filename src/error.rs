use crate::repos;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LayersError {
    #[error("data store error")]
    RepoError(#[from] repos::Error),
}
