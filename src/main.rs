mod game;

use game::{
    Game::{self, Active, Final},
    Square,
};
use std::io;

// Main game loop
fn play_game(game: Game) {
    // reading from stdin can fail
    fn get_input() -> Result<String, String> {
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin
            .read_line(&mut buffer)
            .or_else(|_| Err("System error reading input. Try again.".to_owned()))?;
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
            let next = get_input()
                .and_then(|input| Square::try_from(input))
                .and_then(|sq| g.take_turn(sq));

            // Continue playing or start this turn over if there was an error
            match next {
                Ok(x) => play_game(x),
                Err(e) => {
                    println!("{}", e);
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
