//! # game
//!
//! `game` is the module that handles all the game state transitions.


// these modules need to be public for lib.rs to re-export the library interface
pub mod board;
pub mod error;

use board::{Board, Player, Square, Square::*};
use error::*;
use std::fmt;
use State::{Tie, Win};

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
            board: board::new(),
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
                format!("{}\n{}", title, &g.board)
            }
            Game::Active(g) => {
                format!("::    {}'s Turn    ::\n{}", g.active_player(), &g.board)
            }
        };
        write!(f, "{}", s)
    }
}

/// A `FinalGame` is completed and you cannot take any more turns on it.
/// Because board has no mutating functions it is safe to expose it here.
#[derive(Clone, Debug)]
pub struct FinalGame {
    pub state: State,
    pub board: Board,
}

/// An `ActiveGame` is not completed and you can continue to take more turns on it.
/// Because board has no mutating functions it is safe to expose it here.
#[derive(Clone, Debug)]
pub struct ActiveGame {
    pub board: Board,
}

impl ActiveGame {
    /// determines the active player from the number of pieces on the board.
    /// reads the value of every square on the board.
    fn active_player(&self) -> Player {
        let xs = self.board.piece_count(Player::X);
        let os = self.board.piece_count(Player::O);
        if xs > os {
            Player::O
        } else {
            Player::X
        }
    }

    /// takes a turn and returns a new value representing the next state of the game.
    /// # Errors
    /// `SquareOccupied` - if the square is already occupied by a piece.
    pub fn take_turn(&self, location: Square) -> Result<Game, SquareOccupied> {
        // determine whose turn it is
        let player = self.active_player();

        // placing the piece (will return Err if occupied)
        let next_board = self.board.place(location, player)?;

        // check for winner
        for (x, y, z) in Game::WINNING_LINES {
            if x == location || y == location || z == location {
                if [x, y, z]
                    .into_iter() // TODO do this on the stack instead
                    .all(|loc| next_board.get(loc) == Some(player))
                {
                    return Ok(Game::Final(FinalGame {
                        state: Win(player),
                        board: next_board,
                    }));
                }
            }
        }

        // check for tie
        let xs = next_board.piece_count(Player::X);
        let os = next_board.piece_count(Player::O);
        if xs + os >= 9 {
            return Ok(Game::Final(FinalGame {
                state: Tie,
                board: next_board,
            }));
        }

        // the game isn't final so it must be active
        Ok(Game::Active(ActiveGame{board: next_board}))
    }
}
