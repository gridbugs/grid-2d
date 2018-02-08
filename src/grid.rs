use std::slice;
use coord::*;

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
        self.coords.next().and_then(
            |c| self.iter.next().map(|t| (c, t)),
        )
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
        self.coords.next().and_then(
            |c| self.iter.next().map(|t| (c, t)),
        )
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    size: Size,
    cells: Vec<T>,
}

impl<T> Grid<T> {
    fn new_uninitialised(width: u32, height: u32) -> Self {
        Self {
            size: Size::new(width, height),
            cells: Vec::with_capacity((width * height) as usize),
        }
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
        self.coord_to_index(coord).and_then(
            |index| self.cells.get(index),
        )
    }

    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        if let Some(index) = self.coord_to_index(coord) {
            self.cells.get_mut(index)
        } else {
            None
        }
    }
}

impl<T: Copy> Grid<T> {
    pub fn new_copy(width: u32, height: u32, value: T) -> Self {
        let mut grid = Grid::new_uninitialised(width, height);
        let size = grid.len();
        grid.cells.resize(size, value);
        grid
    }
}

impl<T: Default> Grid<T> {
    pub fn new_default(width: u32, height: u32) -> Self {
        let mut grid = Grid::new_uninitialised(width, height);
        let size = grid.len();
        for _ in 0..size {
            grid.cells.push(Default::default());
        }
        grid
    }
}

impl<T: From<Coord>> Grid<T> {
    pub fn new_from_coord(width: u32, height: u32) -> Self {
        let mut grid = Grid::new_uninitialised(width, height);
        for i in 0..height {
            for j in 0..width {
                let coord = Coord::new(j as i32, i as i32);
                grid.cells.push(From::from(coord));
            }
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
