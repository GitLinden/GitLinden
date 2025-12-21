use std::sync::Arc;

use git::business::{module_assembly::GitModule, repo_manager::RepoManager};
use shaku::HasComponent;

pub(crate) struct BranchList {
    repo_manager: Arc<dyn RepoManager>,
}

impl Default for BranchList {
    fn default() -> Self {
        let git_module = GitModule::builder().build();

        Self {
            repo_manager: git_module.resolve(),
        }
    }
}