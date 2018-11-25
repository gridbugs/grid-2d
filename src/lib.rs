#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;
extern crate coord_2d;

pub mod coord {
    pub use super::coord_2d::{Coord, Size};
}

mod coord_system;
mod grid;

pub use self::coord_system::*;
pub use self::grid::*;
