use std::{path::Path, sync::Arc};

use git::{
    business::{module_assembly::GitModule, repository::RepositoryManager},
    model::repository::Repository,
};

use crate::ui::branch_list::BranchList;

use shaku::HasComponent;

pub(crate) struct MainScreen {
    repository_manager: Arc<dyn RepositoryManager>,
    pub(crate) branch_list: BranchList,

    pub(crate) repository: Arc<Repository>,
}

impl MainScreen {
    pub fn new() -> Self {
        let git_module = GitModule::builder().build();
        let repository_manager: Arc<dyn RepositoryManager> = git_module.resolve();
        let path = Path::new(".");
        let repository = Arc::new(repository_manager.get_repository(path).unwrap());

        Self {
            repository_manager,
            branch_list: BranchList::new(repository.clone()),
            repository,
        }
    }
}
