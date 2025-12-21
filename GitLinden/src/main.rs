mod ui;

use iced::application;

use crate::ui::main_screen::MainScreen;

fn main() -> iced::Result {
    application(MainScreen::default, MainScreen::update, MainScreen::view).run()
}
