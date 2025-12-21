use std::{path::Path, sync::Arc};

use git::business::{module_assembly::GitModule, repository::{RepositoryManager}};
use shaku::HasComponent;

pub(crate) struct BranchList {
    repo_manager: Arc<dyn RepositoryManager>,
}

impl BranchList {
    pub fn get_branches(&self) -> Vec<String> {
        let path = Path::new(".");
        let Ok(repository) = self.repo_manager.get_repository(path) else {
            return Vec::new();
        };

        repository.branches()
    }
}

impl Default for BranchList {
    fn default() -> Self {
        let git_module = GitModule::builder().build();

        Self {
            repo_manager: git_module.resolve(),
        }
    }
}