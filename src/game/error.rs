use self::TicTacToeError::*;
use super::board::Square;
use std::fmt;

#[derive(Debug)]
pub enum TicTacToeError {
    StdInFailure(StdInFailure),
    StringIsNotASquare(StringIsNotASquare),
    SquareOccupied(SquareOccupied)
}

#[derive(Debug)]
pub struct StdInFailure {
    pub e: std::io::Error
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StringIsNotASquare {
    pub string: String
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SquareOccupied {
    pub sq: Square
}

impl std::error::Error for TicTacToeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            StdInFailure(StdInFailure{ref e}) => Some(e),
            StringIsNotASquare(_) => None,
            SquareOccupied(_) => None,
        }
    }
}

impl fmt::Display for TicTacToeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match &self {
            StdInFailure(_) => 
                "System error reading input".to_owned(),
            StringIsNotASquare(StringIsNotASquare{string}) => 
                format!("Squares are numbered 1-9. {} is invalid.", string.clone()),
            SquareOccupied(SquareOccupied{sq}) => 
                format!("Location {} is already taken.", sq),
        };
        write!(f, "{}", s)
    }
}

impl From<StdInFailure> for TicTacToeError {
    fn from(e: StdInFailure) -> TicTacToeError {
        StdInFailure(e)
    }
}

impl From<StringIsNotASquare> for TicTacToeError {
    fn from(e: StringIsNotASquare) -> TicTacToeError {
        StringIsNotASquare(e)
    }
}

impl From<SquareOccupied> for TicTacToeError {
    fn from(e: SquareOccupied) -> TicTacToeError {
        SquareOccupied(e)
    }
}
