use springtime_di::{Component, component_alias, injectable};

#[injectable]
pub trait RepoManager {
}

#[derive(Component)]
struct Git2RepoManagerImpl {
}

#[component_alias]
impl RepoManager for Git2RepoManagerImpl {
    
}