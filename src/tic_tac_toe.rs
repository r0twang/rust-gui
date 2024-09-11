use iced::{Element, Length};
use iced::widget::{Button, Column, Container, Row, Text};
use iced::Length::Fill;

use crate::game_state::{GameState, Player};
use crate::messages::{Message, MenuMessage, TicTacToeMessage};
use crate::views::View;

#[derive(Default)]
pub struct TicTacToe {
    pub game_state: GameState,
    pub game_over: bool,
}

impl TicTacToe {
    pub fn view(&self) -> Element<Message> {
        let mut column = Column::new()
            .align_items(iced::Alignment::Center);

        for (row_index, row_cells) in self.game_state.board.iter().enumerate() {
            let mut row_widget = Row::new()
                .align_items(iced::Alignment::Center);

            for (col, cell) in row_cells.iter().enumerate() {
                let text = match cell {
                    Some(Player::X) => "X",
                    Some(Player::O) => "O",
                    None => "",
                };

                row_widget = row_widget.push(
                    Button::new(Text::new(text))
                        .on_press(Message::TicTacToe(TicTacToeMessage::MakeMove(row_index, col)))
                        .width(Length::Fixed(50.0))
                        .height(Length::Fixed(50.0)),
                );
            }

            column = column.push(row_widget);
        }

        let clear_button = Button::new(Text::new("Clear board"))
            .on_press(Message::TicTacToe(TicTacToeMessage::CleanBoard))
            .width(Length::Shrink);

        let back_button = Button::new(Text::new("Back to Menu"))
            .on_press(Message::Menu(MenuMessage::BackToMenu))
            .width(Length::Shrink);

        column = column.push(clear_button);
        column = column.push(back_button);

        Container::new(column)
            .width(Fill)
            .height(Fill)
            .center_x()
            .center_y()
            .into()
    }
}
