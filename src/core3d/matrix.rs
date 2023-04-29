use std::ops::{Add, Div, Index, IndexMut, Mul, Neg};

use float_cmp::ApproxEq;
use itertools::{iproduct, Itertools};

use crate::core3d::{dot_product::DotProduct, tuple::Tuple};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Matrix<const ROW: usize, const COL: usize, T> {
    pub matrix: [[T; COL]; ROW],
}

impl<const ROW: usize, const COL: usize, T> Matrix<ROW, COL, T> {
    #[must_use]
    pub fn new(matrix: [[T; COL]; ROW]) -> Self {
        Self { matrix }
    }
}

#[cfg(test)]
mod tests_matrix {
    use super::*;

    #[test]
    fn new() {
        let matrix = Matrix::<2, 2, f32>::new([[1.0, 2.0], [3.0, 4.0]]);
        assert_eq!([1.0, 2.0], matrix.matrix[0]);
        assert_eq!([3.0, 4.0], matrix.matrix[1]);
        let matrix = Matrix::<3, 3, f32>::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        assert_eq!([1.0, 2.0, 3.0], matrix.matrix[0]);
        assert_eq!([4.0, 5.0, 6.0], matrix.matrix[1]);
        assert_eq!([7.0, 8.0, 9.0], matrix.matrix[2]);
        let matrix = Matrix::<4, 4, f32>::new([
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
    #[allow(clippy::clone_on_copy)]
    fn clone() {
        let matrix = Matrix::<4, 4, f32>::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);
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
        let matrix = Matrix::<4, 4, f32>::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);
        assert_eq!(
            "Matrix { matrix: [[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0], [8.0, 7.0, 6.0, 5.0], [4.0, 3.0, 2.0, 1.0]] }",
            format!("{matrix:?}")
        );
    }
}

impl<const ROW: usize, const COL: usize, T> Default for Matrix<ROW, COL, T>
where
    T: Default + Copy,
{
    #[must_use]
    fn default() -> Self {
        Self {
            matrix: [[T::default(); COL]; ROW],
        }
    }
}

#[cfg(test)]
mod tests_default {
    use super::*;

    #[test]
    fn test() {
        let matrix = Matrix::<2, 2, f32>::default();
        assert_eq!(matrix, Matrix::new([[0.0, 0.0], [0.0, 0.0]]));

        let matrix = Matrix::<3, 3, f32>::default();
        assert_eq!(
            matrix,
            Matrix::new([[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]])
        );

        let matrix = Matrix::<4, 4, f32>::default();
        assert_eq!(
            matrix,
            Matrix::new([
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0]
            ])
        );
    }
}

impl<const ROW: usize, const COL: usize, T> Index<(usize, usize)> for Matrix<ROW, COL, T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.matrix[index.0][index.1]
    }
}

impl<const ROW: usize, const COL: usize, T> IndexMut<(usize, usize)> for Matrix<ROW, COL, T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.matrix[index.0][index.1]
    }
}

#[cfg(test)]
mod tests_index {
    use super::*;

    #[test]
    fn test() {
        let matrix = Matrix::<2, 2, f32>::new([[1.0, 2.0], [3.0, 4.0]]);
        assert_eq!(1.0, matrix[(0, 0)]);
        assert_eq!(2.0, matrix[(0, 1)]);
        assert_eq!(3.0, matrix[(1, 0)]);
        assert_eq!(4.0, matrix[(1, 1)]);
        let matrix = Matrix::<3, 3, f32>::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        assert_eq!(1.0, matrix[(0, 0)]);
        assert_eq!(2.0, matrix[(0, 1)]);
        assert_eq!(3.0, matrix[(0, 2)]);
        assert_eq!(4.0, matrix[(1, 0)]);
        assert_eq!(5.0, matrix[(1, 1)]);
        assert_eq!(6.0, matrix[(1, 2)]);
        assert_eq!(7.0, matrix[(2, 0)]);
        assert_eq!(8.0, matrix[(2, 1)]);
        assert_eq!(9.0, matrix[(2, 2)]);
        let matrix = Matrix::<4, 4, f32>::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);
        assert_eq!(1.0, matrix[(0, 0)]);
        assert_eq!(2.0, matrix[(0, 1)]);
        assert_eq!(3.0, matrix[(0, 2)]);
        assert_eq!(4.0, matrix[(0, 3)]);
        assert_eq!(5.0, matrix[(1, 0)]);
        assert_eq!(6.0, matrix[(1, 1)]);
        assert_eq!(7.0, matrix[(1, 2)]);
        assert_eq!(8.0, matrix[(1, 3)]);
        assert_eq!(8.0, matrix[(2, 0)]);
        assert_eq!(7.0, matrix[(2, 1)]);
        assert_eq!(6.0, matrix[(2, 2)]);
        assert_eq!(5.0, matrix[(2, 3)]);
        assert_eq!(4.0, matrix[(3, 0)]);
        assert_eq!(3.0, matrix[(3, 1)]);
        assert_eq!(2.0, matrix[(3, 2)]);
        assert_eq!(1.0, matrix[(3, 3)]);
    }
}

