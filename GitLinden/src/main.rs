mod ui;

use iced::application;

use crate::ui::main_screen::MainScreen;

fn main() -> iced::Result {
    application(MainScreen::new, MainScreen::update, MainScreen::view).run()
}
