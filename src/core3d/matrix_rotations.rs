use std::ops::IndexMut;

use super::matrix::{Identity, Matrix};

pub trait Rotations<const ROW: usize, const COL: usize, T> {
    #[must_use]
    fn rotation_around_x_axis(radians: T) -> Self;
    #[must_use]
    fn rotation_around_y_axis(radians: T) -> Self;
    #[must_use]
    fn rotation_around_z_axis(radians: T) -> Self;
}

impl<const ROW: usize, const COL: usize, T> Rotations<ROW, COL, T> for Matrix<ROW, COL, T>
where
    Self: Sized + Identity<COL, T> + IndexMut<(usize, usize), Output = T>,
    T: Default + Copy + std::convert::From<i8> + num::Float,
{
    fn rotation_around_x_axis(radians: T) -> Self {
        assert!(ROW > 2 && COL > 2);

        let mut matrix = Self::identity();

        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        matrix[(1, 1)] = cos_theta;
        matrix[(1, 2)] = -sin_theta;
        matrix[(2, 1)] = sin_theta;
        matrix[(2, 2)] = cos_theta;

        matrix
    }

    fn rotation_around_y_axis(radians: T) -> Self {
        assert!(ROW > 2 && COL > 2);

        let mut matrix = Self::identity();

        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        matrix[(0, 0)] = cos_theta;
        matrix[(0, 2)] = sin_theta;
        matrix[(2, 0)] = -sin_theta;
        matrix[(2, 2)] = cos_theta;

        matrix
    }

    fn rotation_around_z_axis(radians: T) -> Self {
        assert!(ROW > 1 && COL > 1);

        let mut matrix = Self::identity();

        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        matrix[(0, 0)] = cos_theta;
        matrix[(0, 1)] = -sin_theta;
        matrix[(1, 0)] = sin_theta;
        matrix[(1, 1)] = cos_theta;

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
    fn test_rotation_around_x_axis() {
        let angle = 90_f32.to_radians();
        let m = Matrix::<4, 4, f32>::rotation_around_x_axis(angle);
        let p = Point::new(0.0, 1.0, 0.0);
        let expected = Point::new(0.0, 0.0, 1.0);

        let result = m.transform(p);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_rotation_around_y_axis() {
        let angle = 90_f32.to_radians();
        let matrix = Matrix::<4, 4, f32>::rotation_around_y_axis(angle);
        let point = Point::new(1.0, 0.0, 0.0);
        let expected = Point::new(0.0, 0.0, -1.0);

        let result = matrix.transform(point);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_rotation_around_z_axis() {
        let angle = 90_f32.to_radians();
        let matrix = Matrix::<4, 4, f32>::rotation_around_z_axis(angle);
        let point = Point::new(1.0, 0.0, 0.0);
        let expected = Point::new(0.0, 1.0, 0.0);

        let result = matrix.transform(point);

        assert_eq!(result, expected);
    }
}
