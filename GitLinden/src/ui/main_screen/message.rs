use crate::ui::branch_list::BranchListMessage;

#[derive(Debug, Clone)]
pub enum Message {
    BranchList(BranchListMessage),
}
