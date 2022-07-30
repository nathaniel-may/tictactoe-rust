mod game;

use game::{
    Game::{self, Active, Final},
    Square,
};
use std::io;

fn play_game(game: Game) {
    println!("{}", game);
    match game {
        Final(_) => (),
        Active(mut g) => {
            let mut buffer = String::new();
            let stdin = io::stdin();
            stdin.read_line(&mut buffer).unwrap(); // TODO unwrap
                                                   // remove the newline from entering the input
            buffer.pop();
            let next = Square::try_from(buffer).and_then(|sq| g.take_turn(sq));
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
