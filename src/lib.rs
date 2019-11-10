#[cfg(feature = "serialize")]
#[macro_use]
extern crate serde;
extern crate coord_2d;

pub use coord_2d::{Coord, Size};
use std::iter;
use std::ops::{Index, IndexMut};
use std::slice;
use std::vec;

pub type GridIter<'a, T> = slice::Iter<'a, T>;
pub type GridIterMut<'a, T> = slice::IterMut<'a, T>;
pub type GridEnumerate<'a, T> = iter::Zip<CoordIter, GridIter<'a, T>>;
pub type GridEnumerateMut<'a, T> = iter::Zip<CoordIter, GridIterMut<'a, T>>;
pub type GridIntoIter<T> = vec::IntoIter<T>;
pub type GridIntoEnumerate<T> = iter::Zip<CoordIter, GridIntoIter<T>>;
pub type GridRows<'a, T> = slice::Chunks<'a, T>;
pub type GridRowsMut<'a, T> = slice::ChunksMut<'a, T>;

#[derive(Debug, Clone, Copy)]
pub struct IteratorLengthDifferentFromSize;

pub struct CoordIter {
    coord: Coord,
    size: Size,
}

impl CoordIter {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            coord: Coord { x: 0, y: 0 },
        }
    }
}

impl Iterator for CoordIter {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        if self.coord.y == self.size.height() as i32 {
            return None;
        }
        let coord = self.coord;
        self.coord.x += 1;
        if self.coord.x == self.size.width() as i32 {
            self.coord.x = 0;
            self.coord.y += 1;
        }
        Some(coord)
    }
}

#[derive(Debug)]
pub enum Get2Error {
    CoordsEqual,
    LeftOutOfBounds,
    RightOutOfBounds,
}

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Grid<T> {
    cells: Vec<T>,
    size: Size,
}

impl<T> Grid<T> {
    fn new_uninitialised(size: Size) -> Self {
        Self {
            cells: Vec::with_capacity(size.count()),
            size,
        }
    }

    pub fn new_fn<F>(size: Size, mut f: F) -> Self
    where
        F: FnMut(Coord) -> T,
    {
        let mut grid = Grid::new_uninitialised(size);
        for coord in CoordIter::new(size) {
            grid.cells.push(f(coord));
        }
        grid
    }

    pub fn try_new_iterator<I>(
        size: Size,
        iterator: I,
    ) -> Result<Self, IteratorLengthDifferentFromSize>
    where
        I: Iterator<Item = T>,
    {
        let cells: Vec<T> = iterator.collect();
        if cells.len() != size.count() {
            return Err(IteratorLengthDifferentFromSize);
        }
        Ok(Self { cells, size })
    }

    pub fn new_iterator<I>(size: Size, iterator: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        Self::try_new_iterator(size, iterator).unwrap()
    }

    pub fn new_grid_map_with_coord<U, F>(grid: Grid<U>, mut f: F) -> Self
    where
        F: FnMut(Coord, U) -> T,
    {
        let size = grid.size;
        let cells = CoordIter::new(size)
            .zip(grid.cells.into_iter())
            .map(|(coord, u)| f(coord, u))
            .collect();
        Self { cells, size }
    }

    pub fn new_grid_map<U, F>(grid: Grid<U>, f: F) -> Self
    where
        F: FnMut(U) -> T,
    {
        let size = grid.size;
        let cells = grid.cells.into_iter().map(f).collect();
        Self { cells, size }
    }

    pub fn new_grid_map_ref<U, F>(grid: &Grid<U>, f: F) -> Self
    where
        F: FnMut(&U) -> T,
    {
        let size = grid.size;
        let cells = grid.iter().map(f).collect();
        Self { cells, size }
    }

    pub fn new_grid_map_ref_with_coord<U, F>(grid: &Grid<U>, mut f: F) -> Self
    where
        F: FnMut(Coord, &U) -> T,
    {
        let cells = grid
            .coord_iter()
            .zip(grid.iter())
            .map(|(coord, cell)| f(coord, cell))
            .collect();
        Self {
            cells,
            size: grid.size,
        }
    }
}

impl<T: Clone> Grid<T> {
    pub fn new_clone(size: Size, value: T) -> Self {
        Grid::new_fn(size, |_| value.clone())
    }
}

impl<T: Default> Grid<T> {
    pub fn new_default(size: Size) -> Self {
        Grid::new_fn(size, |_| T::default())
    }
}

