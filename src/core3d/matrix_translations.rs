use std::ops::IndexMut;

use super::matrix::{Identity, Matrix};

pub trait Translation<const ROW: usize, const COL: usize, T> {
    #[must_use]
    fn translation(x: T, y: T, z: T) -> Self;
}

impl<const ROW: usize, const COL: usize, T> Translation<ROW, COL, T> for Matrix<ROW, COL, T>
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
mod tests {
    use super::*;
    use crate::core3d::matrix_transforms::Transform;
    use crate::core3d::{point::Point, vector::Vector};

    #[test]
    fn test_translation() {
        let m = Matrix::<4, 4, f32>::translation(2.0, 3.0, 4.0);
        let p = Point::new(1.0, 2.0, 3.0);
        let expected = Point::new(3.0, 5.0, 7.0);

        assert_eq!(m.transform(p), expected);
    }

    #[test]
    fn test_translation_negative() {
        let m = Matrix::<4, 4, f32>::translation(-2.0, 3.0, -4.0);
        let p = Point::new(1.0, 2.0, 3.0);
        let expected = Point::new(-1.0, 5.0, -1.0);

        assert_eq!(m.transform(p), expected);
    }

    #[test]
    fn test_translation_vector() {
        let m = Matrix::<4, 4, f32>::translation(-2.0, 3.0, -4.0);
        let p = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(m.transform(p), p);
    }
}
