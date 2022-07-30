mod board;

use board::Board;
use board::Player;
use board::Square;

pub enum State {
    Win(Player),
    Tie,
}

pub enum Game {
    Final(FinalGame),
    Active(ActiveGame),
}

impl Game {
    pub fn new() -> Game {
        Game::Active(ActiveGame {
            board: Board::new(),
        })
    }
}

pub struct FinalGame {
    pub state: State,
    pub board: Board,
}

pub struct ActiveGame {
    pub board: Board,
}

impl ActiveGame {
    fn take_turn(&self, location: Square) -> Game {
        unimplemented!();
    }
}

impl From<FinalGame> for Game {
    fn from(g: FinalGame) -> Game {
        Game::Final(g)
    }
}

impl From<ActiveGame> for Game {
    fn from(g: ActiveGame) -> Game {
        Game::Active(g)
    }
}
