use crate::ui::branch_list::message::Message;
use crate::ui::branch_list::state::BranchList;
use iced::widget::{column, scrollable, text};
use iced::{Center, Element};

impl BranchList {
    pub(crate) fn view(&self) -> Element<'_, Message> {
        column![
            text("BranchList").size(50),
            scrollable(column(
                self.get_branches()
                    .iter()
                    .map(|branch_name| { text!("{}", branch_name).into() })
            ),),
        ]
        .padding(20)
        .align_x(Center)
        .into()
    }
}
