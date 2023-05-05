use std::ops::{Add, Index, IndexMut};

use super::matrix::{Identity, Matrix};

use std::ops::Mul;

pub trait Transform<const ROW: usize, const COL: usize, T, Type> {
    fn transform(&self, tuple: Type) -> Type;
}

impl<const ROW: usize, const COL: usize, T, Type> Transform<ROW, COL, T, Type>
    for Matrix<ROW, COL, T>
where
    Self: Sized + Index<(usize, usize), Output = T>,
    T: Copy + Default + Mul<T, Output = T> + Add<T, Output = T>,
    Type: Default + Index<usize, Output = T> + IndexMut<usize>,
{
    fn transform(&self, tuple: Type) -> Type {
        (0..COL).fold(Type::default(), |mut acc, c| {
            acc[c] = (0..ROW).fold(T::default(), |f, r| f + self[(r, c)] * tuple[r]);
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core3d::{point::Point, vector::Vector};

    #[test]
    fn test_transform_point() {
        let m = Matrix::<4, 4, f32>::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [1.0, 2.0, 3.0, 1.0],
        ]);

        let p = Point::new(1.0, 2.0, 3.0);
        let expected = Point::new(2.0, 4.0, 6.0);
        let result = Transform::transform(&m, p);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_transform_vector() {
        let m = Matrix::<4, 4, f32>::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [1.0, 2.0, 3.0, 1.0],
        ]);

        let p = Vector::new(1.0, 2.0, 3.0);
        let expected = Vector::new(1.0, 2.0, 3.0);
        let result = Transform::transform(&m, p);

        assert_eq!(result, expected);
    }
}

pub trait Translation<const ROW: usize, const COL: usize, T>
where
    Self: Sized + Identity<COL, T> + IndexMut<(usize, usize), Output = T>,
    T: Default + Copy + std::convert::From<i8>,
{
    #[must_use]
    fn translation(x: T, y: T, z: T) -> Self {
        let mut matrix = Self::identity();

        if COL > 0 {
            matrix[(ROW - 1, 0)] = x;
        }
        if COL > 1 {
            matrix[(ROW - 1, 1)] = y;
        }
        if COL > 2 {
            matrix[(ROW - 1, 2)] = z;
        }

        matrix
    }
}

#[cfg(test)]
mod tests_translation {
    use super::*;
    use crate::core3d::point::Point;

    #[test]
    fn test_translation() {
        let m = Matrix::<4, 4, f32>::translation(2.0, 3.0, 4.0);

        let p1 = Point::new(1.0, 2.0, 3.0);

        let p1_translated = m.transform(p1);

        assert_eq!(p1_translated, Point::new(3.0, 5.0, 7.0));
    }
}

impl Translation<4, 4, f32> for Matrix<4, 4, f32> {}
impl Translation<3, 3, f32> for Matrix<3, 3, f32> {}
impl Translation<2, 2, f32> for Matrix<2, 2, f32> {}
