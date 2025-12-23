use crate::ui::branch_list::message::Message;
use crate::ui::branch_list::state::BranchList;
use crate::ui::main_screen::MainMessage;
use iced::Task;

impl BranchList {
    pub fn update(&mut self, message: Message) -> Task<MainMessage> {
        match message {
            Message::Branch(branc_message) => match branc_message {
                super::components::BranchMessage::Select(name) => {
                    self.selected_branch = name;
                    Task::none()
                }
                super::components::BranchMessage::Checkout => todo!(),
            },
        }
    }
}
