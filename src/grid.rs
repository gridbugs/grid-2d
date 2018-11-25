use coord_2d::*;
use coord_system::*;
use std::iter;
use std::slice;

pub type GridIter<'a, T> = slice::Iter<'a, T>;
pub type GridIterMut<'a, T> = slice::IterMut<'a, T>;
pub type GridEnumerate<'a, C, T> = iter::Zip<C, GridIter<'a, T>>;
pub type GridEnumerateMut<'a, C, T> = iter::Zip<C, GridIterMut<'a, T>>;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Grid<T, C: CoordSystem = XThenY> {
    coord_system: C,
    cells: Vec<T>,
}

impl<T, C: CoordSystem + From<Size>> Grid<T, C> {
    fn new_uninitialised(size: Size) -> Self {
        Self {
            coord_system: C::from(size),
            cells: Vec::with_capacity(size.count()),
        }
    }

    pub fn new_fn<F>(size: Size, f: F) -> Self
    where
        F: Fn(Coord) -> T,
    {
        let mut grid: Self = Grid::new_uninitialised(size);
        for coord in grid.coord_system.coord_iter() {
            grid.cells.push(f(coord));
        }
        grid
    }
}
impl<T: Clone, C: CoordSystem + From<Size>> Grid<T, C> {
    pub fn new_clone(size: Size, value: T) -> Self {
        let mut grid = Grid::new_uninitialised(size);
        grid.cells.resize(size.count(), value);
        grid
    }
}

impl<T: Default, C: CoordSystem + From<Size>> Grid<T, C> {
    pub fn new_default(size: Size) -> Self {
        let mut grid = Grid::new_uninitialised(size);
        for _ in 0..size.count() {
            grid.cells.push(Default::default());
        }
        grid
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
            .index_valid(coord)
            .map(|index| &self.cells[index])
    }
    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        self.coord_system
            .index_valid(coord)
            .map(move |index| &mut self.cells[index])
    }
    pub fn tiled_get(&self, coord: Coord) -> &T {
        &self.cells[self.coord_system.index_normalized(coord)]
    }
    pub fn tiled_get_mut(&mut self, coord: Coord) -> &mut T {
        let index = self.coord_system.index_normalized(coord);
        &mut self.cells[index]
    }
    pub fn enumerate(&self) -> GridEnumerate<C::CoordIter, T> {
        self.coord_iter().zip(self.iter())
    }
    pub fn enumerate_mut(&mut self) -> GridEnumerateMut<C::CoordIter, T> {
        self.coord_iter().zip(self.iter_mut())
    }
}
