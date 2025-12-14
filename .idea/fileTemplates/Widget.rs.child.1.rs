use crate::ui::${snake_widget_name}::message::Message;
use crate::ui::${snake_widget_name}::state::$pascal_widget_name;
use iced::widget::{column, text};
use iced::{Center, Element};

impl $pascal_widget_name {
    pub(crate) fn view(&self) -> Element<'_, Message> {
        column![
            text("$pascal_widget_name").size(50),
        ]
        .padding(20)
        .align_x(Center)
        .into()
    }
}
