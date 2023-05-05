use std::ops::{Add, Index, IndexMut};

use super::matrix::Matrix;

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
