use shaku::{Component, Interface};

pub trait RepoManager: Interface {
}

#[derive(Component)]
#[shaku(interface = RepoManager)]
pub(crate) struct Git2RepoManagerImpl {
}

impl RepoManager for Git2RepoManagerImpl {
    
}
