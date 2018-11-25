use coord::{Coord, Size};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

impl Add for Coord {
    type Output = Coord;
    fn add(self, Coord { x, y }: Coord) -> Self::Output {
        Coord {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl<'a> Add<Coord> for &'a Coord {
    type Output = Coord;
    fn add(self, Coord { x, y }: Coord) -> Self::Output {
        Coord {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl<'a> Add<&'a Coord> for Coord {
    type Output = Coord;
    fn add(self, &Coord { x, y }: &'a Coord) -> Self::Output {
        Coord {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl<'a, 'b> Add<&'a Coord> for &'b Coord {
    type Output = Coord;
    fn add(self, &Coord { x, y }: &'a Coord) -> Self::Output {
        Coord {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl Add<Size> for Coord {
    type Output = Coord;
    fn add(self, size: Size) -> Self::Output {
        Coord {
            x: self.x + size.x() as i32,
            y: self.y + size.y() as i32,
        }
    }
}

impl<'a> Add<Size> for &'a Coord {
    type Output = Coord;
    fn add(self, size: Size) -> Self::Output {
        Coord {
            x: self.x + size.x() as i32,
            y: self.y + size.y() as i32,
        }
    }
}

impl<'a> Add<&'a Size> for Coord {
    type Output = Coord;
    fn add(self, size: &'a Size) -> Self::Output {
        Coord {
            x: self.x + size.x() as i32,
            y: self.y + size.y() as i32,
        }
    }
}

impl<'a, 'b> Add<&'a Size> for &'b Coord {
    type Output = Coord;
    fn add(self, size: &'a Size) -> Self::Output {
        Coord {
            x: self.x + size.x() as i32,
            y: self.y + size.y() as i32,
        }
    }
}

impl Add<Coord> for Size {
    type Output = Coord;
    fn add(self, Coord { x, y }: Coord) -> Self::Output {
        Coord {
            x: self.x() as i32 + x,
            y: self.y() as i32 + y,
        }
    }
}

impl<'a> Add<Coord> for &'a Size {
    type Output = Coord;
    fn add(self, Coord { x, y }: Coord) -> Self::Output {
        Coord {
            x: self.x() as i32 + x,
            y: self.y() as i32 + y,
        }
    }
}

impl<'a> Add<&'a Coord> for Size {
    type Output = Coord;
    fn add(self, &Coord { x, y }: &'a Coord) -> Self::Output {
        Coord {
            x: self.x() as i32 + x,
            y: self.y() as i32 + y,
        }
    }
}

impl<'a, 'b> Add<&'a Coord> for &'b Size {
    type Output = Coord;
    fn add(self, &Coord { x, y }: &'a Coord) -> Self::Output {
        Coord {
            x: self.x() as i32 + x,
            y: self.y() as i32 + y,
        }
    }
}

impl Add for Size {
    type Output = Size;
    fn add(self, size: Size) -> Self::Output {
        Size::new(self.x() + size.x(), self.y() + size.y())
    }
}

impl<'a> Add<Size> for &'a Size {
    type Output = Size;
    fn add(self, size: Size) -> Self::Output {
        Size::new(self.x() + size.x(), self.y() + size.y())
    }
}

impl<'a> Add<&'a Size> for Size {
    type Output = Size;
    fn add(self, size: &'a Size) -> Self::Output {
        Size::new(self.x() + size.x(), self.y() + size.y())
    }
}

impl<'a, 'b> Add<&'a Size> for &'b Size {
    type Output = Size;
    fn add(self, size: &'a Size) -> Self::Output {
        Size::new(self.x() + size.x(), self.y() + size.y())
    }
}

impl<T> AddAssign<T> for Coord
where
    Coord: Add<T, Output = Coord>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

impl Sub for Coord {
    type Output = Coord;
    fn sub(self, Coord { x, y }: Coord) -> Self::Output {
        Coord {
            x: self.x - x,
            y: self.y - y,
        }
    }
}

impl<'a> Sub<Coord> for &'a Coord {
    type Output = Coord;
    fn sub(self, Coord { x, y }: Coord) -> Self::Output {
        Coord {
            x: self.x - x,
            y: self.y - y,
        }
    }
}

impl<'a> Sub<&'a Coord> for Coord {
    type Output = Coord;
    fn sub(self, &Coord { x, y }: &'a Coord) -> Self::Output {
        Coord {
            x: self.x - x,
            y: self.y - y,
        }
    }
}

impl<'a, 'b> Sub<&'a Coord> for &'b Coord {
    type Output = Coord;
    fn sub(self, &Coord { x, y }: &'a Coord) -> Self::Output {
        Coord {
            x: self.x - x,
            y: self.y - y,
        }
    }
}

impl Sub<Size> for Coord {
    type Output = Coord;
    fn sub(self, size: Size) -> Self::Output {
        Coord {
            x: self.x - size.x() as i32,
            y: self.y - size.y() as i32,
        }
    }
}

impl<'a> Sub<Size> for &'a Coord {
    type Output = Coord;
    fn sub(self, size: Size) -> Self::Output {
        Coord {
            x: self.x - size.x() as i32,
            y: self.y - size.y() as i32,
        }
    }
}

impl<'a> Sub<&'a Size> for Coord {
    type Output = Coord;
    fn sub(self, size: &'a Size) -> Self::Output {
        Coord {
            x: self.x - size.x() as i32,
            y: self.y - size.y() as i32,
        }
    }
}

impl<'a, 'b> Sub<&'a Size> for &'b Coord {
    type Output = Coord;
    fn sub(self, size: &'a Size) -> Self::Output {
        Coord {
            x: self.x - size.x() as i32,
            y: self.y - size.y() as i32,
        }
    }
}

impl Sub<Coord> for Size {
    type Output = Coord;
    fn sub(self, Coord { x, y }: Coord) -> Self::Output {
        Coord {
            x: self.x() as i32 - x,
            y: self.y() as i32 - y,
        }
    }
}

impl<'a> Sub<Coord> for &'a Size {
    type Output = Coord;
    fn sub(self, Coord { x, y }: Coord) -> Self::Output {
        Coord {
            x: self.x() as i32 - x,
            y: self.y() as i32 - y,
        }
    }
}

impl<'a> Sub<&'a Coord> for Size {
    type Output = Coord;
    fn sub(self, &Coord { x, y }: &'a Coord) -> Self::Output {
        Coord {
            x: self.x() as i32 - x,
            y: self.y() as i32 - y,
        }
    }
}

impl<'a, 'b> Sub<&'a Coord> for &'b Size {
    type Output = Coord;
    fn sub(self, &Coord { x, y }: &'a Coord) -> Self::Output {
        Coord {
            x: self.x() as i32 - x,
            y: self.y() as i32 - y,
        }
    }
}

impl Sub for Size {
    type Output = Size;
    fn sub(self, size: Size) -> Self::Output {
        Size::new(self.x() - size.x(), self.y() - size.y())
    }
}

impl<'a> Sub<Size> for &'a Size {
    type Output = Size;
    fn sub(self, size: Size) -> Self::Output {
        Size::new(self.x() - size.x(), self.y() - size.y())
    }
}

impl<'a> Sub<&'a Size> for Size {
    type Output = Size;
    fn sub(self, size: &'a Size) -> Self::Output {
        Size::new(self.x() - size.x(), self.y() - size.y())
    }
}

impl<'a, 'b> Sub<&'a Size> for &'b Size {
    type Output = Size;
    fn sub(self, size: &'a Size) -> Self::Output {
        Size::new(self.x() - size.x(), self.y() - size.y())
    }
}

impl<T> SubAssign<T> for Coord
where
    Coord: Sub<T, Output = Coord>,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

impl Mul<i32> for Coord {
    type Output = Coord;
    fn mul(self, rhs: i32) -> Self::Output {
        Coord {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<'a> Mul<i32> for &'a Coord {
    type Output = Coord;
    fn mul(self, rhs: i32) -> Self::Output {
        Coord {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> MulAssign<T> for Coord
where
    Coord: Mul<T, Output = Coord>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl Mul<u32> for Size {
    type Output = Size;
    fn mul(self, rhs: u32) -> Self::Output {
        Size::new(self.x() * rhs, self.y() * rhs)
    }
}

impl<'a> Mul<u32> for &'a Size {
    type Output = Size;
    fn mul(self, rhs: u32) -> Self::Output {
        Size::new(self.x() * rhs, self.y() * rhs)
    }
}

impl<T> MulAssign<T> for Size
where
    Size: Mul<T, Output = Size>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl Div<i32> for Coord {
    type Output = Coord;
    fn div(self, rhs: i32) -> Self::Output {
        Coord {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<'a> Div<i32> for &'a Coord {
    type Output = Coord;
    fn div(self, rhs: i32) -> Self::Output {
        Coord {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> DivAssign<T> for Coord
where
    Coord: Div<T, Output = Coord>,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl Div<u32> for Size {
    type Output = Size;
    fn div(self, rhs: u32) -> Self::Output {
        Size::new(self.x() / rhs, self.y() / rhs)
    }
}

impl<'a> Div<u32> for &'a Size {
    type Output = Size;
    fn div(self, rhs: u32) -> Self::Output {
        Size::new(self.x() / rhs, self.y() / rhs)
    }
}

impl<T> DivAssign<T> for Size
where
    Size: Div<T, Output = Size>,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod test {
    use coord::{Coord, Size};

    #[test]
    fn arithmetic() {
        let mut a = Coord::new(0, 0);
        let _ = a + Coord::new(0, 0);
        let _ = &a + Coord::new(0, 0);
        let _ = a + &Coord::new(0, 0);
        let _ = &a + &Coord::new(0, 0);
        let _ = a + Size::new(0, 0);
        let _ = &a + Size::new(0, 0);
        let _ = a + &Size::new(0, 0);
        let _ = &a + &Size::new(0, 0);
        let _ = Size::new(0, 0) + a;
        let _ = &Size::new(0, 0) + a;
        let _ = Size::new(0, 0) + &a;
        let _ = &Size::new(0, 0) + &a;
        let _ = a += Size::new(0, 0);
        let _ = a += Coord::new(0, 0);
        let _ = a += &Size::new(0, 0);
        let _ = a += &Coord::new(0, 0);
    }
}
