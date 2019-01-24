use coord_2d::{Coord, Size};

/// A mapping from coordinate to position in the Vec backing the grid.
/// Generally implementations will own the size of the grid.
pub trait CoordSystem {
    /// An iterator which yields coords in thet same order as elements
    /// are stored in the grid.
    type CoordIter: Iterator<Item = Coord>;

    /// The size of the grid
    fn size(&self) -> Size;

    /// Given a coord, returns the index of the backing Vec which
    /// corresponds to that coordinate. May assume that
    /// `coord.is_valid(self.size())`.
    fn index_of_coord_unchecked(&self, coord: Coord) -> usize;

    fn index_of_coord_checked(&self, coord: Coord) -> usize {
        if coord.is_valid(self.size()) {
            self.index_of_coord_unchecked(coord)
        } else {
            panic!("coord out of bounds");
        }
    }

    fn index_of_coord(&self, coord: Coord) -> Option<usize> {
        if coord.is_valid(self.size()) {
            Some(self.index_of_coord_unchecked(coord))
        } else {
            None
        }
    }

    fn index_of_normalized_coord(&self, coord: Coord) -> usize {
        self.index_of_coord_unchecked(coord.normalize(self.size()))
    }

    /// Returns an iterator over coords
    fn coord_iter(&self) -> Self::CoordIter;
}

/// Sanity check for `CoordSystem` implementations, which panics if
/// a given coord system is not sane.
pub fn validate<C: CoordSystem>(coord_system: &C) {
    let indices = coord_system
        .coord_iter()
        .map(|coord| coord_system.index_of_coord_unchecked(coord))
        .collect::<Vec<_>>();
    let expected_indices = (0..coord_system.size().count()).collect::<Vec<_>>();
    if indices != expected_indices {
        panic!("coord_iter doesn't visit coordinates in index order");
    }
}

/// `CoordSystem` which starts in the top-left corner and traverses
/// each row from top to bottom, traversing from left to right
/// within each row.
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct XThenY {
    size: Size,
}

impl XThenY {
    pub fn new(size: Size) -> Self {
        Self { size }
    }
}

impl From<Size> for XThenY {
    fn from(size: Size) -> Self {
        Self::new(size)
    }
}

#[derive(Debug)]
pub struct XThenYIter {
    coord: Coord,
    size: Size,
}

impl XThenYIter {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            coord: Coord { x: 0, y: 0 },
        }
    }
}

impl From<Size> for XThenYIter {
    fn from(size: Size) -> Self {
        Self::new(size)
    }
}

impl Iterator for XThenYIter {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        if self.coord.y == self.size.y() as i32 {
            return None;
        }
        let coord = self.coord;
        self.coord.x += 1;
        if self.coord.x == self.size.x() as i32 {
            self.coord.x = 0;
            self.coord.y += 1;
        }
        Some(coord)
    }
}

impl CoordSystem for XThenY {
    type CoordIter = XThenYIter;
    fn size(&self) -> Size {
        self.size
    }

    fn index_of_coord_unchecked(&self, coord: Coord) -> usize {
        (coord.y as u32 * self.size.x() + coord.x as u32) as usize
    }

    fn coord_iter(&self) -> Self::CoordIter {
        XThenYIter::from(self.size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn x_then_y() {
        validate(&XThenY::from(Size::new(37, 51)));
        validate(&XThenY::from(Size::new(1, 1)));
        validate(&XThenY::from(Size::new(0, 0)));
    }
}
