use crate::ui::main_screen::message::Message;
use crate::ui::main_screen::state::MainScreen;
use iced::Element;
use iced::widget::{row, space};

impl MainScreen {
    pub(crate) fn view(&self) -> Element<'_, Message> {
        row![
            self.branch_list.view().map(Message::BranchList),
            space::horizontal(),
        ]
        .into()
    }
}
