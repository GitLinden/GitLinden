mod ui;

use iced::{application, window::run};
use springtime_di::factory::ComponentFactoryBuilder;

use crate::ui::main_screen::{self, MainScreen};

fn main() -> iced::Result {
    application(MainScreen::new, MainScreen::update, MainScreen::view).run()
}
