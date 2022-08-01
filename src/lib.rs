//! # tictactoe
//!
//! `tictactoe` is a library for building a tictactoe game with as much type-safety as possible.

mod game;

pub use self::game::board::Square;
pub use self::game::error::*;
pub use self::game::*;
