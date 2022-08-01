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

// TODO impl this
// impl std::error::Error for TicTacToeError {
//     fn source(&self) -> Option<&(dyn error::Error + 'static)> {
//         match *self {
//             DoubleError::EmptyVec => None,
//             // The cause is the underlying implementation error type. Is implicitly
//             // cast to the trait object `&error::Error`. This works because the
//             // underlying type already implements the `Error` trait.
//             TicTacToeError::Parse(ref e) => Some(e),
//         }
//     }
// }

impl From<StdInFailure> for TicTacToeError {
    fn from(e: StdInFailure) -> TicTacToeError {
        TicTacToeError::StdInFailure(e)
    }
}

impl From<StringIsNotASquare> for TicTacToeError {
    fn from(e: StringIsNotASquare) -> TicTacToeError {
        TicTacToeError::StringIsNotASquare(e.clone())
    }
}

impl From<SquareOccupied> for TicTacToeError {
    fn from(e: SquareOccupied) -> TicTacToeError {
        TicTacToeError::SquareOccupied(e)
    }
}
