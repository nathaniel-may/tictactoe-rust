//! # board
//!
//! `board` is the module that handles all the game state.

use super::error::{SquareOccupied, StringIsNotASquare};
use std::collections::HashMap;
use std::fmt;
use Square::*;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Square {
    I1,
    I2,
    I3,
    I4,
    I5,
    I6,
    I7,
    I8,
    I9,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            I1 => "1",
            I2 => "2",
            I3 => "3",
            I4 => "4",
            I5 => "5",
            I6 => "6",
            I7 => "7",
            I8 => "8",
            I9 => "9",
        };
        write!(f, "{}", s.to_owned())
    }
}

impl TryFrom<String> for Square {
    type Error = StringIsNotASquare;

    fn try_from(s: String) -> Result<Square, StringIsNotASquare> {
        let pairs = [
            ("1", I1),
            ("2", I2),
            ("3", I3),
            ("4", I4),
            ("5", I5),
            ("6", I6),
            ("7", I7),
            ("8", I8),
            ("9", I9),
        ];

        for (input, parsed) in pairs {
            if input == s {
                return Ok(parsed);
            }
        }

        Err(StringIsNotASquare { string: s })
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Player {
    X,
    O,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Player::X => "X",
            Player::O => "O",
        };
        write!(f, "{}", s.to_owned())
    }
}

pub fn new() -> Board {
    Board { m: HashMap::new() }
}

/// An immutable board type that can only ever have exactly 9 keys.
#[derive(Clone, Debug)]
pub struct Board {
    m: HashMap<Square, Player>,
}

impl Board {
    /// places a the player piece on the square.
    /// # Errors
    /// `SquareOccupied` if the square is occupied.
    pub fn place(&self, location: Square, player: Player) -> Result<Board, SquareOccupied> {
        let mut next_m = self.m.clone();
        match next_m.get(&location) {
            None => {
                next_m.insert(location, player);
                Ok(Board{m: next_m})
            }
            Some(_) => Err(SquareOccupied { sq: location }),
        }
    }

    /// returns the number of squares occupied by the player
    pub fn piece_count(&self, player: Player) -> usize {
        self.m.values().filter(|v| **v == player).count()
    }

    /// returns the value at that locationon the board: `Some(player)` or `None`.
    pub fn get(&self, location: Square) -> Option<Player> {
        self.m.get(&location).cloned()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn format_square(m: &HashMap<Square, Player>, sq: Square) -> String {
            m.get(&sq)
                .map_or(format!("<{}>", sq), |p| format!(" {} ", p))
        }
    
        write!(
            f,
            "  {} | {} | {}\n  {} | {} | {}\n  {} | {} | {}",
            format_square(&self.m, I1),
            format_square(&self.m, I2),
            format_square(&self.m, I3),
            format_square(&self.m, I4),
            format_square(&self.m, I5),
            format_square(&self.m, I6),
            format_square(&self.m, I7),
            format_square(&self.m, I8),
            format_square(&self.m, I9)
        )
    }
}
