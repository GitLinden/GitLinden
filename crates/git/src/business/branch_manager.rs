use shaku::{Component, Interface};

use crate::model::branch::Branch;

pub trait BranchManager: Interface {
    fn get_branches(&self) -> Vec<Branch>;
}

#[derive(Component)]
#[shaku(interface = BranchManager)]
pub(crate) struct Git2BranchManagerImpl {

}

impl BranchManager for Git2BranchManagerImpl {
    fn get_branches(&self) -> Vec<Branch> {
        todo!()
    }
}