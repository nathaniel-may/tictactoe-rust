use super::board::Square;

pub enum TicTacToeError {
    StdInFailure(StdInFailure),
    StringIsNotASquare(StringIsNotASquare),
    SquareOccupied(SquareOccupied)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct StdInFailure {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StringIsNotASquare {
    pub string: String
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SquareOccupied {
    pub sq: Square
}

impl From<&StdInFailure> for TicTacToeError {
    fn from(e: &StdInFailure) -> TicTacToeError {
        TicTacToeError::StdInFailure(*e)
    }
}

impl From<&StringIsNotASquare> for TicTacToeError {
    fn from(e: &StringIsNotASquare) -> TicTacToeError {
        TicTacToeError::StringIsNotASquare(e.clone())
    }
}

impl From<&SquareOccupied> for TicTacToeError {
    fn from(e: &SquareOccupied) -> TicTacToeError {
        TicTacToeError::SquareOccupied(*e)
    }
}

// TODO good idea? bad idea?
pub fn into_tictactoe_result<A, B: Into<TicTacToeError>>(r: Result<A, B>) -> Result<A, TicTacToeError> {
    r.map_err(|e| e.into())
}
