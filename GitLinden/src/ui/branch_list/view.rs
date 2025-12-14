use crate::ui::branch_list::message::Message;
use crate::ui::branch_list::state::BranchList;
use iced::widget::{column, scrollable, text};
use iced::{Center, Element, Length};

impl BranchList {
    pub(crate) fn view(&self) -> Element<'_, Message> {
        column![
            text("BranchList").size(50),
            scrollable(
                column((1..10_000).map(|i| { text!("Item {i}").into() }))
                    .width(Length::Fixed(150f32))
            )
        ]
        .padding(20)
        .align_x(Center)
        .into()
    }
}
