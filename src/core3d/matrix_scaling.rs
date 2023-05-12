use std::ops::IndexMut;

use super::matrix::{Identity, Matrix};

/// Trait for scaling transformations on matrices.
pub trait Scaling<const ROW: usize, const COL: usize, T> {
    /// Creates a scaling matrix with the given scaling factors.
    ///
    /// # Arguments
    ///
    /// * `x` - The scaling factor along the x-axis.
    /// * `y` - The scaling factor along the y-axis.
    /// * `z` - The scaling factor along the z-axis.
    ///
    /// # Returns
    ///
    /// A scaling matrix with the given scaling factors applied.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rusty_ray_tracer::core3d::matrix::Matrix;
    /// # use rusty_ray_tracer::core3d::matrix_scaling::Scaling;
    ///
    /// let scaling_matrix = Matrix::<4, 4, f32>::scaling(2.0, 3.0, 1.5);
    ///
    /// // The resulting matrix should have the scaling factors applied diagonally.
    /// assert_eq!(scaling_matrix, Matrix::new([
    ///     [2.0, 0.0, 0.0, 0.0],
    ///     [0.0, 3.0, 0.0, 0.0],
    ///     [0.0, 0.0, 1.5, 0.0],
    ///     [0.0, 0.0, 0.0, 1.0],
    /// ]));
    /// ```
    #[must_use]
    fn scaling(x: T, y: T, z: T) -> Self;
}

impl<const ROW: usize, const COL: usize, T> Scaling<ROW, COL, T> for Matrix<ROW, COL, T>
where
    Self: Sized + Identity<COL, T> + IndexMut<(usize, usize), Output = T>,
    T: Default + Copy + std::convert::From<i8>,
{
    #[must_use]
    fn scaling(x: T, y: T, z: T) -> Self {
        let mut matrix = Self::identity();

        if ROW > 0 && COL > 0 {
            matrix[(0, 0)] = x;
        }
        if ROW > 1 && COL > 1 {
            matrix[(1, 1)] = y;
        }
        if ROW > 2 && COL > 2 {
            matrix[(2, 2)] = z;
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
    fn test_scaling() {
        let m = Matrix::<4, 4, f32>::scaling(2.0, 3.0, 4.0);
        let p = Point::new(1.0, 2.0, 3.0);
        let expected = Point::new(2.0, 6.0, 12.0);

        assert_eq!(m.transform(p), expected);
    }

    #[test]
    fn test_scaling_negative() {
        let m = Matrix::<4, 4, f32>::scaling(-2.0, 3.0, -4.0);
        let p = Point::new(1.0, 2.0, 3.0);
        let expected = Point::new(-2.0, 6.0, -12.0);

        assert_eq!(m.transform(p), expected);
    }

    #[test]
    fn test_scaling_vector() {
        let m = Matrix::<4, 4, f32>::scaling(-2.0, 3.0, -4.0);
        let p = Vector::new(1.0, 2.0, 3.0);
        let expected = Vector::new(-2.0, 6.0, -12.0);

        assert_eq!(m.transform(p), expected);
    }

    #[test]
    fn test_scaling_identity() {
        let m = Matrix::<4, 4, f32>::scaling(1.0, 1.0, 1.0);
        let p = Point::new(1.0, 2.0, 3.0);

        assert_eq!(m.transform(p), p);
    }

    #[test]
    fn test_scaling_zero() {
        let m = Matrix::<4, 4, f32>::scaling(0.0, 3.0, 4.0);
        let p = Point::new(1.0, 2.0, 3.0);
        let expected = Point::new(0.0, 6.0, 12.0);

        assert_eq!(m.transform(p), expected);
    }

    #[test]
    fn test_scaling_nonuniform() {
        let m = Matrix::<4, 4, f32>::scaling(2.0, 3.0, 4.0);
        let p = Point::new(1.0, 1.0, 1.0);
        let expected = Point::new(2.0, 3.0, 4.0);

        assert_eq!(m.transform(p), expected);
    }
}
