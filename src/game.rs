mod board;

use board::{
    ActiveBoard, Board, FinalBoard, Player,
    Square::*,
};
use State::{
    Tie,
    Win
};
use std::fmt;
pub use board::Square;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum State {
    Win(Player),
    Tie,
}

#[derive(Clone, Debug)]
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

    const WINNING_LINES: [(Square, Square, Square); 8] = [
        // Horizontal
        (I1, I2, I3),
        (I4, I5, I6),
        (I7, I8, I9),
        // Vertical
        (I1, I4, I7),
        (I2, I5, I8),
        (I3, I6, I9),
        // Diagonal
        (I1, I5, I9),
        (I7, I5, I3),
    ];
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Game::Final(g) => {
                let title = match g.state {
                    Win(p) => format!(":: Player {} Wins! ::", p),
                    Tie => "::    Tie Game!   ::".to_owned(),
                };
                format!("{}\n{}", title, Board::from(&g.board))
            }
            Game::Active(g) => {
                format!("::    {}'s Turn    ::\n{}", g.active_player(), Board::from(&g.board))
            }
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug)]
pub struct FinalGame {
    pub state: State,
    pub board: FinalBoard,
}

#[derive(Clone, Debug)]
pub struct ActiveGame {
    pub board: ActiveBoard,
}

impl ActiveGame {
    fn active_player(&self) -> Player {
        let xs = self.board.piece_count(Player::X);
        let os = self.board.piece_count(Player::O);
        if xs > os {
            Player::O
        } else {
            Player::X
        }
    }

    pub fn take_turn(&mut self, location: Square) -> Result<Game, String> {
        // determine whose turn it is
        let player = self.active_player();

        // place the piece (will return Err if occupied)
        self.board.place(location, player)?;

        // check for winner
        for (x, y, z) in Game::WINNING_LINES {
            if x == location || y == location || z == location {
                if [x, y, z].into_iter().all(|loc| self.board.get(loc) == Some(player)) {
                    return Ok(Game::from(&FinalGame{state: Win(player), board: FinalBoard::from(&self.board)}))  // TODO clone
                }
            }
        }

        // check for tie
        let xs = self.board.piece_count(Player::X);
        let os = self.board.piece_count(Player::O);
        if xs + os >= 9 {
            return Ok(Game::from(&FinalGame{state: Tie, board: FinalBoard::from(&self.board)}))
        }

        Ok(Game::Active(self.clone())) // TODO clone
    }
}

impl From<&FinalGame> for Game {
    fn from(g: &FinalGame) -> Game {
        Game::Final(g.clone())
    }
}

impl From<&ActiveGame> for Game {
    fn from(g: &ActiveGame) -> Game {
        Game::Active(g.clone())
    }
}
