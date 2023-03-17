use std::ops::Mul;

use float_cmp::ApproxEq;
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
    #[allow(clippy::too_many_arguments)]
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

    /// Creates a new identity Matrix44f32
    ///
    /// # Examples
    ///
    /// ```
    /// # use rusty_ray_tracer::core3d::matrix::Matrix44f32;
    ///
    /// let matrix = Matrix44f32::identity();
    /// assert_eq!([1.0, 0.0, 0.0, 0.0], matrix.matrix[0]);
    /// assert_eq!([0.0, 1.0, 0.0, 0.0], matrix.matrix[1]);
    /// assert_eq!([0.0, 0.0, 1.0, 0.0], matrix.matrix[2]);
    /// assert_eq!([0.0, 0.0, 0.0, 1.0], matrix.matrix[3]);
    /// ```
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
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
    #[must_use]
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
    #[must_use]
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

impl ApproxEq for Matrix44f32 {
    type Margin = <f32 as ApproxEq>::Margin;

    /// Performs the `~=` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::matrix::Matrix44f32;
    /// # use float_cmp::ApproxEq;
    /// let a = Matrix44f32::from(vec!(vec!(1.23, 4.56, 7.89, 0.000000000000)));
    /// let b = Matrix44f32::from(vec!(vec!(1.23, 4.56, 7.89, 0.000000000001)));
    /// assert!(a.approx_eq(b, <Matrix44f32 as ApproxEq>::Margin::default()));
    /// ```
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::matrix::Matrix44f32;
    /// # use float_cmp::ApproxEq;
    /// let a = Matrix44f32::from(vec!(vec!(1.23, 4.56, 7.89, 1.0000000)));
    /// let b = Matrix44f32::from(vec!(vec!(1.23, 4.56, 7.89, 1.0000001)));
    /// assert!(a.approx_eq(b, <Matrix44f32 as ApproxEq>::Margin::default().ulps(2)));
    /// ```
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::matrix::Matrix44f32;
    /// # use float_cmp::ApproxEq;
    /// let a = Matrix44f32::from(vec!(vec!(1.23, 4.56, 7.89, 0.0)));
    /// let b = Matrix44f32::from(vec!(vec!(1.23, 4.56, 7.89, 1.0)));
    /// assert!(a.approx_eq(b, <Matrix44f32 as ApproxEq>::Margin::default().epsilon(1.0)));
    /// ```
    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        self.matrix
            .flatten()
            .iter()
            .zip_eq(other.matrix.flatten().iter())
            .all(|(a, b)| (*a).approx_eq(*b, margin))
    }
}

#[cfg(test)]
mod tests_approx_eq {
    use super::*;
    use float_cmp::assert_approx_eq;
    use std::panic;

    #[test]
    fn eq() {
        assert_approx_eq!(
            Matrix44f32,
            Matrix44f32::from(vec![vec![1.23, 4.56, 7.89, 0.000_000_000_000]]),
            Matrix44f32::from(vec![vec![1.23, 4.56, 7.89, 0.000_000_000_001]])
        );
        assert_approx_eq!(
            Matrix44f32,
            Matrix44f32::from(vec![vec![1.23, 4.56, 7.89, 1.000_000_0]]),
            Matrix44f32::from(vec![vec![1.23, 4.56, 7.89, 1.000_000_1]]),
            ulps = 2
        );
        assert_approx_eq!(
            Matrix44f32,
            Matrix44f32::from(vec![vec![1.23, 4.56, 7.89, 0.0]]),
            Matrix44f32::from(vec![vec![1.23, 4.56, 7.89, 1.0]]),
            epsilon = 1.0
        );
    }

    #[test]
    fn ne() {
        {
            let a = Matrix44f32::from(vec![vec![1.23, 4.56, 7.89, 1.000_000]]);
            let b = Matrix44f32::from(vec![vec![1.23, 4.56, 7.89, 1.000_001]]);
            assert!(a.approx_ne(b, <Matrix44f32 as ApproxEq>::Margin::default()));
        }
        {
            let a = Matrix44f32::from(vec![vec![1.23, 4.56, 7.89, 1.000_000]]);
            let b = Matrix44f32::from(vec![vec![1.23, 4.56, 7.89, 1.000_001]]);
            assert!(a.approx_ne(b, <Matrix44f32 as ApproxEq>::Margin::default().ulps(2)));
        }
        {
            let a = Matrix44f32::from(vec![vec![1.23, 4.56, 7.89, 0.000_000_0]]);
            let b = Matrix44f32::from(vec![vec![1.23, 4.56, 7.89, 1.000_000_1]]);
            assert!(a.approx_ne(b, <Matrix44f32 as ApproxEq>::Margin::default().epsilon(1.0)));
        }
    }
}

