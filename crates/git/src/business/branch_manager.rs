use crate::model::branch::Branch;

pub trait BranchManager {
    fn get_branches(&self) -> Vec<Branch>;
}

struct Git2BranchManagerImpl {

}

impl BranchManager for Git2BranchManagerImpl {
    fn get_branches(&self) -> Vec<Branch> {
        todo!()
    }
}