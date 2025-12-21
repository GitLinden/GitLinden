use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Repository not found at path {0:?}")]
    NotFound(PathBuf),
}
