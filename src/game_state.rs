use std::cmp::PartialEq;

#[derive(Clone)]
pub struct GameState {
    pub board: [[Option<Player>; 3]; 3],
    pub current_player: Player,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    X,
    O,
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
    pub fn make_move(&mut self, row: usize, col: usize) -> bool {
        if self.board[row][col].is_none() {
            self.board[row][col] = Some(self.current_player);
            self.current_player = match self.current_player {
                Player::X => Player::O,
                Player::O => Player::X,
            };
            true
        } else {
            false
        }
    }

    pub fn best_move(&self) -> Option<(usize, usize)> {
        let mut best_score = i32::MIN;
        let mut best_move = None;

        for row in 0..3 {
            for col in 0..3 {
                if self.board[row][col].is_none() {
                    let mut board_copy = self.clone();
                    board_copy.board[row][col] = Some(Player::O);
                    let score = board_copy.minimax(0, false);
                    if score > best_score {
                        best_score = score;
                        best_move = Some((row, col));
                    }
                }
            }
        }

        best_move
    }

    pub fn minimax(&self, depth: i32, is_maximizing: bool) -> i32 {
        if let Some(winner) = self.check_win() {
            return match winner {
                Player::X => -10 + depth,
                Player::O => 10 - depth,
            };
        }

        if self.is_draw() {
            return 0;
        }

        if is_maximizing {
            let mut best_score = i32::MIN;
            for row in 0..3 {
                for col in 0..3 {
                    if self.board[row][col].is_none() {
                        let mut board_copy = self.clone();
                        board_copy.board[row][col] = Some(Player::O);
                        let score = board_copy.minimax(depth + 1, false);
                        best_score = best_score.max(score);
                    }
                }
            }
            best_score
        } else {
            let mut best_score = i32::MAX;
            for row in 0..3 {
                for col in 0..3 {
                    if self.board[row][col].is_none() {
                        let mut board_copy = self.clone();
                        board_copy.board[row][col] = Some(Player::X);
                        let score = board_copy.minimax(depth + 1, true);
                        best_score = best_score.min(score);
                    }
                }
            }
            best_score
        }
    }

    pub fn is_draw(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(|&cell| cell.is_some()))
    }

    pub fn check_win(&self) -> Option<Player> {
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

    pub fn clean_board(&mut self) {
        self.board = [[None; 3]; 3];
        self.current_player = Player::X;
    }
}