/// Trait that defines the identity function for a matrix.
///
/// This trait defines the identity function for a matrix.
/// The identity function is a matrix that has 1s along the main diagonal
/// and 0s everywhere else.
///
/// Example:
/// ```
/// # use rusty_ray_tracer::core3d::matrix::Matrix;
/// # use rusty_ray_tracer::core3d::matrix::Identity;
///
/// let a = Matrix::<4, 4, f32>::identity();
/// assert_eq!(
///     a,
///     Matrix::new([
///         [1.0, 0.0, 0.0, 0.0],
///         [0.0, 1.0, 0.0, 0.0],
///         [0.0, 0.0, 1.0, 0.0],
///         [0.0, 0.0, 0.0, 1.0]
///     ])
/// );
/// ```
pub trait Identity<const N: usize, T>
where
    Self: Sized,
    T: Default + Copy + From<i8>,
{
    #[must_use]
    fn identity() -> Matrix<N, N, T> {
        let mut matrix = Matrix::default();
        for i in 0..N {
            matrix.matrix[i][i] = T::from(1);
        }
        matrix
    }
}

#[cfg(test)]
mod tests_identity {
    use super::*;

    #[test]
    fn test_identity() {
        let a = Matrix::<2, 2, f32>::identity();
        assert_eq!(a, Matrix::new([[1.0, 0.0], [0.0, 1.0]]));

        let a = Matrix::<3, 3, f32>::identity();
        assert_eq!(
            a,
            Matrix::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
        );

        let a = Matrix::<4, 4, f32>::identity();
        assert_eq!(
            a,
            Matrix::new([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ])
        );
    }
}

impl<const ROW: usize, const COL: usize, T> From<[[T; COL]; ROW]> for Matrix<ROW, COL, T> {
    /// Creates a new matrix from an array of scaler values
    ///
    /// # Examples
    ///
    /// ```
    /// # use rusty_ray_tracer::core3d::matrix::Matrix;
    ///
    /// let matrix = Matrix::<4, 4, f32>::new([
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
    fn from(arr: [[T; COL]; ROW]) -> Self {
        Self { matrix: arr }
    }
}

impl<const ROW: usize, const COL: usize, T> From<Vec<Vec<T>>> for Matrix<ROW, COL, T>
where
    Self: Default,
    T: Copy,
{
    /// Creates a new matrix from an array of scaler values
    ///
    /// # Examples
    ///
    /// ```
    /// # use rusty_ray_tracer::core3d::matrix::Matrix;
    ///
    /// let matrix = Matrix::<4, 4, f32>::new([
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
    fn from(vec: Vec<Vec<T>>) -> Self {
        let mut matrix = Self::default();

        vec.iter().enumerate().for_each(|(i, v)| {
            matrix.matrix[i].iter_mut().set_from(v.iter().copied());
        });

        matrix
    }
}

#[cfg(test)]
mod tests_from {
    use super::*;

