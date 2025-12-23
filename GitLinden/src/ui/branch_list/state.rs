use std::sync::Arc;

use git::model::repository::Repository;

pub struct BranchList {
    repository: Arc<Repository>,
    pub selected_branch: String,
}

impl BranchList {
    pub fn new(repository: Arc<Repository>) -> Self {
        Self {
            repository,
            selected_branch: String::new(),
        }
    }

    pub fn get_branches(&self) -> Vec<String> {
        self.repository.branches()
    }
}