impl Mul for Matrix44f32 {
    type Output = Self;

    #[must_use]
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

#[cfg(test)]
mod tests_mul {
    use float_cmp::assert_approx_eq;

    use super::*;

    #[test]
    fn closure() {
        let a = Matrix44f32::from([
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
        ]);
        let b = Matrix44f32::from([
            [1.11, 2.22, 3.33, 1.0],
            [1.11, 2.22, 3.33, 1.0],
            [1.11, 2.22, 3.33, 1.0],
            [1.11, 2.22, 3.33, 1.0],
        ]);
        let expected = Matrix44f32::from([
            [15.184_801, 30.369_602, 45.554_398, 13.68],
            [15.184_801, 30.369_602, 45.554_398, 13.68],
            [15.184_801, 30.369_602, 45.554_398, 13.68],
            [15.184_801, 30.369_602, 45.554_398, 13.68],
        ]);
        assert_eq!(expected, a * b);
    }

    #[test]
    fn identity() {
        let a = Matrix44f32::from([
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
        ]);
        let b = Matrix44f32::identity();
        assert_eq!(a * b, a);
        assert_eq!(b * a, a);
    }

    #[test]
    fn associative() {
        let a = Matrix44f32::from([
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
        ]);
        let b = Matrix44f32::from([
            [1.11, 2.22, 3.33, 1.0],
            [1.11, 2.22, 3.33, 1.0],
            [1.11, 2.22, 3.33, 1.0],
            [1.11, 2.22, 3.33, 1.0],
        ]);
        let c = Matrix44f32::from([
            [2.34, 6.78, 11.22, 1.0],
            [1.23, 4.56, 7.89, 0.0],
            [1.11, 2.22, 3.33, 1.0],
            [1.11, 2.22, 3.33, 1.0],
        ]);
        assert_approx_eq!(Matrix44f32, a * (b * c), (a * b) * c);
        assert_approx_eq!(Matrix44f32, c * (a * b), (c * a) * b);
    }
}

impl Mul<Tuple> for Matrix44f32 {
    type Output = Tuple;

    #[must_use]
    fn mul(self, rhs: Tuple) -> Self::Output {
        assert_eq!(self.matrix[0].len(), rhs.tuple.len());

        let binding = (0..4)
            .map(|r| {
                let a = Tuple::new(
                    self.matrix[r][0],
                    self.matrix[r][1],
                    self.matrix[r][2],
                    self.matrix[r][3],
                );
                let b = Tuple::new(rhs.tuple[0], rhs.tuple[1], rhs.tuple[2], rhs.tuple[3]);

                a.dot(b)
            })
            .collect_vec();
        let (arr, []) = binding
            .as_chunks::<4>() else {
                panic!("matrix tuple multiplication resulted in misformed matrix!")
            };

        Tuple::from(arr[0])
    }
}

#[cfg(test)]
mod tests_mul_tuple {
    use float_cmp::assert_approx_eq;

    use super::*;

    #[test]
    fn closure() {
        let a = Matrix44f32::from([
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
        ]);
        let b = Tuple::from([1.11, 2.22, 3.33, 1.0]);
        let expected = Tuple::from([37.7622, 37.7622, 37.7622, 37.7622]);
        assert_eq!(expected, a * b);
    }

    #[test]
    fn identity() {
        let a = Matrix44f32::identity();
        let b = Tuple::from([1.23, 4.56, 7.89, 0.0]);
        assert_eq!(a * b, b);
        // assert_eq!(b * a, a);
    }

    #[test]
    fn associative() {
        let a = Matrix44f32::from([
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
        ]);
        let b = Matrix44f32::from([
            [1.11, 2.22, 3.33, 1.0],
            [1.11, 2.22, 3.33, 1.0],
            [1.11, 2.22, 3.33, 1.0],
            [1.11, 2.22, 3.33, 1.0],
        ]);
        let c = Tuple::from([2.34, 6.78, 11.22, 1.0]);
        assert_approx_eq!(Tuple, a * (b * c), (a * b) * c);
        // assert_approx_eq!(Tuple, c * (a * b), (c * a) * b);
    }
}

pub trait Transpose {
    fn transpose(self) -> Self;
}

impl Transpose for Matrix44f32 {
    fn transpose(self) -> Self {
        iproduct!(0..4, 0..4).fold(Self::default(), |mut acc, (r, c)| {
            acc.matrix[c][r] = self.matrix[r][c];
            acc
        })
    }
}

#[cfg(test)]
mod tests_transpose {
    use super::*;

