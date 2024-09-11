mod game_state;
mod tic_tac_toe;
mod menu_app;
mod messages;
mod views;

use menu_app::MenuApp;
use iced::{Application, Settings};

fn main() -> iced::Result {
    MenuApp::run(Settings::default())
}