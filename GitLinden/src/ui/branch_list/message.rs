use crate::ui::branch_list::components::BranchMessage;

#[derive(Debug, Clone)]
pub enum Message {
    Branch(BranchMessage),
}
