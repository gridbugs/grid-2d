use coord_2d::*;
use coord_system::*;
use std::iter;
use std::ops::{Index, IndexMut};
use std::slice;
use std::vec;

pub type GridIter<'a, T> = slice::Iter<'a, T>;
pub type GridIterMut<'a, T> = slice::IterMut<'a, T>;
pub type GridEnumerate<'a, T, C = XThenYIter> = iter::Zip<C, GridIter<'a, T>>;
pub type GridEnumerateMut<'a, T, C = XThenYIter> = iter::Zip<C, GridIterMut<'a, T>>;
pub type GridIntoIter<T> = vec::IntoIter<T>;
pub type GridIntoEnumerate<T, C = XThenYIter> = iter::Zip<C, GridIntoIter<T>>;

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Grid<T, C: CoordSystem = XThenY> {
    coord_system: C,
    cells: Vec<T>,
}

impl<T, C: CoordSystem> Grid<T, C> {
    fn new_uninitialised_with_coord_system(coord_system: C) -> Self {
        Self {
            cells: Vec::with_capacity(coord_system.size().count()),
            coord_system,
        }
    }

    pub fn new_fn_with_coord_system<F>(coord_system: C, mut f: F) -> Self
    where
        F: FnMut(Coord) -> T,
    {
        let mut grid = Grid::new_uninitialised_with_coord_system(coord_system);
        for coord in grid.coord_system.coord_iter() {
            grid.cells.push(f(coord));
        }
        grid
    }

    pub fn new_fn_move_map_with_coord_system<F, U, D>(
        coord_system: C,
        grid: Grid<U, D>,
        mut f: F,
    ) -> Self
    where
        F: FnMut(Coord, U) -> T,
        D: CoordSystem,
    {
        unimplemented!()
    }
}
impl<T: Clone, C: CoordSystem> Grid<T, C> {
    pub fn new_clone_with_coord_system(coord_system: C, value: T) -> Self {
        Grid::new_fn_with_coord_system(coord_system, |_| value.clone())
    }
}

impl<T: Default, C: CoordSystem> Grid<T, C> {
    pub fn new_default_with_coord_system(coord_system: C) -> Self {
        Grid::new_fn_with_coord_system(coord_system, |_| T::default())
    }
}
impl<T> Grid<T, XThenY> {
    pub fn new_fn<F>(size: Size, f: F) -> Self
    where
        F: FnMut(Coord) -> T,
    {
        Self::new_fn_with_coord_system(XThenY::from(size), f)
    }
}
impl<T: Clone> Grid<T, XThenY> {
    pub fn new_clone(size: Size, value: T) -> Self {
        Grid::new_clone_with_coord_system(XThenY::from(size), value)
    }
}

impl<T: Default> Grid<T, XThenY> {
    pub fn new_default(size: Size) -> Self {
        Grid::new_default_with_coord_system(XThenY::from(size))
    }
}

impl<T, C: CoordSystem> Grid<T, C> {
    pub fn width(&self) -> u32 {
        self.size().x()
    }
    pub fn height(&self) -> u32 {
        self.size().y()
    }
    pub fn size(&self) -> Size {
        self.coord_system.size()
    }
    pub fn len(&self) -> usize {
        self.size().count()
    }
    pub fn iter(&self) -> GridIter<T> {
        self.cells.iter()
    }
    pub fn iter_mut(&mut self) -> GridIterMut<T> {
        self.cells.iter_mut()
    }
    pub fn coord_iter(&self) -> C::CoordIter {
        self.coord_system.coord_iter()
    }
    pub fn get(&self, coord: Coord) -> Option<&T> {
        self.coord_system
            .index_of_coord(coord)
            .map(|index| &self.cells[index])
    }
    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        self.coord_system
            .index_of_coord(coord)
            .map(move |index| &mut self.cells[index])
    }
    pub fn get_tiled(&self, coord: Coord) -> &T {
        &self.cells[self.coord_system.index_of_normalized_coord(coord)]
    }
    pub fn get_tiled_mut(&mut self, coord: Coord) -> &mut T {
        let index = self.coord_system.index_of_normalized_coord(coord);
        &mut self.cells[index]
    }
    pub fn index_of_coord(&self, coord: Coord) -> Option<usize> {
        self.coord_system.index_of_coord(coord)
    }
    pub fn get_index(&self, index: usize) -> &T {
        self.cells.index(index)
    }
    pub fn get_index_mut(&mut self, index: usize) -> &mut T {
        self.cells.index_mut(index)
    }
    pub fn get_checked(&self, coord: Coord) -> &T {
        self.cells
            .index(self.coord_system.index_of_coord_checked(coord))
    }
    pub fn get_checked_mut(&mut self, coord: Coord) -> &mut T {
        let index = self.coord_system.index_of_coord_checked(coord);
        self.cells.index_mut(index)
    }
    pub fn enumerate(&self) -> GridEnumerate<T, C::CoordIter> {
        self.coord_iter().zip(self.iter())
    }
    pub fn enumerate_mut(&mut self) -> GridEnumerateMut<T, C::CoordIter> {
        self.coord_iter().zip(self.iter_mut())
    }
    pub fn into_enumerate(self) -> GridIntoEnumerate<T, C::CoordIter> {
        self.coord_iter().zip(self.into_iter())
    }
}

impl<T, C: CoordSystem> Index<usize> for Grid<T, C> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.cells.index(index)
    }
}

impl<T, C: CoordSystem> IndexMut<usize> for Grid<T, C> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.cells.index_mut(index)
    }
}

impl<T, C: CoordSystem> IntoIterator for Grid<T, C> {
    type Item = T;
    type IntoIter = GridIntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.cells.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        grid[index] = Coord::new(1000, 1000);
        assert_eq!(grid[index], Coord::new(1000, 1000));
    }
}
