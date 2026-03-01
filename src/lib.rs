pub use coord_2d::{self, ICoord, UCoord};
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use std::iter;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::slice;
use std::vec;

pub type ICoordIter = coord_2d::ICoordIterRowMajor;
pub type EdgeICoordIter = coord_2d::EdgeIter;
pub type GridIter<'a, T> = slice::Iter<'a, T>;
pub type GridIterMut<'a, T> = slice::IterMut<'a, T>;
pub type GridEnumerate<'a, T> = iter::Zip<ICoordIter, GridIter<'a, T>>;
pub type GridEnumerateMut<'a, T> = iter::Zip<ICoordIter, GridIterMut<'a, T>>;
pub type GridIntoIter<T> = vec::IntoIter<T>;
pub type GridIntoEnumerate<T> = iter::Zip<ICoordIter, GridIntoIter<T>>;
pub type GridRows<'a, T> = slice::Chunks<'a, T>;
pub type GridRowsMut<'a, T> = slice::ChunksMut<'a, T>;
pub type GridEdgeEnumerate<'a, T> = iter::Zip<EdgeICoordIter, GridEdgeIter<'a, T>>;
pub type GridEdgeEnumerateMut<'a, T> = iter::Zip<EdgeICoordIter, GridEdgeIterMut<'a, T>>;

#[derive(Debug, Clone, Copy)]
pub struct IteratorLengthDifferentFromUCoord;

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
    size: UCoord,
}

impl<T> Grid<T> {
    pub fn new_fn<F>(size: UCoord, mut f: F) -> Self
    where
        F: FnMut(ICoord) -> T,
    {
        let count = size.count();
        let mut cells = Vec::with_capacity(count);
        for coord in size.icoord_iter_row_major() {
            cells.push(f(coord));
        }
        Self { cells, size }
    }

    pub fn try_new_iterator<I>(
        size: UCoord,
        iterator: I,
    ) -> Result<Self, IteratorLengthDifferentFromUCoord>
    where
        I: Iterator<Item = T>,
    {
        let cells: Vec<T> = iterator.collect();
        if cells.len() != size.count() {
            return Err(IteratorLengthDifferentFromUCoord);
        }
        Ok(Self { cells, size })
    }

    pub fn new_iterator<I>(size: UCoord, iterator: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        Self::try_new_iterator(size, iterator).unwrap()
    }

    pub fn new_grid_map<U, F>(grid: Grid<U>, f: F) -> Self
    where
        F: FnMut(U) -> T,
    {
        let size = grid.size;
        let cells = grid.cells.into_iter().map(f).collect();
        Self { cells, size }
    }

