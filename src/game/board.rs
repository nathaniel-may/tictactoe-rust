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
            I9 => "9"
        };
        write!(f, "{}", s.to_owned())
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
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

pub enum Board {
    Final(FinalBoard),
    Active(ActiveBoard),
}

impl Board {
    pub fn new() -> Board {
        Board::Active(ActiveBoard { m: HashMap::new() })
    }

    pub fn get_m(&self) -> &HashMap<Square, Player> {
        match self {
            Board::Final(b) => &b.m,
            Board::Active(b) => &b.m,
        }
    }
}

pub struct FinalBoard {
    m: HashMap<Square, Player>,
}

pub struct ActiveBoard {
    m: HashMap<Square, Player>,
}

impl ActiveBoard {
    pub fn place(&mut self, location: Square, player: Player) -> Result<(), String> {
        match self.m.get(&location) {
            None => {
                self.m.insert(location, player);
                Ok(())
            },
            Some(_) => Err(format!("Location {} is already taken.", location)),
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn format_square(b: &Board, sq: Square) -> String {
            b.get_m()
                .get(&sq)
                .map_or(" ".to_owned(), |p| format!("{}", p))
        }

        write!(
            f,
            "{}|{}|{}\n{}|{}|{}\n{}|{}|{}",
            format_square(self, I1),
            format_square(self, I2),
            format_square(self, I3),
            format_square(self, I4),
            format_square(self, I5),
            format_square(self, I6),
            format_square(self, I7),
            format_square(self, I8),
            format_square(self, I9)
        )
    }
}

impl From<FinalBoard> for Board {
    fn from(b: FinalBoard) -> Board {
        Board::Final(b)
    }
}

impl From<ActiveBoard> for Board {
    fn from(b: ActiveBoard) -> Board {
        Board::Active(b)
    }
}
