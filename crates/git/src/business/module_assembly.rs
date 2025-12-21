use shaku::module;

use crate::business::branch_manager::Git2BranchManagerImpl;
use crate::business::repo_manager::Git2RepoManagerImpl;

module! {
    pub GitModule {
        components = [
            Git2BranchManagerImpl,
            Git2RepoManagerImpl,
        ],
        providers = [],
    }
}