    pub fn new_grid_map_with_coord<U, F>(grid: Grid<U>, mut f: F) -> Self
    where
        F: FnMut(ICoord, U) -> T,
    {
        let size = grid.size;
        let cells = size
            .icoord_iter_row_major()
            .zip(grid.cells)
            .map(|(coord, u)| f(coord, u))
            .collect();
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
        F: FnMut(ICoord, &U) -> T,
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
    pub fn new_clone(size: UCoord, value: T) -> Self {
        let count = size.count();
        let mut cells = Vec::with_capacity(count);
        cells.resize(count, value);
        Self { cells, size }
    }

    pub fn transpose_clone(&self) -> Self {
        Self::new_fn(self.size.transpose(), |coord| {
            self.get_checked(coord.transpose()).clone()
        })
    }
}

impl<T: Copy> Grid<T> {
    pub fn new_copy(size: UCoord, value: T) -> Self {
        let count = size.count();
        let mut cells = Vec::with_capacity(count);
        cells.resize_with(count, || value);
        Self { cells, size }
    }
}

impl<T: Default> Grid<T> {
    pub fn new_default(size: UCoord) -> Self {
        let count = size.count();
        let mut cells = Vec::new();
        cells.resize_with(count, Default::default);
        Self { cells, size }
    }
}

impl<T> Grid<T> {
    pub fn width(&self) -> u32 {
        self.size.width()
    }
    pub fn height(&self) -> u32 {
        self.size.height()
    }
    pub fn size(&self) -> UCoord {
        self.size
    }
    pub fn is_empty(&self) -> bool {
        self.size.is_empty()
    }
    pub fn len(&self) -> usize {
        self.cells.len()
    }
    pub fn iter(&self) -> GridIter<'_, T> {
        self.cells.iter()
    }
    pub fn iter_mut(&mut self) -> GridIterMut<'_, T> {
        self.cells.iter_mut()
    }
    pub fn coord_iter(&self) -> ICoordIter {
        self.size.icoord_iter_row_major()
    }
    pub fn get(&self, coord: ICoord) -> Option<&T> {
        self.index_of_coord(coord).map(|index| &self.cells[index])
    }
    pub fn get_mut(&mut self, coord: ICoord) -> Option<&mut T> {
        self.index_of_coord(coord)
            .map(move |index| &mut self.cells[index])
    }
    pub fn get_tiled(&self, coord: ICoord) -> &T {
        &self.cells[self.index_of_normalized_coord(coord)]
    }
    pub fn get_tiled_mut(&mut self, coord: ICoord) -> &mut T {
        let index = self.index_of_normalized_coord(coord);
        &mut self.cells[index]
    }
    pub fn index_of_coord_unchecked(&self, coord: ICoord) -> usize {
        (coord.y as u32 * self.size.width() + coord.x as u32) as usize
    }
    pub fn index_of_coord(&self, coord: ICoord) -> Option<usize> {
        if coord.is_valid(self.size) {
            Some(self.index_of_coord_unchecked(coord))
        } else {
            None
        }
    }
    fn index_of_coord_checked(&self, coord: ICoord) -> usize {
        if coord.is_valid(self.size) {
            self.index_of_coord_unchecked(coord)
        } else {
            panic!("coord {} out of bounds", coord);
        }
    }
    fn index_of_normalized_coord(&self, coord: ICoord) -> usize {
        self.index_of_coord_unchecked(coord.normalize(self.size))
    }
    pub fn get_index_checked(&self, index: usize) -> &T {
        self.cells.index(index)
    }
    pub fn get_index_checked_mut(&mut self, index: usize) -> &mut T {
        self.cells.index_mut(index)
    }
    pub fn get_checked(&self, coord: ICoord) -> &T {
        self.cells.index(self.index_of_coord_checked(coord))
    }
    pub fn get_checked_mut(&mut self, coord: ICoord) -> &mut T {
        let index = self.index_of_coord_checked(coord);
        self.cells.index_mut(index)
    }
    pub fn enumerate(&self) -> GridEnumerate<'_, T> {
        self.coord_iter().zip(self.iter())
    }
    pub fn enumerate_mut(&mut self) -> GridEnumerateMut<'_, T> {
        self.coord_iter().zip(self.iter_mut())
    }
    pub fn into_enumerate(self) -> GridIntoEnumerate<T> {
        self.coord_iter().zip(self.cells)
    }
    pub fn rows(&self) -> GridRows<'_, T> {
        self.cells.chunks(self.size.width() as usize)
    }
    pub fn rows_mut(&mut self) -> GridRowsMut<'_, T> {
        self.cells.chunks_mut(self.size.width() as usize)
    }
    pub fn get2_mut(&mut self, a: ICoord, b: ICoord) -> Result<(&mut T, &mut T), Get2Error> {
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
    pub fn get2_checked_mut(&mut self, a: ICoord, b: ICoord) -> (&mut T, &mut T) {
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
    pub fn raw(&self) -> &[T] {
        &self.cells
    }
    pub fn raw_mut(&mut self) -> &mut [T] {
        &mut self.cells
    }
    pub fn map<U, F: FnMut(T) -> U>(self, f: F) -> Grid<U> {
        Grid::new_grid_map(self, f)
    }
    pub fn map_with_coord<U, F: FnMut(ICoord, T) -> U>(self, f: F) -> Grid<U> {
        Grid::new_grid_map_with_coord(self, f)
    }
    pub fn map_ref<U, F: FnMut(&T) -> U>(&self, f: F) -> Grid<U> {
        Grid::new_grid_map_ref(self, f)
    }
    pub fn map_ref_with_coord<U, F: FnMut(ICoord, &T) -> U>(&self, f: F) -> Grid<U> {
        Grid::new_grid_map_ref_with_coord(self, f)
    }
    pub fn is_on_edge(&self, coord: ICoord) -> bool {
        self.size.is_on_edge(coord)
    }
    pub fn edge_coord_iter(&self) -> EdgeICoordIter {
        self.size.edge_iter()
    }
    pub fn edge_iter(&self) -> GridEdgeIter<'_, T> {
        GridEdgeIter {
            edge_coord_iter: self.edge_coord_iter(),
            grid: self,
        }
    }
    pub fn edge_iter_mut(&mut self) -> GridEdgeIterMut<'_, T> {
        GridEdgeIterMut {
            edge_coord_iter: self.edge_coord_iter(),
            grid: self,
            marker: PhantomData,
        }
    }
    pub fn edge_enumerate(&self) -> GridEdgeEnumerate<'_, T> {
        self.edge_coord_iter().zip(self.edge_iter())
    }
    pub fn edge_enumerate_mut(&mut self) -> GridEdgeEnumerateMut<'_, T> {
        self.edge_coord_iter().zip(self.edge_iter_mut())
    }
}

