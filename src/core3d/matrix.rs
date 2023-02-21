use std::ops::Mul;

use itertools::{iproduct, Itertools};

use crate::core3d::{dot_product::DotProduct, tuple::Tuple};

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Matrix44f32 {
    pub matrix: [[f32; 4]; 4],
}

impl Matrix44f32 {
    /// Creates a new Matrix44f32 from 4x4 scaler values
    ///
    /// # Examples
    ///
    /// ```
    /// # use rusty_ray_tracer::core3d::matrix::Matrix44f32;
    ///
    /// let matrix = Matrix44f32::new(
    ///     1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0
    /// );
    /// assert_eq!([1.0, 2.0, 3.0, 4.0], matrix.matrix[0]);
    /// assert_eq!([5.0, 6.0, 7.0, 8.0], matrix.matrix[1]);
    /// assert_eq!([8.0, 7.0, 6.0, 5.0], matrix.matrix[2]);
    /// assert_eq!([4.0, 3.0, 2.0, 1.0], matrix.matrix[3]);
    /// ```
    #[must_use]
    pub const fn new(
        rc00: f32,
        rc01: f32,
        rc02: f32,
        rc03: f32,
        rc10: f32,
        rc11: f32,
        rc12: f32,
        rc13: f32,
        rc20: f32,
        rc21: f32,
        rc22: f32,
        rc23: f32,
        rc30: f32,
        rc31: f32,
        rc32: f32,
        rc33: f32,
    ) -> Self {
        Self {
            matrix: [
                [rc00, rc01, rc02, rc03],
                [rc10, rc11, rc12, rc13],
                [rc20, rc21, rc22, rc23],
                [rc30, rc31, rc32, rc33],
            ],
        }
    }
}

#[cfg(test)]
mod tests_matrix {
    use super::*;

    #[test]
    fn new() {
        let matrix = Matrix44f32::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        );
        assert_eq!([1.0, 2.0, 3.0, 4.0], matrix.matrix[0]);
        assert_eq!([5.0, 6.0, 7.0, 8.0], matrix.matrix[1]);
        assert_eq!([8.0, 7.0, 6.0, 5.0], matrix.matrix[2]);
        assert_eq!([4.0, 3.0, 2.0, 1.0], matrix.matrix[3]);
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn clone() {
        let matrix = Matrix44f32::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        );
        let matrix_copy = matrix;
        let matrix_clone = matrix_copy.clone();
        assert_eq!([1.0, 2.0, 3.0, 4.0], matrix_copy.matrix[0]);
        assert_eq!([5.0, 6.0, 7.0, 8.0], matrix_copy.matrix[1]);
        assert_eq!([8.0, 7.0, 6.0, 5.0], matrix_copy.matrix[2]);
        assert_eq!([4.0, 3.0, 2.0, 1.0], matrix_copy.matrix[3]);
        assert_eq!([1.0, 2.0, 3.0, 4.0], matrix_clone.matrix[0]);
        assert_eq!([5.0, 6.0, 7.0, 8.0], matrix_clone.matrix[1]);
        assert_eq!([8.0, 7.0, 6.0, 5.0], matrix_clone.matrix[2]);
        assert_eq!([4.0, 3.0, 2.0, 1.0], matrix_clone.matrix[3]);
    }

    #[test]
    fn debug_fmt() {
        let matrix = Matrix44f32::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        );
        assert_eq!(
            "Matrix44f32 { matrix: [[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0], [8.0, 7.0, 6.0, 5.0], [4.0, 3.0, 2.0, 1.0]] }",
            format!("{matrix:?}")
        );
    }
}

