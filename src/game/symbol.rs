use std::fmt;
use std::fmt::{Display, Formatter};

/// Possible symbols for any given cell
/// of a TicTacToe board, can be represented
/// as bytes or ASCII characters
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Symbol {
    X = b'X',
    O = b'O',
    EMPTY = b' ',
}

impl Symbol {
    /// Convert the symbol to an ASCII character
    /// 
    /// # Returns
    /// 
    /// The symbol represented as a byte, then
    /// converted to an ASCII character
    pub fn as_char(&self) -> char {
        *self as u8 as char
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let sym = self.as_char();
        write!(f, "{}", sym)
    }
}
