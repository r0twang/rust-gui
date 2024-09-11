use crate::views::MenuOption;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Menu(MenuMessage),
    TicTacToe(TicTacToeMessage),
}

#[derive(Debug, Clone, Copy)]
pub enum MenuMessage {
    Select(MenuOption),
    BackToMenu,
}

#[derive(Debug, Clone, Copy)]
pub enum TicTacToeMessage {
    MakeMove(usize, usize),
    CleanBoard,
}
