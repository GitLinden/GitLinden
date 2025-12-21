use std::path::Path;

use shaku::Component;

use crate::{
    business::repository::{manager::RepositoryManager, error::RepositoryError},
    model::repository::Repository,
};

#[derive(Component)]
#[shaku(interface = RepositoryManager)]
pub(crate) struct Git2RepoManagerImpl {}

impl RepositoryManager for Git2RepoManagerImpl {
    fn get_repository(&self, path: &Path) -> Result<Repository, RepositoryError> {
        Repository::open(path).map_err(|_| RepositoryError::NotFound(path.to_path_buf()))
    }
}