impl<T> Grid<T> {
    pub fn width(&self) -> u32 {
        self.size.width()
    }
    pub fn height(&self) -> u32 {
        self.size.height()
    }
    pub fn size(&self) -> Size {
        self.size
    }
    pub fn iter(&self) -> GridIter<T> {
        self.cells.iter()
    }
    pub fn iter_mut(&mut self) -> GridIterMut<T> {
        self.cells.iter_mut()
    }
    pub fn coord_iter(&self) -> CoordIter {
        CoordIter::new(self.size)
    }
    pub fn get(&self, coord: Coord) -> Option<&T> {
        self.index_of_coord(coord).map(|index| &self.cells[index])
    }
    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        self.index_of_coord(coord)
            .map(move |index| &mut self.cells[index])
    }
    pub fn get_tiled(&self, coord: Coord) -> &T {
        &self.cells[self.index_of_normalized_coord(coord)]
    }
    pub fn get_tiled_mut(&mut self, coord: Coord) -> &mut T {
        let index = self.index_of_normalized_coord(coord);
        &mut self.cells[index]
    }
    pub fn index_of_coord_unchecked(&self, coord: Coord) -> usize {
        (coord.y as u32 * self.size.width() + coord.x as u32) as usize
    }
    pub fn index_of_coord(&self, coord: Coord) -> Option<usize> {
        if coord.is_valid(self.size) {
            Some(self.index_of_coord_unchecked(coord))
        } else {
            None
        }
    }
    fn index_of_coord_checked(&self, coord: Coord) -> usize {
        if coord.is_valid(self.size) {
            self.index_of_coord_unchecked(coord)
        } else {
            panic!("coord out of bounds");
        }
    }
    fn index_of_normalized_coord(&self, coord: Coord) -> usize {
        self.index_of_coord_unchecked(coord.normalize(self.size))
    }
    pub fn get_index_checked(&self, index: usize) -> &T {
        self.cells.index(index)
    }
    pub fn get_index_checked_mut(&mut self, index: usize) -> &mut T {
        self.cells.index_mut(index)
    }
    pub fn get_checked(&self, coord: Coord) -> &T {
        self.cells.index(self.index_of_coord_checked(coord))
    }
    pub fn get_checked_mut(&mut self, coord: Coord) -> &mut T {
        let index = self.index_of_coord_checked(coord);
        self.cells.index_mut(index)
    }
    pub fn enumerate(&self) -> GridEnumerate<T> {
        self.coord_iter().zip(self.iter())
    }
    pub fn enumerate_mut(&mut self) -> GridEnumerateMut<T> {
        self.coord_iter().zip(self.iter_mut())
    }
    pub fn into_enumerate(self) -> GridIntoEnumerate<T> {
        self.coord_iter().zip(self.cells.into_iter())
    }
    pub fn rows(&self) -> GridRows<T> {
        self.cells.chunks(self.size.width() as usize)
    }
    pub fn rows_mut(&mut self) -> GridRowsMut<T> {
        self.cells.chunks_mut(self.size.width() as usize)
    }
    pub fn get2_mut(&mut self, a: Coord, b: Coord) -> Result<(&mut T, &mut T), Get2Error> {
        if a == b {
            return Err(Get2Error::CoordsEqual);
        }
        let index_a = self.index_of_coord(a).ok_or(Get2Error::LeftOutOfBounds)?;
        let index_b = self.index_of_coord(b).ok_or(Get2Error::LeftOutOfBounds)?;
        if index_a < index_b {
            let (slice_a, slice_b) = self.cells.split_at_mut(index_b);
            Ok((&mut slice_a[index_a], &mut slice_b[0]))
        } else {
            let (slice_b, slice_a) = self.cells.split_at_mut(index_a);
            Ok((&mut slice_a[0], &mut slice_b[index_b]))
        }
    }
    pub fn get2_checked_mut(&mut self, a: Coord, b: Coord) -> (&mut T, &mut T) {
        if a == b {
            panic!("coords may not be equal");
        }
        let index_a = self.index_of_coord_checked(a);
        let index_b = self.index_of_coord_checked(b);
        if index_a < index_b {
            let (slice_a, slice_b) = self.cells.split_at_mut(index_b);
            (&mut slice_a[index_a], &mut slice_b[0])
        } else {
            let (slice_b, slice_a) = self.cells.split_at_mut(index_a);
            (&mut slice_a[0], &mut slice_b[index_b])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    fn coord_grid(size: Size) -> Grid<Coord> {
        Grid::new_fn(size, |coord| coord)
    }

    #[test]
    #[should_panic]
    fn out_of_bounds() {
        let grid = coord_grid(Size::new(2, 3));
        grid.get_checked(Coord::new(0, 3));
    }

    #[test]
    fn tiling() {
        let mut grid = coord_grid(Size::new(2, 3));
        assert_eq!(*grid.get_tiled(Coord::new(-10, -30)), Coord::new(0, 0));
        *grid.get_tiled_mut(Coord::new(-12, -12)) = Coord::new(1000, 1000);
        assert_eq!(*grid.get_tiled(Coord::new(10, 30)), Coord::new(1000, 1000));
    }

    #[test]
    fn enumerate() {
        let mut grid = coord_grid(Size::new(24, 42));
        grid.enumerate()
            .for_each(|(coord, cell)| assert_eq!(coord, *cell));
        grid.enumerate_mut()
            .for_each(|(coord, cell)| *cell = coord * 3);
        grid.enumerate()
            .for_each(|(coord, cell)| assert_eq!(coord * 3, *cell));
    }

    #[test]
    fn index() {
        let mut grid = coord_grid(Size::new(7, 9));
        let index = grid.index_of_coord(Coord::new(5, 3)).unwrap();
        assert_eq!(index, 26);
        *grid.get_index_checked_mut(index) = Coord::new(1000, 1000);
        assert_eq!(*grid.get_index_checked(index), Coord::new(1000, 1000));
    }

    #[test]
    fn get2_checked_mut() {
        let mut grid = coord_grid(Size::new(4, 4));
        let (a, b) = grid.get2_checked_mut(Coord::new(1, 2), Coord::new(2, 1));
        mem::swap(a, b);
        assert_eq!(*grid.get_checked(Coord::new(1, 2)), Coord::new(2, 1));
        assert_eq!(*grid.get_checked(Coord::new(2, 1)), Coord::new(1, 2));
    }

    #[test]
    #[should_panic]
    fn get2_checked_mut_equal_coords() {
        let mut grid = coord_grid(Size::new(4, 4));
        let (_, _) = grid.get2_checked_mut(Coord::new(1, 2), Coord::new(1, 2));
    }
}
