use std::sync::Arc;

use git::model::repository::Repository;

pub(crate) struct BranchList {
    repository: Arc<Repository>,
}

impl BranchList {
    pub fn new(repository: Arc<Repository>) -> Self {
        Self { repository }
    }

    pub fn get_branches(&self) -> Vec<String> {
        self.repository.branches()
    }
}
