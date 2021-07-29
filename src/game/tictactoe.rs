use super::symbol::Symbol;
use std::fmt::{Display, Formatter};
use std::{fmt, usize};

#[derive(Debug)]
pub struct TicTacToe {
    board: [Symbol; 9]
}

impl TicTacToe {
    pub fn new() -> Self {
        Self {
            board: [Symbol::EMPTY; 9]
        }
    }

    /// Resets the game to it's initial state
    pub fn reset(&mut self) {
        self.board = [Symbol::EMPTY; 9];
    }

    /// Forces the game to a state based on the given 'flat_state'
    pub fn from_state(&mut self, flat_state: &String) {
        for (i, c) in flat_state.chars().enumerate() {
            self.board[i] = Symbol::from(c);
        }
    }

    /// Places the specified piece at index `action` on the board
    ///
    /// # Returns
    ///
    /// `Ok(())` if `action` is a legal move, `Err(msg)` otherwise
    pub fn place_piece(&mut self, piece: Symbol, action: u8) -> Result<(), String> {
        if self.board[action as usize] != Symbol::EMPTY {
            return Err(format!("'{}' is not an empty space!", action));
        }
        self.board[action as usize] = piece;

        Ok(())
    }

    /// # Returns
    /// 
    /// A list of the next possible flattened states the game could have
    pub fn next_states(&self, piece: Symbol) -> Vec<String> {
        self.legal_moves().iter().map(|action| {
            let mut board = self.board.clone();

            board[*action as usize] = piece;
            board.iter().map(
                |cell| cell.as_char()
            ).collect()
        }).collect()
    }

    /// Determines if the specified piece has won the game
    ///
    /// # Returns
    ///
    /// `true` if the piece meets winning criteria, `false` otherwise
    pub fn is_winner(&self, piece: Symbol) -> bool {
        let win_conds: [[u8; 3]; 8] = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6]
        ];

        for win_cond in win_conds.iter() {
            if win_cond.iter().all(|&c| self.board[c as usize] == piece) {
                return true;
            }
        }
        return false;
    }

    /// # Returns
    /// 
    /// 'true' if the game has ended in a draw, 'false' otherwise
    pub fn is_draw(&self) -> bool {
        for cell in self.board.iter() {
            if *cell == Symbol::EMPTY {
                return false
            }
        }
        true
    }

    /// # Returns
    ///
    /// A flattened string representation of `board`
    pub fn flat(&self) -> String {
        let mut flattened = String::new();

        for piece in self.board.iter() {
            flattened.push(piece.as_char());
        }
        flattened
    }

    /// # Returns
    ///
    /// A list of all legal moves that can be made
    fn legal_moves(&self) -> Vec<u8> {
        let mut allowed_actions: Vec<u8> = Vec::with_capacity(9);

        for (i, piece) in self.board.iter().enumerate() {
            if piece == &Symbol::EMPTY {
                allowed_actions.push(i as u8);
            }
        }
        allowed_actions
    }
}

impl Display for TicTacToe {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut str_board = String::new();

        for (i, cell) in self.board.iter().enumerate() {
            if (i != 0) && (i % 3 == 0) {
                str_board.push_str("\n-----------\n");
            }
            str_board.push_str(format!(" {} ", cell).as_str());

            if (i + 1) % 3 != 0 {
                str_board.push('|');
            }
        }
        write!(f, "{}", str_board)
    }
}
