use crate::ui::branch_list::components::branch;
use crate::ui::branch_list::message::Message;
use crate::ui::branch_list::state::BranchList;
use iced::widget::{column, mouse_area, scrollable, text};
use iced::{Center, Element};

impl BranchList {
    pub(crate) fn view(&self) -> Element<'_, Message> {
        column![
            text("BranchList").size(50),
            scrollable(column(self.get_branches().into_iter().map(|branch_name| {
                let selected = branch_name == self.selected_branch;
                branch(branch_name, selected).map(Message::Branch)
            })),),
            mouse_area(text("coucou"))
        ]
        .padding(20)
        .align_x(Center)
        .into()
    }
}
