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

#[derive(Default)]
struct MenuApp {
    selected_option: Option<MenuOption>,
    current_view: View,
    tic_tac_toe: TicTacToe,
}

#[derive(Debug, Clone, Copy)]
enum View {
    Menu,
    Option1,
    Option2,
}

#[derive(Debug, Clone, Copy)]
enum MenuOption {
    Option1,
    Option2,
    Option3,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Menu(MenuMessage),
    TicTacToe(TicTacToeMessage),
}

#[derive(Debug, Clone, Copy)]
enum MenuMessage {
    Select(MenuOption),
    BackToMenu,
}

#[derive(Debug, Clone, Copy)]
enum TicTacToeMessage {
    MakeMove(usize, usize),
    CleanBoard,
}

impl Default for View {
    fn default() -> Self {
        View::Menu
    }
}

impl Default for TicTacToe {
    fn default() -> Self {
        TicTacToe {
            game_state: GameState::default(),
            game_over: false,
        }
    }
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

    fn clean_board(&mut self) {
        self.board = [[None; 3]; 3];
        self.current_player = Player::X;
    }
}

struct TicTacToe {
    game_state: GameState,
    game_over: bool,
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
                        self.current_view = View::Option1; // Go to Option 1 view (Single player)
                        self.tic_tac_toe.game_over = false;
                        self.tic_tac_toe.game_state.clean_board();
                    }
                    MenuOption::Option2 => {
                        self.current_view = View::Option2; // Go to Option 2 view
                    }
                    MenuOption::Option3 => {
                        std::process::exit(0); // Exit the application
                    }
                },
                MenuMessage::BackToMenu => {
                    self.current_view = View::Menu; // Navigate back to the main menu
                }
            },
            Message::TicTacToe(tic_tac_toe_message) => {
                // Update TicTacToe game state with mutable reference to game_state
                match tic_tac_toe_message {
                    TicTacToeMessage::MakeMove(row, col) => {
                        if !self.tic_tac_toe.game_over && self.tic_tac_toe.game_state.make_move(row, col) {
                            if let Some(player) = self.tic_tac_toe.game_state.check_win() {
                                println!("Player {:?} wins!", player);
                                self.tic_tac_toe.game_over = true;
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
            View::Option2 => self.view_option2()
        }
    }
}

impl MenuApp {
    fn view_menu(&self) -> Element<Message> {
        let button_option1 = Button::new(Text::new("Single player"))
            .on_press(Message::Menu(MenuMessage::Select(MenuOption::Option1)))
            .width(Length::Shrink);

        let button_option2 = Button::new(Text::new("Multiplayer"))
            .on_press(Message::Menu(MenuMessage::Select(MenuOption::Option2)))
            .width(Length::Shrink);

        let button_option3 = Button::new(Text::new("Exit"))
            .on_press(Message::Menu(MenuMessage::Select(MenuOption::Option3)))
            .width(Length::Shrink);

        let content = Column::new()
            .push(button_option1)
            .push(button_option2)
            .push(button_option3)
            .spacing(20)
            .padding(20)
            .align_items(iced::Alignment::Center);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn view_option2(&self) -> Element<Message> {
        let back_button = Button::new(Text::new("Back to Menu"))
            .on_press(Message::Menu(MenuMessage::BackToMenu))
            .width(Length::Shrink);

        let content = Column::new()
            .push(Text::new("This is Option 2 view"))
            .push(back_button)
            .spacing(20)
            .padding(20)
            .align_items(iced::Alignment::Center);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

impl TicTacToe {
    fn view(&self) -> Element<Message> {
        let mut column = Column::new();

        for (row_index, row_cells) in self.game_state.board.iter().enumerate() {
            let mut row_widget = Row::new();

            for (col, cell) in row_cells.iter().enumerate() {
                let text = match cell {
                    Some(Player::X) => "X",
                    Some(Player::O) => "O",
                    None => "",
                };

                let mut button: Button<Message> = Button::new(Text::new(text))
                    .width(Length::Fixed(50.0))
                    .height(Length::Fixed(50.0));

                // Disable button if the game is over
                if !self.game_over {
                    button = button.on_press(Message::TicTacToe(TicTacToeMessage::MakeMove(row_index, col)));
                }

                row_widget = row_widget.push(
                    Button::new(Text::new(text))
                        .on_press(Message::TicTacToe(TicTacToeMessage::MakeMove(
                            row_index, col,
                        )))
                        .width(Length::Fixed(50.0))
                        .height(Length::Fixed(50.0)),
                );
            }

            column = column.push(row_widget);
        }

        let clear_button = Button::new(Text::new("Clear board"))
            .on_press(Message::TicTacToe(TicTacToeMessage::CleanBoard))
            .width(Length::Shrink);

        column = column.push(clear_button);

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn main() -> iced::Result {
    MenuApp::run(Settings::default())
}

