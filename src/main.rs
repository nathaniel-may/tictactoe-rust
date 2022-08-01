use std::{fmt, io};
use tictactoe::{
    error::{SquareOccupied, StdInFailure, StringIsNotASquare, TicTacToeError},
    ActiveGame,
    Game::{self, Active, Final},
    Square,
};

// wrapper for errors so we can create our own trait instances for the library
// error type.
struct TicTacToeUserFacingError {
    e: TicTacToeError,
}

// explicitly takes control of the user interface from the underlying library.
// If the library updates its Display instance for exceptions we don't want
// the user interface to be silently modified.
impl fmt::Display for TicTacToeUserFacingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match &self.e {
            TicTacToeError::StdInFailure(_) => "System error reading input. Try again.".to_owned(),
            TicTacToeError::StringIsNotASquare(StringIsNotASquare { string }) => {
                format!("Squares are numbered 1-9. {} is invalid.", string.clone())
            }
            TicTacToeError::SquareOccupied(SquareOccupied { sq }) => {
                format!("Location {} is already taken.", sq)
            }
        };
        write!(f, "{}", s)
    }
}

// main game loop
fn play_game(game: Game) {
    // reading from stdin can fail
    fn get_input() -> Result<String, StdInFailure> {
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin
            .read_line(&mut buffer)
            .or_else(|e| Err(StdInFailure { e: e }))?;
        // remove the newline from entering the input
        buffer.pop();
        Ok(buffer)
    }

    // attempt to take a turn with the user input
    fn next(game: &mut ActiveGame) -> Result<Game, TicTacToeUserFacingError> {
        // Inner function makes calls to `?` utilize the correct `From` instances
        fn _next(game: &mut ActiveGame) -> Result<Game, TicTacToeError> {
            let input = get_input()?;
            let sq = Square::try_from(input)?;
            let next = game.take_turn(sq)?;
            Ok(next)
        }

        // explicitly convert the error to the user-facing variant
        _next(game).map_err(|e| TicTacToeUserFacingError { e: e })
    }

    println!("{}", game);
    match game {
        // the game is over. Exit the program.
        Final(_) => (),
        // the game is still active. Keep playing
        Active(mut g) => match next(&mut g) {
            Ok(x) => play_game(x),
            // if there was an error, show it to the user and try the turn again
            Err(e) => {
                println!("{}", e);
                play_game(Game::Active(g));
            }
        },
    }
}

fn main() {
    println!("::  Tic Tac Toe  ::\n");
    play_game(Game::new());
}
