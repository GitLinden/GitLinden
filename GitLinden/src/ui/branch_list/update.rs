use crate::ui::branch_list::message::Message;
use crate::ui::branch_list::state::BranchList;
use iced::Task;

impl BranchList {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {}
    }
}
