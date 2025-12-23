use shaku::module;

use crate::business::{
    branch_manager::Git2BranchManagerImpl, repository::implementation::Git2RepoManagerImpl,
};

module! {
    pub GitModule {
        components = [
            Git2BranchManagerImpl,
            Git2RepoManagerImpl,
        ],
        providers = [],
    }
}
