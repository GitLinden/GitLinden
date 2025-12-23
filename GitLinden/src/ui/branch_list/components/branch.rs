use iced::{
    Element,
    widget::{button, text},
};

#[derive(Debug, Clone)]
pub enum Message {
    Select(String),
    Checkout,
}

pub fn branch<'a>(name: String, selected: bool) -> Element<'a, Message> {
    button(text!("{} {name}", if selected { " ✔️" } else { "" }))
        .on_press(Message::Select(name))
        .into()
}