    #[test]
    fn test_transpose() {
        let a = Matrix44f32::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let b = Matrix44f32::from([
            [1.0, 5.0, 9.0, 13.0],
            [2.0, 6.0, 10.0, 14.0],
            [3.0, 7.0, 11.0, 15.0],
            [4.0, 8.0, 12.0, 16.0],
        ]);
        assert_eq!(a.transpose(), b);
    }

    #[test]
    fn test_transpose_identity() {
        let a = Matrix44f32::identity();
        let b = Matrix44f32::identity().transpose();
        assert_eq!(a, b);
    }
}

pub trait Determinant {
    fn determinant(self) -> f32;
}

impl Determinant for Matrix44f32 {
    fn determinant(self) -> f32 {
        (self.matrix[0][0] * self.matrix[1][1]) - (self.matrix[0][1] * self.matrix[1][0])
    }
}

#[cfg(test)]
mod tests_determinant {
    use super::*;

    #[test]
    fn test_2x2_determinant() {
        let a = Matrix44f32::from([
            [1.0, 2.0, 0.0, 0.0],
            [5.0, 6.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);
        assert_eq!(a.determinant(), -4.0);
    }
}

pub trait Submatrix {
    fn submatrix(self, r: usize, c: usize) -> Self;
}

impl Submatrix for Matrix44f32 {
    fn submatrix(self, skiprow: usize, skipcol: usize) -> Self {
        iproduct!(0..4, 0..4)
            .filter(|(r, c)| *r != skiprow && *c != skipcol)
            .enumerate()
            .fold(Self::default(), |mut acc, (i, (skipr, skipc))| {
                let ir = i / 3;
                let ic = i % 3;
                acc.matrix[ir][ic] = self.matrix[skipr][skipc];
                acc
            })
    }
}

#[cfg(test)]
mod tests_submatrix {
    use super::*;

    #[test]
    fn test_submatrix() {
        let a = Matrix44f32::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let expected = Matrix44f32::from([
            [5.0, 6.0, 8.0, 0.0],
            [9.0, 10.0, 12.0, 0.0],
            [13.0, 14.0, 16.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);
        let result = a.submatrix(0, 2);
        assert_eq!(result, expected);
    }
}

pub trait Minor {
    fn minor(self, r: usize, c: usize) -> f32;
}

impl Minor for Matrix44f32 {
    fn minor(self, r: usize, c: usize) -> f32 {
        self.submatrix(r, c).determinant()
    }
}

#[cfg(test)]
mod tests_minor {
    use super::*;

    #[test]
    fn test_minor() {
        let a = Matrix44f32::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        assert_eq!(a.minor(0, 0), -4.0);
        assert_eq!(a.minor(0, 1), -8.0);
        assert_eq!(a.minor(0, 2), -4.0);

        assert_eq!(a.minor(1, 0), -8.0);
        assert_eq!(a.minor(1, 1), -16.0);
        assert_eq!(a.minor(1, 2), -8.0);

        assert_eq!(a.minor(2, 2), -4.0);
        assert_eq!(a.minor(3, 3), -4.0);
    }
}

pub trait Cofactor {
    fn cofactor(self, r: usize, c: usize) -> f32;
}

impl Cofactor for Matrix44f32 {
    fn cofactor(self, r: usize, c: usize) -> f32 {
        let sign = if (r + c) % 2 == 0 { 1.0 } else { -1.0 };
        self.minor(r, c) * sign
    }
}

#[cfg(test)]
mod tests_cofactor {
    use super::*;

    #[test]
    fn test_cofactor() {
        let a = Matrix44f32::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        assert_eq!(a.cofactor(0, 0), -4.0);
        assert_eq!(a.cofactor(0, 1), 8.0);
        assert_eq!(a.cofactor(0, 2), -4.0);

        assert_eq!(a.cofactor(1, 0), 8.0);
        assert_eq!(a.cofactor(1, 1), -16.0);
        assert_eq!(a.cofactor(1, 2), 8.0);

        assert_eq!(a.cofactor(2, 2), -4.0);
        assert_eq!(a.cofactor(3, 3), -4.0);
    }
}
