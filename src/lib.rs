#[cfg(feature = "serialize")]
#[macro_use]
extern crate serde;
extern crate coord_2d;

pub mod coord_system;
mod grid;

pub use self::grid::*;
pub use coord_2d::{Coord, Size};
