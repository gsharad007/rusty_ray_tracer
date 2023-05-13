use std::fmt::Debug;
use std::ops::IndexMut;

use super::matrix::{Identity, Matrix};

/// Trait for matrix translation operations.
pub trait Translation<const ROW: usize, const COL: usize, T> {
    /// Creates a translation matrix for the specified translation values.
    ///
    /// # Arguments
    ///
    /// * `x` - The translation along the x-axis.
    /// * `y` - The translation along the y-axis.
    /// * `z` - The translation along the z-axis.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rusty_ray_tracer::core3d::matrix::Matrix;
    /// # use rusty_ray_tracer::core3d::matrix_translations::Translation;
    ///
    /// let translation_matrix = Matrix::<4, 4, f32>::translation(2.0, 3.0, -1.0);
    /// ```
    ///
    /// ```
    /// # use rusty_ray_tracer::core3d::matrix::Matrix;
    /// # use rusty_ray_tracer::core3d::point::Point;
    /// # use rusty_ray_tracer::core3d::matrix_translations::Translation;
    /// # use rusty_ray_tracer::core3d::matrix_transforms::Transform;
    ///
    /// let m = Matrix::<4, 4, f32>::translation(2.0, 3.0, 4.0);
    /// let p = Point::new(1.0, 2.0, 3.0);
    /// let expected = Point::new(3.0, 5.0, 7.0);
    ///
    /// assert_eq!(m.transform(p), expected);
    ///
    /// let m = Matrix::<4, 4, f32>::translation(-2.0, -3.0, -4.0);
    /// let expected = Point::new(-1.0, -1.0, -1.0);
    ///
    /// assert_eq!(m.transform(p), expected);
    /// ```
    #[must_use]
    fn translation(x: T, y: T, z: T) -> Self;
}

impl<const ROW: usize, const COL: usize, T> Translation<ROW, COL, T> for Matrix<ROW, COL, T>
where
    Self: Sized + Identity<COL, T> + IndexMut<(usize, usize), Output = T>,
    T: Default + Copy + std::convert::From<i8> + PartialEq + Debug,
{
    #[must_use]
    fn translation(x: T, y: T, z: T) -> Self {
        let mut matrix = Self::identity();

        if ROW > 0 {
            matrix[(0, COL - 1)] = x;
        } else {
            assert_eq!(x, T::default());
        }
        if ROW > 1 {
            matrix[(1, COL - 1)] = y;
        } else {
            assert_eq!(y, T::default());
        }
        if ROW > 2 {
            matrix[(2, COL - 1)] = z;
        } else {
            assert_eq!(z, T::default());
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

    #[test]
    fn test_translation_identity() {
        let identity_matrix = Matrix::<4, 4, f32>::identity();
        let translation_matrix = Matrix::<4, 4, f32>::translation(0.0, 0.0, 0.0);

        assert_eq!(translation_matrix, identity_matrix);
    }
}
