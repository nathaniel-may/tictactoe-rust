mod game;

use game::{
    Game::{self, Active, Final},
    Square,
    error::{TicTacToeError, StdInFailure, StringIsNotASquare, SquareOccupied}
};
use std::{fmt, io};

struct TicTacToeUserFacingError {
    e: TicTacToeError
}

impl fmt::Display for TicTacToeUserFacingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match &self.e {
            TicTacToeError::StdInFailure(_) => 
                "System error reading input. Try again.".to_owned(),
            TicTacToeError::StringIsNotASquare(StringIsNotASquare{string}) => 
                format!("Squares are numbered 1-9. {} is invalid.", string.clone()),
            TicTacToeError::SquareOccupied(SquareOccupied{sq}) => 
                format!("Location {} is already taken.", sq),
        };
        write!(f, "{}", s)
    }
}

// Main game loop
fn play_game(game: Game) {
    // reading from stdin can fail
    fn get_input() -> Result<String, StdInFailure> {
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin
            .read_line(&mut buffer)
            .or_else(|_| Err(StdInFailure {}))?;
        // remove the newline from entering the input
        buffer.pop();
        Ok(buffer)
    }

    println!("{}", game);
    match game {
        // The game is over. Exit the program.
        Final(_) => (),
        // The game is still active. Keep playing
        Active(mut g) => {
            // Attempt to take a turn with the user input
            let next = get_input().map_err(|e| TicTacToeError::from(&e))
                .and_then(|input| Square::try_from(input).map_err(|e| TicTacToeError::from(&e)))
                .and_then(|sq| g.take_turn(sq).map_err(|e| TicTacToeError::from(&e)));

            // Continue playing or start this turn over if there was an error
            match next {
                Ok(x) => play_game(x),
                Err(e) => {
                    println!("{}", TicTacToeUserFacingError{ e: e });
                    play_game(Game::Active(g));
                }
            }
        }
    }
}

fn main() {
    println!("::  Tic Tac Toe  ::\n");
    play_game(Game::new());
}
