use super::symbol::Symbol;

use std::fmt;
use std::fmt::{Display, Formatter};

/// TicTacToe game logic
#[derive(Debug)]
pub struct TicTacToe {
    /// The current boards state, 
    /// represented as a 2D array
    board: [[Symbol; 3]; 3],
}

impl TicTacToe {
    /// Create a new TicTacToe game with an empty board
    /// 
    /// # Returns
    /// 
    /// New TicTacToe game instance
    pub fn new() -> Self {
        Self {
            board: [
                [Symbol::EMPTY, Symbol::EMPTY, Symbol::EMPTY],
                [Symbol::EMPTY, Symbol::EMPTY, Symbol::EMPTY],
                [Symbol::EMPTY, Symbol::EMPTY, Symbol::EMPTY],
            ],
        }
    }

    /// Sets a given cell to a given symbol
    /// 
    /// # Arguments
    /// 
    /// * `cell` - The cell to set in the form (row, col)
    /// * `sym` - The symbol to set at the given cell
    pub fn set(&mut self, cell: (usize, usize), sym: Symbol) {
        self.board[cell.0][cell.1] = sym;
    }

    /// Decide if a move can be made on the given space
    /// 
    /// # Arguments
    /// 
    /// * `cell` - The cell to check in the form (row, col)
    pub fn legal_move(&self, cell: (usize, usize)) -> bool {
        self.board[cell.0][cell.1] == Symbol::EMPTY
    }

    /// Check if the board has no EMPTY spaces left
    /// 
    /// # Returns
    /// 
    /// True if the board is full (all cells occupied by
    /// either X or O), false otherwise
    pub fn board_full(&self) -> bool {
        for row in &self.board {
            for sym in row {
                if *sym == Symbol::EMPTY {
                    return false;
                }
            }
        }
        true
    }

    /// Check if either X's or O's has won. First,
    /// each row is checked. Then each column, then both diagonals
    /// 
    /// # Returns
    /// 
    /// True if a winner was found, false otherwise
    pub fn check_win(&self) -> bool {
        for row in &self.board {
            if row[0] == row[1] && row[1] == row[2] && row[0] != Symbol::EMPTY {
                return true;
            }
        }

        for col in 0..=2 {
            if self.board[0][col] == self.board[1][col]
                && self.board[1][col] == self.board[2][col]
                && self.board[0][col] != Symbol::EMPTY
            {
                return true;
            }
        }

        if self.board[1][1] != Symbol::EMPTY {
            return (self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2])
                || (self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0]);
        }
        false
    }

    /// Transforms the 2D board state into a flattened string
    /// 
    /// # Returns
    /// 
    /// String with each row serialized after the other,
    /// Creating a 1D string from a 2D array
    pub fn flatten(&self) -> String {
        let mut flattened = String::new();

        for row in &self.board {
            for col in row {
                flattened.push(col.as_char());
            }
        }
        flattened
    }

    /// Generates a list of possible moves that either player
    /// could make, given the current board state
    /// 
    /// # Returns
    /// 
    /// List of tuples representing each cell that's 
    /// occupied by the EMPTY symbol
    pub fn possible_moves(&self) -> Vec<(usize, usize)> {
        let mut moves: Vec<(usize, usize)> = Vec::new();

        for row in 0..=2 {
            for col in 0..=2 {
                if self.board[row][col] == Symbol::EMPTY {
                    moves.push((row as usize, col as usize));
                }
            }
        }
        moves
    }
}

impl Display for TicTacToe {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut row_count = 0;

        for row in &self.board {
            writeln!(f, " {} | {} | {}", row[0], row[1], row[2])?;

            row_count += 1;
            if row_count < 3 {
                writeln!(f, "-----------")?;
            }
        }
        Ok(())
    }
}
