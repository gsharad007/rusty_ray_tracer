use std::ops::IndexMut;

use super::matrix::{Identity, Matrix};

pub trait Shearing<const ROW: usize, const COL: usize, T> {
    #[must_use]
    fn shearing(x_y: T, x_z: T, y_x: T, y_z: T, z_x: T, z_y: T) -> Self;
}

impl<const ROW: usize, const COL: usize, T> Shearing<ROW, COL, T> for Matrix<ROW, COL, T>
where
    Self: Sized + Identity<COL, T> + IndexMut<(usize, usize), Output = T>,
    T: Default + Copy + std::convert::From<i8>,
{
    #[must_use]
    fn shearing(x_y: T, x_z: T, y_x: T, y_z: T, z_x: T, z_y: T) -> Self {
        let mut matrix = Self::identity();

        matrix[(0, 1)] = x_y;
        matrix[(0, 2)] = x_z;
        matrix[(1, 0)] = y_x;
        matrix[(1, 2)] = y_z;
        matrix[(2, 0)] = z_x;
        matrix[(2, 1)] = z_y;

        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core3d::matrix::Matrix;
    use crate::core3d::matrix_transforms::Transform;
    use crate::core3d::point::Point;

    #[test]
    fn test_shearing() {
        let m = Matrix::<4, 4, f32>::shearing(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);

        let p = Point::new(2.0, 3.0, 4.0);
        let result = m.transform(p);
        let expected = Point::new(
            2.0 + (1.0 * 3.0) + (2.0 * 4.0),
            (3.0 * 2.0) + 3.0 + (4.0 * 4.0),
            (5.0 * 2.0) + (6.0 * 3.0) + 4.0,
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_shearing_identity() {
        let m = Matrix::<4, 4, f32>::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(m.transform(p), p);
    }
}
