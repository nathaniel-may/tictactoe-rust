mod game;

use game::Game;
use game::Game::{Active, Final};
use game::State::{Tie, Win};

fn play_game(game: Game) {
    match game {
        Final(g) => {
            let title = match g.state {
                Win(p) => format!(":: Player {} Wins! ::", p),
                Tie => "::    Tie Game!    ::".to_owned(),
            };
            println!("{}", title);
            println!("{}", Game::from(g));
        }
        Active(g) => {
            unimplemented!();
        }
    }
}

fn main() {
    println!(":: Tic Tac Toe ::");
    play_game(Game::from(Game::new()));
}
