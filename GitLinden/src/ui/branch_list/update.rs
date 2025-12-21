use crate::ui::branch_list::message::Message;
use crate::ui::branch_list::state::BranchList;
use crate::ui::main_screen::MainMessage;
use iced::Task;

impl BranchList {
    pub fn update(&mut self, message: Message) -> Task<MainMessage> {
        match message {}
    }
}
