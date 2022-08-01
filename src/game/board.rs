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

pub fn new() -> ActiveBoard {
    ActiveBoard { m: HashMap::new() }
}

fn display_string(m: &HashMap<Square, Player>) -> String {
    fn format_square(m: &HashMap<Square, Player>, sq: Square) -> String {
        m.get(&sq)
            .map_or(format!("<{}>", sq), |p| format!(" {} ", p))
    }

    format!(
        "  {} | {} | {}\n  {} | {} | {}\n  {} | {} | {}",
        format_square(m, I1),
        format_square(m, I2),
        format_square(m, I3),
        format_square(m, I4),
        format_square(m, I5),
        format_square(m, I6),
        format_square(m, I7),
        format_square(m, I8),
        format_square(m, I9)
    )
}

#[derive(Clone, Debug)]
pub struct FinalBoard {
    m: HashMap<Square, Player>,
}

impl fmt::Display for FinalBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", display_string(&self.m))
    }
}

#[derive(Clone, Debug)]
pub struct ActiveBoard {
    m: HashMap<Square, Player>,
}

impl ActiveBoard {
    pub fn place(&mut self, location: Square, player: Player) -> Result<(), SquareOccupied> {
        match self.m.get(&location) {
            None => {
                self.m.insert(location, player);
                Ok(())
            }
            Some(_) => Err(SquareOccupied { sq: location }),
        }
    }

    pub fn piece_count(&self, player: Player) -> usize {
        self.m.values().filter(|v| **v == player).count()
    }

    pub fn get(&self, location: Square) -> Option<Player> {
        self.m.get(&location).cloned()
    }
}

impl fmt::Display for ActiveBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", display_string(&self.m))
    }
}

impl From<&ActiveBoard> for FinalBoard {
    fn from(b: &ActiveBoard) -> FinalBoard {
        FinalBoard { m: b.m.clone() }
    }
}
