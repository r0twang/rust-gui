use iced::{Application, Command, Element, Length, Settings};
use iced::widget::{Button, Column, Container, Row, Text};

struct GameState {
    board: [[Option<Player>; 3]; 3],
    current_player: Player,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Player {
    X,
    O,
}

#[derive(Clone, Debug)]
enum Message {
    MakeMove(usize, usize),
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            board: [[None; 3]; 3],
            current_player: Player::X,
        }
    }
}

impl GameState {
    fn make_move(&mut self, row: usize, col: usize) -> bool {
        if self.board[row][col].is_none() {
            self.board[row][col] = Some(self.current_player.clone());
            self.current_player = match self.current_player {
                Player::X => Player::O,
                Player::O => Player::X,
            };
            true
        } else {
            false
        }
    }

    fn check_win(&self) -> Option<Player> {
        // Check rows for a win
        for row in self.board.iter() {
            if let [Some(player1), Some(player2), Some(player3)] = row {
                if player1 == player2 && player2 == player3 {
                    return Some(*player1);
                }
            }
        }

        // Check columns for a win
        for col in 0..3 {
            if let (Some(player1), Some(player2), Some(player3)) = (
                self.board[0][col],
                self.board[1][col],
                self.board[2][col],
            ) {
                if player1 == player2 && player2 == player3 {
                    return Some(player1);
                }
            }
        }

        // Check diagonals for a win
        if let (Some(player1), Some(player2), Some(player3)) = (
            self.board[0][0],
            self.board[1][1],
            self.board[2][2],
        ) {
            if player1 == player2 && player2 == player3 {
                return Some(player1);
            }
        }

        if let (Some(player1), Some(player2), Some(player3)) = (
            self.board[0][2],
            self.board[1][1],
            self.board[2][0],
        ) {
            if player1 == player2 && player2 == player3 {
                return Some(player1);
            }
        }

        None
    }
}

struct TicTacToe {
    game_state: GameState,
}

impl Application for TicTacToe {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self { game_state: Default::default() }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Tic Tac Toe")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::MakeMove(row, col) => {
                if self.game_state.make_move(row, col) {
                    if let Some(player) = self.game_state.check_win() {
                        println!("Player {:?} wins!", player);
                    }
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let mut column = Column::new();

        for (row_index, row_cells) in self.game_state.board.iter().enumerate() {
            let mut row_widget = Row::new();

            for (col, cell) in row_cells.iter().enumerate() {
                let text = match cell {
                    Some(Player::X) => "X",
                    Some(Player::O) => "O",
                    None => "",
                };

                row_widget = row_widget.push(
                    Button::new(Text::new(text))
                        .on_press(Message::MakeMove(row_index, col))
                        .width(Length::Fixed(50.0))
                        .height(Length::Fixed(50.0)),
                );
            }

            column = column.push(row_widget);
        }

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn main() -> iced::Result {
    TicTacToe::run(Settings::default())
    
}
