use iced::{
    Element,
    widget::{mouse_area, text},
};

#[derive(Debug, Clone)]
pub enum Message {
    Select(String),
    Checkout(String),
}

pub fn branch<'a>(name: String, selected: bool) -> Element<'a, Message> {
    mouse_area(text!("{} {name}", if selected { " ✔️" } else { "" }))
        .on_press(Message::Select(name.clone()))
        .on_double_click(Message::Checkout(name))
        .into()
}