    #[test]
    fn from_array() {
        let matrix = Matrix::<4, 4, f32>::from([
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
        let matrix = Matrix::<4, 4, f32>::from(vec![
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

impl<const ROW: usize, const COL: usize, T> ApproxEq for Matrix<ROW, COL, T>
where
    T: ApproxEq + Copy,
{
    type Margin = <T as ApproxEq>::Margin;

    /// Performs the `~=` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use rusty_ray_tracer::core3d::matrix::Matrix;
    /// # use float_cmp::ApproxEq;
    /// let a = Matrix::<4, 4, f32>::from(vec!(vec!(1.23, 4.56, 7.89, 0.000000000000)));
    /// let b = Matrix::<4, 4, f32>::from(vec!(vec!(1.23, 4.56, 7.89, 0.000000000001)));
    /// assert!(a.approx_eq(b, <Matrix::<4, 4, f32> as ApproxEq>::Margin::default()));
    /// ```
    ///
    /// ```
    /// # use rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use rusty_ray_tracer::core3d::matrix::Matrix;
    /// # use float_cmp::ApproxEq;
    /// let a = Matrix::<4, 4, f32>::from(vec!(vec!(1.23, 4.56, 7.89, 1.0000000)));
    /// let b = Matrix::<4, 4, f32>::from(vec!(vec!(1.23, 4.56, 7.89, 1.0000001)));
    /// assert!(a.approx_eq(b, <Matrix::<4, 4, f32> as ApproxEq>::Margin::default().ulps(2)));
    /// ```
    ///
    /// ```
    /// # use rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use rusty_ray_tracer::core3d::matrix::Matrix;
    /// # use float_cmp::ApproxEq;
    /// let a = Matrix::<4, 4, f32>::from(vec!(vec!(1.23, 4.56, 7.89, 0.0)));
    /// let b = Matrix::<4, 4, f32>::from(vec!(vec!(1.23, 4.56, 7.89, 1.0)));
    /// assert!(a.approx_eq(b, <Matrix::<4, 4, f32> as ApproxEq>::Margin::default().epsilon(1.0)));
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
            Matrix::<4, 4, f32>,
            Matrix::<4, 4, f32>::from(vec![vec![1.23, 4.56, 7.89, 0.000_000_000_000]]),
            Matrix::<4, 4, f32>::from(vec![vec![1.23, 4.56, 7.89, 0.000_000_000_001]])
        );
        assert_approx_eq!(
            Matrix::<4, 4, f32>,
            Matrix::<4, 4, f32>::from(vec![vec![1.23, 4.56, 7.89, 1.000_000_0]]),
            Matrix::<4, 4, f32>::from(vec![vec![1.23, 4.56, 7.89, 1.000_000_1]]),
            ulps = 2
        );
        assert_approx_eq!(
            Matrix::<4, 4, f32>,
            Matrix::<4, 4, f32>::from(vec![vec![1.23, 4.56, 7.89, 0.0]]),
            Matrix::<4, 4, f32>::from(vec![vec![1.23, 4.56, 7.89, 1.0]]),
            epsilon = 1.0
        );
    }

    #[test]
    fn ne() {
        {
            let a = Matrix::<4, 4, f32>::from(vec![vec![1.23, 4.56, 7.89, 1.000_000]]);
            let b = Matrix::<4, 4, f32>::from(vec![vec![1.23, 4.56, 7.89, 1.000_001]]);
            assert!(a.approx_ne(b, <Matrix::<4, 4, f32> as ApproxEq>::Margin::default()));
        }
        {
            let a = Matrix::<4, 4, f32>::from(vec![vec![1.23, 4.56, 7.89, 1.000_000]]);
            let b = Matrix::<4, 4, f32>::from(vec![vec![1.23, 4.56, 7.89, 1.000_001]]);
            assert!(a.approx_ne(
                b,
                <Matrix::<4, 4, f32> as ApproxEq>::Margin::default().ulps(2)
            ));
        }
        {
            let a = Matrix::<4, 4, f32>::from(vec![vec![1.23, 4.56, 7.89, 0.000_000_0]]);
            let b = Matrix::<4, 4, f32>::from(vec![vec![1.23, 4.56, 7.89, 1.000_000_1]]);
            assert!(a.approx_ne(
                b,
                <Matrix::<4, 4, f32> as ApproxEq>::Margin::default().epsilon(1.0)
            ));
        }
    }
}

impl Mul for Matrix<4, 4, f32> {
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
        let a = Matrix::<4, 4, f32>::new([
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
        ]);
        let b = Matrix::<4, 4, f32>::new([
            [1.11, 2.22, 3.33, 1.0],
            [1.11, 2.22, 3.33, 1.0],
            [1.11, 2.22, 3.33, 1.0],
            [1.11, 2.22, 3.33, 1.0],
        ]);
        let expected = Matrix::<4, 4, f32>::new([
            [15.184_801, 30.369_602, 45.554_398, 13.68],
            [15.184_801, 30.369_602, 45.554_398, 13.68],
            [15.184_801, 30.369_602, 45.554_398, 13.68],
            [15.184_801, 30.369_602, 45.554_398, 13.68],
        ]);
        assert_eq!(expected, a * b);
    }

    #[test]
    fn identity() {
        let a = Matrix::<4, 4, f32>::new([
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
        ]);
        let b = Matrix::<4, 4, f32>::identity();
        assert_eq!(a * b, a);
        assert_eq!(b * a, a);
    }

    #[test]
    fn associative() {
        let a = Matrix::<4, 4, f32>::new([
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
        ]);
        let b = Matrix::<4, 4, f32>::new([
            [1.11, 2.22, 3.33, 1.0],
            [1.11, 2.22, 3.33, 1.0],
            [1.11, 2.22, 3.33, 1.0],
            [1.11, 2.22, 3.33, 1.0],
        ]);
        let c = Matrix::<4, 4, f32>::new([
            [2.34, 6.78, 11.22, 1.0],
            [1.23, 4.56, 7.89, 0.0],
            [1.11, 2.22, 3.33, 1.0],
            [1.11, 2.22, 3.33, 1.0],
        ]);
        assert_approx_eq!(Matrix::<4, 4, f32>, a * (b * c), (a * b) * c);
        assert_approx_eq!(Matrix::<4, 4, f32>, c * (a * b), (c * a) * b);
    }
}

impl Mul<Tuple> for Matrix<4, 4, f32> {
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
        let a = Matrix::<4, 4, f32>::new([
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
        let a = Matrix::<4, 4, f32>::identity();
        let b = Tuple::from([1.23, 4.56, 7.89, 0.0]);
        assert_eq!(a * b, b);
        // assert_eq!(b * a, a);
    }

    #[test]
    fn associative() {
        let a = Matrix::<4, 4, f32>::new([
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
            [1.23, 4.56, 7.89, 0.0],
        ]);
        let b = Matrix::<4, 4, f32>::new([
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

// This is a trait used to transpose a 4x4 matrix.
pub trait Transpose<const N: usize, T>: Sized
where
    Self: Default + Index<(usize, usize)> + IndexMut<(usize, usize)>,
    <Self as Index<(usize, usize)>>::Output: Copy,
{
    fn transpose(self) -> Self
    where
        <Self as Index<(usize, usize)>>::Output: Sized,
    {
        iproduct!(0..N, 0..N).fold(Self::default(), |mut acc, (r, c)| {
            acc[(c, r)] = self[(r, c)];
            acc
        })
    }
}

impl<const N: usize, T> Transpose<N, T> for Matrix<N, N, T>
where
    Self: Default + Index<(usize, usize)> + IndexMut<(usize, usize)>,
    <Self as Index<(usize, usize)>>::Output: Copy,
{
}

#[cfg(test)]
mod tests_transpose {
    use super::*;

    #[test]
    fn test_transpose() {
        let a = Matrix::<4, 4, f32>::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let b = Matrix::<4, 4, f32>::new([
            [1.0, 5.0, 9.0, 13.0],
            [2.0, 6.0, 10.0, 14.0],
            [3.0, 7.0, 11.0, 15.0],
            [4.0, 8.0, 12.0, 16.0],
        ]);
        assert_eq!(a.transpose(), b);
    }

    #[test]
    fn test_transpose_identity() {
        let a = Matrix::<4, 4, f32>::identity();
        let b = Matrix::<4, 4, f32>::identity().transpose();
        assert_eq!(a, b);
    }
}

// This code implements the Submatrix trait for the Matrix::<4, 4, f32> type.
// It takes a matrix and two indices for rows and columns and returns
// a submatrix of the original matrix without the row and column specified
// by the indices.
// The indices are zero-based.
// The function is used by the determinant function.
pub trait Submatrix<const ROW: usize, const COL: usize, T>
where
    Self: Index<(usize, usize), Output = T>,
    T: Copy,
{
    type Result: Default + IndexMut<(usize, usize), Output = T>;

    fn submatrix(&self, skiprow: usize, skipcol: usize) -> Self::Result {
        iproduct!(0..ROW, 0..COL)
            .filter(|(r, c)| *r != skiprow && *c != skipcol)
            .enumerate()
            .fold(Self::Result::default(), |mut acc, (i, (r, c))| {
                let ir = i / (COL - 1);
                let ic = i % (COL - 1);
                acc[(ir, ic)] = self[(r, c)];
                acc
            })
    }
}

// impl Submatrix<4, 4, f32> for Matrix<4, 4, f32> {
//     type Result = Matrix<3, 3, f32>;
// }

// impl Submatrix<3, 3, f32> for Matrix<3, 3, f32> {
//     type Result = Matrix<2, 2, f32>;
// }

#[cfg(test)]
mod tests_submatrix {
    use super::*;

    #[test]
    fn test_submatrix() {
        let a = Matrix::<4, 4, f32>::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let a33 =
            Matrix::<3, 3, f32>::new([[5.0, 6.0, 8.0], [9.0, 10.0, 12.0], [13.0, 14.0, 16.0]]);
        let a22 = Matrix::<2, 2, f32>::new([[5.0, 8.0], [13.0, 16.0]]);
        let result = a.submatrix(0, 2);
        assert_eq!(result, a33);
        let result = a33.submatrix(1, 1);
        assert_eq!(result, a22);
    }
}

/// Given a 2x2 matrix, return the determinant.
/// The determinant of a 2x2 matrix is:
///   | a  b |
///   | c  d |
///   a * d - b * c
///
///   | 1  2 |
///   | 3  4 |
///   1 * 4 - 2 * 3 = -2
/// # Examples
/// ```rust
/// # use rusty_ray_tracer::core3d::matrix::Matrix;
/// # use rusty_ray_tracer::core3d::matrix::Determinant;
///
/// let m = Matrix::<2, 2, f32>::new([[1.0, 2.0], [3.0, 4.0]]);
/// assert_eq!(m.determinant(), -2.0);
/// ```
// pub trait Determinant<const ROW: usize, const COL: usize, T>
// where
//     Self: Index<(usize, usize), Output = T> + Cofactor<ROW, COL, T>,
//     T: Copy + Default + Add<Output = T> + Neg<Output = T>,
// {
//     type Output = T;
//     fn determinant(&self) -> T {
//         (0..COL).fold(T::default(), |acc, c| {
//             acc + (self[(0, c)] * self.cofactor(0, c))
//         })
//     }
// }

pub trait Determinant<const ROW: usize, const COL: usize, T> {
    type Output;
    fn determinant(&self) -> Self::Output;
}

impl Determinant<4, 4, f32> for Matrix<4, 4, f32> {
    type Output = f32;
    fn determinant(&self) -> f32 {
        (0..4).fold(f32::default(), |acc, c| {
            acc + (self[(0, c)] * self.cofactor(0, c))
        })
    }
}

impl Determinant<3, 3, f32> for Matrix<3, 3, f32> {
    type Output = f32;
    fn determinant(&self) -> f32 {
        (0..3).fold(f32::default(), |acc, c| {
            acc + (self[(0, c)] * self.cofactor(0, c))
        })
    }
}

impl Determinant<2, 2, f32> for Matrix<2, 2, f32> {
    type Output = f32;
    fn determinant(&self) -> f32 {
        (self.matrix[0][0] * self.matrix[1][1]) - (self.matrix[0][1] * self.matrix[1][0])
    }
}

impl Determinant<1, 1, f32> for Matrix<1, 1, f32> {
    type Output = f32;
    fn determinant(&self) -> f32 {
        self.matrix[0][0]
    }
}

#[cfg(test)]
mod tests_determinant {
    use super::*;

    #[test]
    fn test_2x2_determinant() {
        let a = Matrix::<2, 2, f32>::new([[1.0, 2.0], [5.0, 6.0]]);
        assert_eq!(a.determinant(), -4.0);
    }

    #[test]
    fn test_3x3_determinant() {
        let a = Matrix::<3, 3, f32>::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        assert_eq!(a.determinant(), 0.0);
    }

    #[test]
    fn test_4x4_determinant() {
        let a = Matrix::<4, 4, f32>::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        assert_eq!(a.determinant(), 0.0);
    }
}

/// Determine the minor of a matrix at a given row and column.
///
/// The minor of a matrix is the determinant of the submatrix that is created
/// when the given row and column are omitted. For example, the minor of a 3x3
/// matrix at row 0 and column 0 is the determinant of the 2x2 matrix that is
/// formed by omitting the first row and first column.
///
/// # Arguments
///
/// * `row` - The row to omit when calculating the minor
/// * `col` - The column to omit when calculating the minor
///
/// # Examples
///
/// ```
/// # use rusty_ray_tracer::core3d::matrix::Matrix;
/// # use rusty_ray_tracer::core3d::matrix::Minor;
///
/// let a = Matrix::<3, 3, f32>::new([[1.0, 2.0, 3.0],
///                                   [4.0, 5.0, 6.0],
///                                   [7.0, 8.0, 9.0]]);
/// assert_eq!(a.minor(0, 0), -3.0);
/// assert_eq!(a.minor(0, 1), -6.0);
/// assert_eq!(a.minor(0, 2), -3.0);
/// assert_eq!(a.minor(1, 0), -6.0);
/// assert_eq!(a.minor(1, 1), -12.0);
/// assert_eq!(a.minor(1, 2), -6.0);
/// assert_eq!(a.minor(2, 0), -3.0);
/// assert_eq!(a.minor(2, 1), -6.0);
/// assert_eq!(a.minor(2, 2), -3.0);
///
/// let m = Matrix::<4, 4, f32>::new([
///     [1.0, 2.0, 3.0, 4.0],
///     [5.0, 6.0, 7.0, 8.0],
///     [9.0, 10.0, 11.0, 12.0],
///     [13.0, 14.0, 15.0, 16.0],
/// ]);
/// assert_eq!(m.minor(0, 0), 0.0);
/// assert_eq!(m.minor(0, 1), 0.0);
/// assert_eq!(m.minor(0, 2), 0.0);
/// assert_eq!(m.minor(0, 3), 0.0);
/// assert_eq!(m.minor(1, 0), 0.0);
/// assert_eq!(m.minor(1, 1), 0.0);
/// assert_eq!(m.minor(1, 2), 0.0);
/// assert_eq!(m.minor(1, 3), 0.0);
/// assert_eq!(m.minor(2, 0), 0.0);
/// assert_eq!(m.minor(2, 1), 0.0);
/// assert_eq!(m.minor(2, 2), 0.0);
/// assert_eq!(m.minor(2, 3), 0.0);
/// assert_eq!(m.minor(3, 0), 0.0);
/// assert_eq!(m.minor(3, 1), 0.0);
/// assert_eq!(m.minor(3, 2), 0.0);
/// assert_eq!(m.minor(3, 3), 0.0);
/// ```
pub trait Minor<const ROW: usize, const COL: usize, T>
where
    Self: Submatrix<ROW, COL, T> + Sized,
    T: Copy + Default + Add<Output = T>,
    <Self as Submatrix<ROW, COL, T>>::Result: Determinant<{ ROW - 1 }, { COL - 1 }, T, Output = T>,
{
    type Output = T;

    fn minor(&self, row: usize, col: usize) -> T {
        let submatrix = self.submatrix(row, col);
        submatrix.determinant()
    }
}

#[cfg(test)]
mod tests_minor {
    use super::*;

    #[test]
    fn test_minor3x3() {
        let a = Matrix::<3, 3, f32>::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]]);
        assert_eq!(a.minor(0, 0), -4.0);
        assert_eq!(a.minor(0, 1), -8.0);
        assert_eq!(a.minor(0, 2), -4.0);

        assert_eq!(a.minor(1, 0), -8.0);
        assert_eq!(a.minor(1, 1), -16.0);
        assert_eq!(a.minor(1, 2), -8.0);

        assert_eq!(a.minor(2, 2), -4.0);
    }

    #[test]
    fn test_minor4x4() {
        let a = Matrix::<4, 4, f32>::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        assert_eq!(a.minor(0, 0), 0.0);
        assert_eq!(a.minor(0, 1), -0.0);
        assert_eq!(a.minor(0, 2), 0.0);
        assert_eq!(a.minor(0, 3), -0.0);

        assert_eq!(a.minor(1, 0), 0.0);
        assert_eq!(a.minor(1, 1), -0.0);
        assert_eq!(a.minor(1, 2), 0.0);
        assert_eq!(a.minor(1, 3), -0.0);

        assert_eq!(a.minor(2, 0), 0.0);
        assert_eq!(a.minor(2, 1), -0.0);
        assert_eq!(a.minor(2, 2), 0.0);
        assert_eq!(a.minor(2, 3), -0.0);

        assert_eq!(a.minor(3, 0), 0.0);
        assert_eq!(a.minor(3, 1), -0.0);
        assert_eq!(a.minor(3, 2), 0.0);
        assert_eq!(a.minor(3, 3), -0.0);
    }
}

/// Cofactor trait for a matrix
/// This trait is implemented for any matrix that implements the Submatrix and Determinant traits
/// It has a function called cofactor that calculates the cofactor of a given element in the matrix
/// It takes in a row and column index
/// It returns the cofactor of the given element
/// The sign of the cofactor depends on whether the sum of the row and column indices is even or odd
/// Returns the minor of the given matrix at the given row and column.
///
/// # Example
/// ```
/// # use rusty_ray_tracer::core3d::matrix::Matrix;
/// # use rusty_ray_tracer::core3d::matrix::Cofactor;
///
/// let a = Matrix::<3, 3, f32>::new([[1.0, 2.0, 3.0],
///                                   [4.0, 5.0, 6.0],
///                                   [7.0, 8.0, 9.0]]);
/// assert_eq!(a.cofactor(0, 0), -3.0);
/// assert_eq!(a.cofactor(0, 1), 6.0);
/// assert_eq!(a.cofactor(0, 2), -3.0);
/// assert_eq!(a.cofactor(1, 0), 6.0);
/// assert_eq!(a.cofactor(1, 1), -12.0);
/// assert_eq!(a.cofactor(1, 2), 6.0);
/// assert_eq!(a.cofactor(2, 0), -3.0);
/// assert_eq!(a.cofactor(2, 1), 6.0);
/// assert_eq!(a.cofactor(2, 2), -3.0);
/// ```
pub trait Cofactor<const ROW: usize, const COL: usize, T>
where
    Self: Minor<ROW, COL, T, Output = T>,
    T: Copy + Default + Add<Output = T> + Neg<Output = T>,
    <Self as Submatrix<ROW, COL, T>>::Result: Determinant<{ ROW - 1 }, { COL - 1 }, T, Output = T>,
{
    type Output = T;

    fn cofactor(&self, row: usize, col: usize) -> T {
        let minor = self.minor(row, col);
        let is_positive_cell = (row + col) % 2 == 0;
        if is_positive_cell {
            minor
        } else {
            -minor
        }
    }
}

#[cfg(test)]
mod tests_cofactor {
    use super::*;

    #[test]
    fn test_cofactor3x3() {
        let a = Matrix::<3, 3, f32>::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]]);
        assert_eq!(a.cofactor(0, 0), -4.0);
        assert_eq!(a.cofactor(0, 1), 8.0);
        assert_eq!(a.cofactor(0, 2), -4.0);

        assert_eq!(a.cofactor(1, 0), 8.0);
        assert_eq!(a.cofactor(1, 1), -16.0);
        assert_eq!(a.cofactor(1, 2), 8.0);

        assert_eq!(a.cofactor(2, 2), -4.0);
    }

    #[test]
    fn test_cofactor4x4() {
        let a = Matrix::<4, 4, f32>::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        assert_eq!(a.cofactor(0, 0), 0.0);
        assert_eq!(a.cofactor(0, 1), -0.0);
        assert_eq!(a.cofactor(0, 2), 0.0);
        assert_eq!(a.cofactor(0, 3), -0.0);

        assert_eq!(a.cofactor(1, 0), 0.0);
        assert_eq!(a.cofactor(1, 1), -0.0);
        assert_eq!(a.cofactor(1, 2), 0.0);
        assert_eq!(a.cofactor(1, 3), -0.0);

        assert_eq!(a.cofactor(2, 0), 0.0);
        assert_eq!(a.cofactor(2, 1), -0.0);
        assert_eq!(a.cofactor(2, 2), 0.0);
        assert_eq!(a.cofactor(2, 3), -0.0);

        assert_eq!(a.cofactor(3, 0), 0.0);
        assert_eq!(a.cofactor(3, 1), -0.0);
        assert_eq!(a.cofactor(3, 2), 0.0);
        assert_eq!(a.cofactor(3, 3), -0.0);
    }
}

/// This function computes the inverse of a matrix, if possible.
/// It returns None if the matrix is not invertible.
/// The inverse of a matrix is the matrix that, when multiplied with the original matrix, results in the identity matrix.
/// The inverse of a matrix is denoted as A^-1.
/// The inverse of a matrix can be computed by dividing the adjugate of the matrix by its determinant.
/// The adjugate of a matrix is the transpose of the matrix of its cofactors.
/// The cofactor of a matrix element is the determinant of the minor of the matrix that results from removing the row and column containing that element.
/// The minor of a matrix element is the matrix obtained by removing the row and column containing that element.
/// The determinant of a matrix is the sum of the products of the elements on the leading diagonal, minus the sum of the products of the elements on the other diagonal.
/// The leading diagonal of a matrix is the diagonal from the upper left to the lower right.
/// The other diagonal of a matrix is the diagonal from the lower left to the upper right.
/// The determinant of a matrix can be computed by recursively computing the determinant of a smaller matrix until the matrix is 2x2.
/// The determinant of a 2x2 matrix is the product of the elements in the leading diagonal minus the product of the elements in the other diagonal.
/// To compute the determinant of a 2x2 matrix, the matrix is partitioned into four quadrants, as follows:
/// A B
/// C D
/// The determinant of the matrix is then computed as follows:
/// det(A) = (A)
/// det(B) = (B)
/// det(C) = (C)
/// det(D) = (D)
/// det(M) = (A*D - B*C)
pub trait Invert<const ROW: usize, const COL: usize, T>
where
    Self: Sized
        + Default
        + IndexMut<(usize, usize), Output = T>
        + Determinant<ROW, COL, T, Output = T>
        + Cofactor<ROW, COL, T, Output = T>,
    <Self as Submatrix<ROW, COL, T>>::Result: Determinant<{ ROW - 1 }, { COL - 1 }, T, Output = T>,
    T: Copy + Default + Add<Output = T> + Div<Output = T> + PartialEq<f32> + Neg<Output = T>,
{
    fn is_invertible(&self) -> bool {
        self.determinant() != 0.0_f32
    }

    fn inverse(&self) -> Option<Self> {
        if !self.is_invertible() {
            return None;
        }

        let det = self.determinant();
        let result = iproduct!(0..ROW, 0..COL).fold(Self::default(), |mut acc, (row, col)| {
            let c = self.cofactor(row, col);
            acc[(col, row)] = c / det;
            acc
        });
        Some(result)
    }
}

#[cfg(test)]
mod tests_invert {
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_is_invertible_2x2_true() {
        // Test a 2x2 matrix that is invertible
        let m = Matrix::<2, 2, f32>::new([[1.0, 2.0], [3.0, 4.0]]);
        assert!(m.is_invertible());
    }

    #[test]
    fn test_is_invertible_2x2_false() {
        // Test a 2x2 matrix that is not invertible
        let m = Matrix::<2, 2, f32>::new([[1.0, 2.0], [2.0, 4.0]]);
        assert!(!m.is_invertible());
    }

    #[test]
    fn test_is_invertible_3x3_true() {
        // Test a 3x3 matrix that is invertible
        let m = Matrix::<3, 3, f32>::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
        assert!(m.is_invertible());
    }

    #[test]
    fn test_is_invertible_3x3_false() {
        // Test a 3x3 matrix that is invertible
        let m = Matrix::<3, 3, f32>::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        assert!(!m.is_invertible());
    }

    #[test]
    fn test_is_invertible_4x4_true() {
        // Test a 4x4 matrix that is invertible
        let m = Matrix::<4, 4, f32>::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        assert!(m.is_invertible());
    }

    #[test]
    fn test_is_invertible_4x4_false() {
        // Test a 4x4 matrix that is invertible
        let m = Matrix::<4, 4, f32>::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        assert!(!m.is_invertible());
    }

    #[test]
    fn test_inverse_2x2() {
        // Test a 2x2 invertible matrix
        let m = Matrix::<2, 2, f32>::new([[4.0, 7.0], [2.0, 6.0]]);
        let m_inv = m.inverse().unwrap();
        assert_eq!(m_inv, Matrix::<2, 2, f32>::new([[0.6, -0.7], [-0.2, 0.4]]));
    }

    #[test]
    fn test_inverse_2x2_not_invertible() {
        // Test a 2x2 matrix that is not invertible
        let m = Matrix::<2, 2, f32>::new([[1.0, 2.0], [2.0, 4.0]]);
        assert!(m.inverse().is_none());
    }

    #[test]
    fn test_inverse_3x3() {
        // Test a 3x3 invertible matrix
        let m = Matrix::<3, 3, f32>::new([[3.0, 0.0, 2.0], [2.0, 0.0, -2.0], [0.0, 1.0, 1.0]]);
        let m_inv = m.inverse().unwrap();
        assert_eq!(
            m_inv,
            Matrix::<3, 3, f32>::new([[0.2, 0.2, 0.0], [-0.2, 0.3, 1.0], [0.2, -0.3, 0.0],])
        );
    }

    #[test]
    fn test_inverse_3x3_not_invertible() {
        // Test a 3x3 matrix that is not invertible
        let m = Matrix::<3, 3, f32>::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        assert!(m.inverse().is_none());
    }

    #[test]
    fn test_inverse_4x4() {
        // Test a 4x4 invertible matrix
        let m = Matrix::<4, 4, f32>::new([
            [4.0, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
            [0.0, 0.0, 0.5, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let m_inv = m.inverse().unwrap();
        assert_eq!(
            m_inv,
            Matrix::<4, 4, f32>::new([
                [0.25, 0.0, 0.0, 0.0],
                [0.0, 0.5, 0.0, 0.0],
                [0.0, 0.0, 2.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ])
        );
    }

    #[test]
    fn test_inverse_identity() {
        let m = Matrix::<4, 4, f32>::new([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);
        let m_inv = m.inverse().unwrap();
        let ident = Matrix::<4, 4, f32>::identity();
        let prod = m * m_inv;

        assert_approx_eq!(Matrix::<4, 4, f32>, prod, ident);
    }

    #[test]
    fn test_identity_inverse() {
        let m = Matrix::<4, 4, f32>::identity();
        let m_inv = m.inverse().unwrap();

        assert_eq!(m, m_inv);
    }

    #[test]
    fn test_inverse_transpose() {
        let m = Matrix::<4, 4, f32>::new([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);
        let m_inv_trans = m.inverse().unwrap().transpose();
        let m_trans_inv = m.transpose().inverse().unwrap();

        assert_approx_eq!(Matrix::<4, 4, f32>, m_inv_trans, m_trans_inv);
    }
}

pub type Matrix44f32 = Matrix<4, 4, f32>;
pub type Matrix33f32 = Matrix<3, 3, f32>;
pub type Matrix22f32 = Matrix<2, 2, f32>;
// pub type Matrix11f32 = Matrix<1, 1, f32>;

impl Identity<4, f32> for Matrix44f32 {}
impl Identity<3, f32> for Matrix33f32 {}
impl Identity<2, f32> for Matrix22f32 {}

// impl Determinant<4, 4, f32> for Matrix44f32 {}
// impl Determinant<3, 3, f32> for Matrix33f32 {}

impl Submatrix<4, 4, f32> for Matrix44f32 {
    type Result = Matrix<3, 3, f32>;
}
impl Submatrix<3, 3, f32> for Matrix33f32 {
    type Result = Matrix<2, 2, f32>;
}
impl Submatrix<2, 2, f32> for Matrix22f32 {
    type Result = Matrix<1, 1, f32>;
}

impl Minor<4, 4, f32> for Matrix44f32 {}
impl Minor<3, 3, f32> for Matrix33f32 {}
impl Minor<2, 2, f32> for Matrix22f32 {}

impl Cofactor<4, 4, f32> for Matrix44f32 {}
impl Cofactor<3, 3, f32> for Matrix33f32 {}
impl Cofactor<2, 2, f32> for Matrix22f32 {}

impl Invert<4, 4, f32> for Matrix44f32 {}
impl Invert<3, 3, f32> for Matrix33f32 {}
impl Invert<2, 2, f32> for Matrix22f32 {}