impl From<[[f32; 4]; 4]> for Matrix44f32 {
    /// Creates a new matrix from an array of scaler values
    ///
    /// # Examples
    ///
    /// ```
    /// # use rusty_ray_tracer::core3d::matrix::Matrix44f32;
    ///
    /// let matrix = Matrix44f32::from([
    ///     [1.0, 2.0, 3.0, 4.0],
    ///     [5.0, 6.0, 7.0, 8.0],
    ///     [8.0, 7.0, 6.0, 5.0],
    ///     [4.0, 3.0, 2.0, 1.0],
    /// ]);
    /// assert_eq!([1.0, 2.0, 3.0, 4.0], matrix.matrix[0]);
    /// assert_eq!([5.0, 6.0, 7.0, 8.0], matrix.matrix[1]);
    /// assert_eq!([8.0, 7.0, 6.0, 5.0], matrix.matrix[2]);
    /// assert_eq!([4.0, 3.0, 2.0, 1.0], matrix.matrix[3]);
    /// ```
    fn from(arr: [[f32; 4]; 4]) -> Self {
        Self { matrix: arr }
    }
}

impl From<Vec<Vec<f32>>> for Matrix44f32 {
    /// Creates a new matrix from an array of scaler values
    ///
    /// # Examples
    ///
    /// ```
    /// # use rusty_ray_tracer::core3d::matrix::Matrix44f32;
    ///
    /// let matrix = Matrix44f32::from([
    ///     [1.0, 2.0, 3.0, 4.0],
    ///     [5.0, 6.0, 7.0, 8.0],
    ///     [8.0, 7.0, 6.0, 5.0],
    ///     [4.0, 3.0, 2.0, 1.0],
    /// ]);
    /// assert_eq!([1.0, 2.0, 3.0, 4.0], matrix.matrix[0]);
    /// assert_eq!([5.0, 6.0, 7.0, 8.0], matrix.matrix[1]);
    /// assert_eq!([8.0, 7.0, 6.0, 5.0], matrix.matrix[2]);
    /// assert_eq!([4.0, 3.0, 2.0, 1.0], matrix.matrix[3]);
    /// ```
    fn from(vec: Vec<Vec<f32>>) -> Self {
        let mut matrix = Self::default();

        vec.iter().enumerate().for_each(|(i, v)| {
            matrix.matrix[i].iter_mut().set_from(v.iter().cloned());
        });

        matrix
    }
}

#[cfg(test)]
mod tests_from {
    use super::*;

    #[test]
    fn from_array() {
        let matrix = Matrix44f32::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);
        assert_eq!([1.0, 2.0, 3.0, 4.0], matrix.matrix[0]);
        assert_eq!([5.0, 6.0, 7.0, 8.0], matrix.matrix[1]);
        assert_eq!([8.0, 7.0, 6.0, 5.0], matrix.matrix[2]);
        assert_eq!([4.0, 3.0, 2.0, 1.0], matrix.matrix[3]);
    }

    #[test]
    fn from_vec() {
        let matrix = Matrix44f32::from(vec![
            vec![1.0, 2.0, 3.0],
            vec![5.0, 6.0, 7.0],
            vec![8.0, 7.0, 6.0],
        ]);
        assert_eq!([1.0, 2.0, 3.0, 0.0], matrix.matrix[0]);
        assert_eq!([5.0, 6.0, 7.0, 0.0], matrix.matrix[1]);
        assert_eq!([8.0, 7.0, 6.0, 0.0], matrix.matrix[2]);
        assert_eq!([0.0, 0.0, 0.0, 0.0], matrix.matrix[3]);
    }
}

impl Mul for Matrix44f32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.matrix[0].len(), rhs.matrix.len());

        let binding = iproduct!(0..4, 0..4)
            .map(|(r, c)| {
                let a = Tuple::new(
                    self.matrix[r][0],
                    self.matrix[r][1],
                    self.matrix[r][2],
                    self.matrix[r][3],
                );
                let b = Tuple::new(
                    rhs.matrix[0][c],
                    rhs.matrix[1][c],
                    rhs.matrix[2][c],
                    rhs.matrix[3][c],
                );

                a.dot(b)
            })
            .collect_vec();
        let (arr, []) = binding
            .as_chunks::<4>() else {
                panic!("matrix multiplication resulted in misformed matrix!")
            };

        Self {
            matrix: [arr[0], arr[1], arr[2], arr[3]],
        }
    }
}
