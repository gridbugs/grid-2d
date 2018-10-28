use std::slice;
use crate::coord::*;

pub type Iter<'a, T> = slice::Iter<'a, T>;
pub type IterMut<'a, T> = slice::IterMut<'a, T>;

pub struct CoordEnumerate<'a, T: 'a> {
    coords: CoordIter,
    iter: Iter<'a, T>,
}

impl<'a, T> CoordEnumerate<'a, T> {
    fn new(coords: CoordIter, iter: Iter<'a, T>) -> Self {
        Self { coords, iter }
    }
}

impl<'a, T> Iterator for CoordEnumerate<'a, T> {
    type Item = (Coord, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.coords
            .next()
            .and_then(|c| self.iter.next().map(|t| (c, t)))
    }
}

pub struct CoordEnumerateMut<'a, T: 'a> {
    coords: CoordIter,
    iter: IterMut<'a, T>,
}

impl<'a, T> CoordEnumerateMut<'a, T> {
    fn new(coords: CoordIter, iter: IterMut<'a, T>) -> Self {
        Self { coords, iter }
    }
}

impl<'a, T> Iterator for CoordEnumerateMut<'a, T> {
    type Item = (Coord, &'a mut T);
    fn next(&mut self) -> Option<Self::Item> {
        self.coords
            .next()
            .and_then(|c| self.iter.next().map(|t| (c, t)))
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Grid<T> {
    size: Size,
    cells: Vec<T>,
}

impl<T> Grid<T> {
    fn new_uninitialised(size: Size) -> Self {
        Self {
            size,
            cells: Vec::with_capacity(size.count()),
        }
    }

    pub fn new_from_fn<F>(size: Size, f: F) -> Self
    where
        F: Fn(Coord) -> T,
    {
        let mut grid = Grid::new_uninitialised(size);
        for coord in size.coords() {
            grid.cells.push(f(coord));
        }
        grid
    }

    pub fn width(&self) -> u32 {
        self.size.x()
    }

    pub fn height(&self) -> u32 {
        self.size.y()
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn len(&self) -> usize {
        self.size.count()
    }

    pub fn iter(&self) -> Iter<T> {
        self.cells.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.cells.iter_mut()
    }

    pub fn coords(&self) -> CoordIter {
        self.size.coords()
    }

    pub fn enumerate(&self) -> CoordEnumerate<T> {
        CoordEnumerate::new(self.coords(), self.iter())
    }

    pub fn enumerate_mut(&mut self) -> CoordEnumerateMut<T> {
        CoordEnumerateMut::new(self.coords(), self.iter_mut())
    }

    pub fn coord_to_index(&self, coord: Coord) -> Option<usize> {
        self.size.index(coord)
    }

    pub fn get(&self, coord: Coord) -> Option<&T> {
        self.coord_to_index(coord)
            .and_then(|index| self.cells.get(index))
    }

    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        if let Some(index) = self.coord_to_index(coord) {
            self.cells.get_mut(index)
        } else {
            None
        }
    }
}

impl<T: Clone> Grid<T> {
    pub fn new_clone(size: Size, value: T) -> Self {
        let mut grid = Grid::new_uninitialised(size);
        grid.cells.resize(size.count(), value);
        grid
    }
    pub fn reset_clone(&mut self, value: T) {
        self.cells.clear();
        self.cells.resize(self.size.count(), value);
    }
    pub fn resize_clone(&mut self, size: Size, value: T) {
        self.cells.clear();
        self.cells.resize(size.count(), value);
        self.size = size;
    }
}

impl<T: Default> Grid<T> {
    pub fn new_default(size: Size) -> Self {
        let mut grid = Grid::new_uninitialised(size);
        for _ in 0..size.count() {
            grid.cells.push(Default::default());
        }
        grid
    }
}

impl<T: Default + Clone> Grid<T> {
    pub fn reset_default(&mut self) {
        self.cells.clear();
        self.cells.resize(self.size.count(), Default::default());
    }
    pub fn resize_default(&mut self, size: Size) {
        self.cells.clear();
        self.cells.resize(size.count(), Default::default());
        self.size = size;
    }
}

impl<T: From<Coord>> Grid<T> {
    pub fn new_from_coord(size: Size) -> Self {
        let mut grid = Grid::new_uninitialised(size);
        for coord in size.coords() {
            grid.cells.push(From::from(coord));
        }
        grid
    }
}

impl<T> ::std::ops::Index<usize> for Grid<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.cells.index(index)
    }
}

impl<T> ::std::ops::IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.cells.index_mut(index)
    }
}
