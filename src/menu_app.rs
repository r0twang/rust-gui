use iced::{Application, Command, Element, Length};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Button, Column, Container, Text};
use iced::widget::image;
use iced::Length::{Fill, Shrink};

use crate::tic_tac_toe::TicTacToe;
use crate::messages::{Message, MenuMessage, TicTacToeMessage};
use crate::views::{MenuOption, View};

#[derive(Default)]
pub struct MenuApp {
    pub current_view: View,
    pub tic_tac_toe: TicTacToe,
}

impl Application for MenuApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Tic Tac Toe")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Menu(menu_message) => match menu_message {
                MenuMessage::Select(option) => match option {
                    MenuOption::Option1 => {
                        self.current_view = View::Option1;
                    }
                    MenuOption::Option2 => {
                        self.current_view = View::Option2;
                        self.tic_tac_toe.game_over = false;
                        self.tic_tac_toe.game_state.clean_board();
                    }
                    MenuOption::Option3 => {
                        std::process::exit(0);
                    }
                },
                MenuMessage::BackToMenu => {
                    self.current_view = View::Menu;
                    self.tic_tac_toe.game_state.clean_board();
                }
            },
            Message::TicTacToe(tic_tac_toe_message) => {
                match tic_tac_toe_message {
                    TicTacToeMessage::MakeMove(row, col) => {
                        if !self.tic_tac_toe.game_over && self.tic_tac_toe.game_state.make_move(row, col) {
                            if let Some(player) = self.tic_tac_toe.game_state.check_win() {
                                println!("Player {:?} wins!", player);
                                self.tic_tac_toe.game_over = true;
                            } else if self.current_view == View::Option1 {
                                if let Some((ai_row, ai_col)) = self.tic_tac_toe.game_state.best_move() {
                                    self.tic_tac_toe.game_state.make_move(ai_row, ai_col);
                                    if let Some(player) = self.tic_tac_toe.game_state.check_win() {
                                        println!("Player {:?} wins!", player);
                                        self.tic_tac_toe.game_over = true;
                                    }
                                }
                            }
                        }
                    }
                    TicTacToeMessage::CleanBoard => {
                        self.tic_tac_toe.game_state.clean_board();
                        self.tic_tac_toe.game_over = false;
                    }
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        match self.current_view {
            View::Menu => self.view_menu(),
            View::Option1 => self.tic_tac_toe.view(),
            View::Option2 => self.tic_tac_toe.view(),
        }
    }
}

impl MenuApp {
    pub fn view_menu(&self) -> Element<Message> {
        let button_option1 = Button::new(Text::new("Single player"))
            .on_press(Message::Menu(MenuMessage::Select(MenuOption::Option1)))
            .width(Length::Shrink);

        let button_option2 = Button::new(Text::new("Multiplayer"))
            .on_press(Message::Menu(MenuMessage::Select(MenuOption::Option2)))
            .width(Length::Shrink);

        let button_option3 = Button::new(Text::new("Exit"))
            .on_press(Message::Menu(MenuMessage::Select(MenuOption::Option3)))
            .width(Length::Shrink);

        let logo_handle = image::Handle::from_path("resources/ttt.bmp");
        let logo = image::Image::new(logo_handle)
            .width(Length::Shrink)
            .height(Length::Shrink);

        let signature = Text::new("2024 by rotwang")
            .horizontal_alignment(Horizontal::Left)
            .vertical_alignment(Vertical::Bottom);

        let menu_content = Column::new()
            .push(logo)
            .push(button_option1)
            .push(button_option2)
            .push(button_option3)
            .spacing(20)
            .padding(20)
            .align_items(iced::Alignment::Center);

        let main_menu = Container::new(menu_content)
            .width(Shrink)
            .center_x()
            .center_y();

        let signature_container = Container::new(signature)
            .width(Fill)
            .padding(10)
            .align_x(Horizontal::Left)
            .align_y(Vertical::Bottom);

        let layout = Column::new()
            .push(main_menu)
            .push(signature_container)
            .height(Length::Fill)
            .align_items(iced::Alignment::Center);

        Container::new(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