pub struct GridEdgeIter<'a, T> {
    edge_coord_iter: EdgeICoordIter,
    grid: &'a Grid<T>,
}

impl<'a, T> Iterator for GridEdgeIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.edge_coord_iter
            .next()
            .map(|coord| self.grid.get_checked(coord))
    }
}

pub struct GridEdgeIterMut<'a, T> {
    edge_coord_iter: EdgeICoordIter,
    grid: *mut Grid<T>,
    marker: PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for GridEdgeIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(coord) = self.edge_coord_iter.next() {
            // SAFETY: The compiler doesn't know that the `EdgeICoordIter` iterator only visits each
            // coordinate at most once. If this iterator yielded the same coordinate twice, then
            // the `EdgeIterMut` iterator could give out multiple mutable references to the same
            // cell of the grid.
            let value = unsafe { (&mut *self.grid).get_checked_mut(coord) };
            Some(value)
        } else {
            None
        }
    }
}

impl<T> Index<ICoord> for Grid<T> {
    type Output = T;

    fn index(&self, index: ICoord) -> &Self::Output {
        self.get_checked(index)
    }
}

impl<T> IndexMut<ICoord> for Grid<T> {
    fn index_mut(&mut self, index: ICoord) -> &mut Self::Output {
        self.get_checked_mut(index)
    }
}

impl<T> Index<UCoord> for Grid<T> {
    type Output = T;

    fn index(&self, index: UCoord) -> &Self::Output {
        self.get_checked(index.to_icoord())
    }
}

impl<T> IndexMut<UCoord> for Grid<T> {
    fn index_mut(&mut self, index: UCoord) -> &mut Self::Output {
        self.get_checked_mut(index.to_icoord())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    fn coord_grid(size: UCoord) -> Grid<ICoord> {
        Grid::new_fn(size, |coord| coord)
    }

    #[test]
    #[should_panic]
    fn out_of_bounds() {
        let grid = coord_grid(UCoord::new(2, 3));
        grid.get_checked(ICoord::new(0, 3));
    }

    #[test]
    fn tiling() {
        let mut grid = coord_grid(UCoord::new(2, 3));
        assert_eq!(*grid.get_tiled(ICoord::new(-10, -30)), ICoord::new(0, 0));
        *grid.get_tiled_mut(ICoord::new(-12, -12)) = ICoord::new(1000, 1000);
        assert_eq!(
            *grid.get_tiled(ICoord::new(10, 30)),
            ICoord::new(1000, 1000)
        );
    }

    #[test]
    fn enumerate() {
        let mut grid = coord_grid(UCoord::new(24, 42));
        grid.enumerate()
            .for_each(|(coord, cell)| assert_eq!(coord, *cell));
        grid.enumerate_mut()
            .for_each(|(coord, cell)| *cell = coord * 3);
        grid.enumerate()
            .for_each(|(coord, cell)| assert_eq!(coord * 3, *cell));
    }

    #[test]
    fn index() {
        let mut grid = coord_grid(UCoord::new(7, 9));
        let index = grid.index_of_coord(ICoord::new(5, 3)).unwrap();
        assert_eq!(index, 26);
        *grid.get_index_checked_mut(index) = ICoord::new(1000, 1000);
        assert_eq!(*grid.get_index_checked(index), ICoord::new(1000, 1000));
    }

    #[test]
    fn get2_checked_mut() {
        let mut grid = coord_grid(UCoord::new(4, 4));
        let (a, b) = grid.get2_checked_mut(ICoord::new(1, 2), ICoord::new(2, 1));
        mem::swap(a, b);
        assert_eq!(*grid.get_checked(ICoord::new(1, 2)), ICoord::new(2, 1));
        assert_eq!(*grid.get_checked(ICoord::new(2, 1)), ICoord::new(1, 2));
    }

    #[test]
    #[should_panic]
    fn get2_checked_mut_equal_coords() {
        let mut grid = coord_grid(UCoord::new(4, 4));
        let (_, _) = grid.get2_checked_mut(ICoord::new(1, 2), ICoord::new(1, 2));
    }
}
