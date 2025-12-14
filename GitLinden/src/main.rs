mod ui;

use crate::ui::main_screen::MainScreen;

fn main() -> iced::Result {
    iced::run(MainScreen::update, MainScreen::view)
}
