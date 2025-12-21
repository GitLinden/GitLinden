use git::business::repo_manager::RepoManager;
use springtime_di::{factory::ComponentFactory, instance_provider::{ComponentInstancePtr, TypedComponentInstanceProvider}};

pub(crate) struct BranchList {
    repo_manager: ComponentInstancePtr<dyn RepoManager + Send + Sync>
}

impl BranchList {
    pub(crate) fn new(factory: &mut ComponentFactory) -> Self {
        let repo_manager = factory.primary_instance_typed::<dyn RepoManager + Send + Sync>().expect("");


        Self {
            repo_manager
        }
    }
}