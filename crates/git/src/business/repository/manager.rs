use std::path::Path;

use shaku::Interface;

use crate::{business::repository::error::RepositoryError, model::repository::Repository};

pub trait RepositoryManager: Interface {
    fn get_repository(&self, path: &Path) -> Result<Repository, RepositoryError>;
}
