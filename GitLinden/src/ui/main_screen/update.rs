use crate::ui::main_screen::message::Message;
use crate::ui::main_screen::state::MainScreen;
use iced::Task;

impl MainScreen {
    pub(crate) fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::BranchList(_) => Task::none(),
        }
    }
}
