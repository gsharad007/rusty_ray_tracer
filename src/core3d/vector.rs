#[cfg(test)]
extern crate test;
use derive_more::{Index, IndexMut};
#[cfg(test)]
use test::Bencher;
#[cfg(test)]
const N: i32 = 1000;

use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use super::{array_base::ArrayBase, coordinates4::Coordinates4, dot_product::DotProduct};
use crate::core3d::tuple::Tuple;
use float_cmp::{approx_eq, ApproxEq};

/// A Vector in 3D (x,y,z) space is a 4 unit (x,y,z,w) set with the `w` value being 0.0 to ignore translations from matrices
#[derive(Copy, Clone, Default, Debug, Index, IndexMut)]
pub struct Vector {
    pub tuple: [f32; 4],
}
impl Vector {
    /// Creates a new Vector
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    ///
    /// let a = Vector::new(1.0, 2.0, 3.0);
    /// assert_eq!(1.0, a.x());
    /// assert_eq!(2.0, a.y());
    /// assert_eq!(3.0, a.z());
    /// assert_eq!(0.0, a.w());
    /// assert!(a.is_vector() == true);
    /// assert!(a.is_point() == false);
    /// assert!(a.is_valid());
    /// ```
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            tuple: [x, y, z, 0.0],
        }
    }
}

#[cfg(test)]
mod tests_vector {
    use super::*;

    #[test]
    fn new() {
        let a = Vector::new(1.0, 2.0, 3.0);
        assert_eq!([1.0, 2.0, 3.0, 0.0], a.tuple);
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn copy_clone() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let a_copy = a;
        let a_clone = a.clone();
        assert_eq!([1.0, 2.0, 3.0, 0.0], a_copy.tuple);
        assert_eq!([1.0, 2.0, 3.0, 0.0], a_clone.tuple);
    }

    #[test]
    fn debug_fmt() {
        let a = Vector::new(1.0, 2.0, 3.0);
        assert_eq!("Vector { tuple: [1.0, 2.0, 3.0, 0.0] }", format!("{a:?}"));
    }
}

#[cfg(test)]
mod benchs_vector {
    use super::*;

    #[allow(clippy::cast_precision_loss)]
    #[bench]
    fn new(bench: &mut Bencher) {
        let a = Vector::new(1.0, 2.0, 3.0);
        bench.iter(|| {
            (0..N).fold(a, |a, b| {
                Vector::new(a.x() + b as f32, a.y() + b as f32, a.z() + b as f32)
            })
        });
    }

    #[bench]
    fn copy(bench: &mut Bencher) {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(4.0, 5.0, 6.0);
        bench.iter(|| (0..N).fold(a, |a, i| if i == 0 { a } else { b }));
    }

    #[bench]
    #[allow(clippy::clone_on_copy)]
    fn clone(bench: &mut Bencher) {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(4.0, 5.0, 6.0);
        bench.iter(|| (0..N).fold(a, |a, i| if i == 0 { a.clone() } else { b.clone() }));
    }

    #[bench]
    fn debug_fmt(bench: &mut Bencher) {
        let a = Vector::new(1.0, 2.0, 3.0);
        bench.iter(|| (0..N).fold(String::default(), |_, _| format!("{a:?}")));
    }
}

impl From<[f32; 3]> for Vector {
    /// Creates a new vector from an array of scaler values
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let a = Vector::from([1.0, 2.0, 3.0]);
    /// assert_eq!([1.0, 2.0, 3.0, 0.0], a.tuple);
    /// ```
    fn from(arr: [f32; 3]) -> Self {
        Self::new(arr[0], arr[1], arr[2])
    }
}

impl From<Tuple> for Vector {
    /// Creates a new Vector from a Tuple
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let a = Vector::from(Tuple::from([1.0, 2.0, 3.0, 0.0]));
    /// assert_eq!([1.0, 2.0, 3.0, 0.0], a.tuple);
    /// ```
    ///
    /// ```
    /// # use std::panic;
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let a = Tuple::from([1.0, 2.0, 3.0, 4.0]);
    /// assert!(panic::catch_unwind(|| Vector::from(a)).is_err());
    /// ```
    fn from(tuple: Tuple) -> Self {
        debug_assert!(tuple.is_vector());
        Self::new(tuple.x(), tuple.y(), tuple.z())
    }
}

impl From<Vector> for Tuple {
    /// Creates a new Tuple from a Vector
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let a = Tuple::from(Vector::from([1.0, 2.0, 3.0]));
    /// assert_eq!([1.0, 2.0, 3.0, 0.0], a.tuple);
    /// ```
    fn from(vector: Vector) -> Self {
        debug_assert!(vector.is_vector());
        Self::from(vector.tuple)
    }
}

#[cfg(test)]
mod tests_from {
    use super::*;
    use std::panic;

    #[test]
    fn from_array() {
        let a = Vector::from([1.0, 2.0, 3.0]);
        assert_eq!([1.0, 2.0, 3.0, 0.0], a.tuple);
    }

    #[test]
    fn from_tuple() {
        let a = Vector::from(Tuple::new(1.0, 2.0, 3.0, 0.0));
        assert_eq!([1.0, 2.0, 3.0, 0.0], a.tuple);

        let tuple = Tuple::from([1.0, 2.0, 3.0, 4.0]);
        panic::catch_unwind(|| Vector::from(tuple)).unwrap_err();
    }

    #[test]
    fn into_tuple() {
        let a = Tuple::from(Vector::new(1.0, 2.0, 3.0));
        assert_eq!([1.0, 2.0, 3.0, 0.0], a.tuple);

        let a: Tuple = Vector::new(1.0, 2.0, 3.0).into();
        assert_eq!([1.0, 2.0, 3.0, 0.0], a.tuple);
    }
}

impl ArrayBase for Vector {
    type Item = f32;
    // type SizedArray = [f32; 4];

    /// Returns base array consuming
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let a = Vector::new(1.0, 2.0, 3.0);
    /// assert_eq!([1.0, 2.0, 3.0, 0.0], a.get_array());
    /// ```
    fn get_array(self) -> [f32; 4] {
        self.tuple
    }

    /// Returns base array reference
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let a = Vector::new(1.0, 2.0, 3.0);
    /// assert_eq!([1.0, 2.0, 3.0, 0.0], *a.get_array_ref());
    /// ```
    fn get_array_ref(&self) -> &[f32; 4] {
        &self.tuple
    }

    /// Returns a mutable base array reference
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let mut a = Vector::new(1.0, 2.0, 3.0);
    /// assert_eq!([1.0, 2.0, 3.0, 0.0], *a.get_array_mut());
    /// a.get_array_mut()[0] += 10.0;
    /// a.get_array_mut()[1] += 10.0;
    /// a.get_array_mut()[2] += 10.0;
    /// a.get_array_mut()[3] += 10.0;
    /// assert_eq!([11.0, 12.0, 13.0, 10.0], *a.get_array_mut());
    /// ```
    fn get_array_mut(&mut self) -> &mut [f32; 4] {
        &mut self.tuple
    }
}

#[cfg(test)]
mod tests_array_base {
    use super::*;

    #[test]
    fn get_array() {
        let a = Vector::new(1.0, 2.0, 3.0);
        assert_eq!([1.0, 2.0, 3.0, 0.0], *a.get_array_ref());
        assert_eq!([1.0, 2.0, 3.0, 0.0], a.get_array());
        assert_eq!([1.0, 2.0, 3.0, 0.0], *a.get_array_ref());
    }

    #[test]
    fn get_array_mut() {
        let mut a = Vector::new(1.0, 2.0, 3.0);
        assert_eq!([1.0, 2.0, 3.0, 0.0], *a.get_array_mut());
        a.get_array_mut()[0] += 10.0;
        a.get_array_mut()[1] += 10.0;
        a.get_array_mut()[2] += 10.0;
        a.get_array_mut()[3] += 10.0;
        assert_eq!([11.0, 12.0, 13.0, 10.0], *a.get_array_mut());
        assert_eq!([11.0, 12.0, 13.0, 10.0], a.get_array());
        assert_eq!([11.0, 12.0, 13.0, 10.0], *a.get_array_ref());
    }
}

impl Coordinates4 for Vector {}

#[cfg(test)]
mod tests_coordinates4 {
    use super::*;
    use crate::core3d::coordinates4::Coordinates4;

    #[test]
    fn assign_array() {
        let a = Vector::from([3.0, 2.0, 1.0]);
        assert_eq!(3.0, a.x());
        assert_eq!(2.0, a.y());
        assert_eq!(1.0, a.z());
        assert_eq!(0.0, a.w());
        assert!(a.is_vector());
        assert!(!a.is_point());
        assert!(a.is_valid());
    }

    #[test]
    fn create_new() {
        let a = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(1.0, a.x());
        assert_eq!(2.0, a.y());
        assert_eq!(3.0, a.z());
        assert_eq!(0.0, a.w());
        assert!(a.is_vector());
        assert!(!a.is_point());
        assert!(a.is_valid());
    }
}

impl Display for Vector {
    /// Returns a string representation of the Point object as [{x}, {y}, {z}, {w}]
    ///
    /// ```
    /// use rusty_ray_tracer::core3d::vector::Vector;
    ///
    /// let a = Vector::new(1.0, 2.0, 3.0);
    /// assert_eq!("[1, 2, 3, 0]", format!("{}", a));
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}, {}, {}, {}]",
            self.get_at(0),
            self.get_at(1),
            self.get_at(2),
            self.get_at(3),
        )
    }
}

#[cfg(test)]
mod tests_display {
    use super::*;

    #[test]
    fn display() {
        let a = Vector::new(1.0, 2.0, 3.0);
        assert_eq!("[1, 2, 3, 0]", format!("{a}"));
    }
}

#[cfg(test)]
mod benchs_display {
    use super::*;

    #[bench]
    fn display(bench: &mut Bencher) {
        let a = Vector::new(1.0, 2.0, 3.0);
        bench.iter(|| (0..N).fold(String::default(), |_, _| format!("{a}")));
    }
}

impl PartialEq for Vector {
    /// Performs the `=` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let a = Vector::new(1.23, 4.56, 0.0);
    /// let b = Vector::new(1.23, 4.56, 0.0);
    /// assert_eq!(a, b);
    /// ```
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let a = Vector::new(1.23, 4.56, 1.000000);
    /// let b = Vector::new(1.23, 4.56, 1.000001);
    /// assert_ne!(a, b);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        Self::zip(self, other).all(|(a, b)| approx_eq!(f32, *a, *b))
    }
}

#[cfg(test)]
mod tests_eq {
    use super::*;

    #[test]
    fn eq() {
        assert_eq!(
            Vector::new(1.23, 4.56, 0.000_000_000_000_00),
            Vector::new(1.23, 4.56, 0.000_000_000_000_01)
        );
        assert_eq!(
            Vector::new(1.23, 4.56, 0.000_000_0),
            Vector::new(1.23, 4.56, 0.000_000_1)
        );
        assert_eq!(
            Vector::new(1.23, 4.56, 1.000_000_0),
            Vector::new(1.23, 4.56, 1.000_000_1)
        );
        assert_eq!(
            Vector::new(1.23, 4.56, 1_000_000.0),
            Vector::new(1.23, 4.56, 1_000_000.1)
        );
    }

    #[test]
    fn ne() {
        assert_ne!(
            Vector::new(1.23, 4.56, 0.000_010),
            Vector::new(1.23, 4.56, 0.000_011)
        );
        assert_ne!(
            Vector::new(1.23, 4.56, 1.000_000),
            Vector::new(1.23, 4.56, 1.000_001)
        );
        assert_ne!(
            Vector::new(1.23, 4.56, 100_000.0),
            Vector::new(1.23, 4.56, 100_000.1)
        );
    }
}

impl ApproxEq for Vector {
    type Margin = <f32 as ApproxEq>::Margin;

    /// Performs the `~=` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// # use float_cmp::ApproxEq;
    /// let a = Vector::new(1.23, 4.56, 0.000000000000);
    /// let b = Vector::new(1.23, 4.56, 0.000000000001);
    /// assert!(a.approx_eq(b, <Vector as ApproxEq>::Margin::default()));
    /// ```
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// # use float_cmp::ApproxEq;
    /// let a = Vector::new(1.23, 4.56, 1.0000000);
    /// let b = Vector::new(1.23, 4.56, 1.0000001);
    /// assert!(a.approx_eq(b, <Vector as ApproxEq>::Margin::default().ulps(2)));
    /// ```
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// # use float_cmp::ApproxEq;
    /// let a = Vector::new(1.23, 4.56, 0.0);
    /// let b = Vector::new(1.23, 4.56, 1.0);
    /// assert!(a.approx_eq(b, <Vector as ApproxEq>::Margin::default().epsilon(1.0)));
    /// ```
    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        Self::into_zip(self, other).all(|(a, b)| a.approx_eq(b, margin))
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
            Vector,
            Vector::new(1.23, 4.56, 0.000_000_000_000),
            Vector::new(1.23, 4.56, 0.000_000_000_001)
        );
        assert_approx_eq!(
            Vector,
            Vector::new(1.23, 4.56, 1.000_000_0),
            Vector::new(1.23, 4.56, 1.000_000_1),
            ulps = 2
        );
        assert_approx_eq!(
            Vector,
            Vector::new(1.23, 4.56, 0.0),
            Vector::new(1.23, 4.56, 1.0),
            epsilon = 1.0
        );
    }

    #[test]
    fn ne() {
        {
            let a = Vector::new(1.23, 4.56, 1.000_000);
            let b = Vector::new(1.23, 4.56, 1.000_001);
            assert!(a.approx_ne(b, <Vector as ApproxEq>::Margin::default()));
        }
        {
            let a = Vector::new(1.23, 4.56, 1.000_000);
            let b = Vector::new(1.23, 4.56, 1.000_001);
            assert!(a.approx_ne(b, <Vector as ApproxEq>::Margin::default().ulps(2)));
        }
        {
            let a = Vector::new(1.23, 4.56, 0.000_000_0);
            let b = Vector::new(1.23, 4.56, 1.000_000_1);
            assert!(a.approx_ne(b, <Vector as ApproxEq>::Margin::default().epsilon(1.0)));
        }
    }
}

impl Add for Vector {
    /// The resulting type after applying the `+` operator.
    type Output = Self;

    /// Performs the `+` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let a = Vector::new(1.23, 4.56, 7.89);
    /// let b = Vector::new(1.11, 2.22, 3.33);
    /// let expected = Vector::new(2.34, 6.78, 11.22);
    /// assert_eq!(expected, a + b);
    /// ```
    #[must_use]
    fn add(self, rhs: Self) -> Self {
        Self::zip_for_each_collect(self, rhs, |a, b| a + b)
    }
}

#[cfg(test)]
mod tests_add {
    use super::*;

    #[test]
    fn closure() {
        let a = Vector::new(1.23, 4.56, 7.89);
        let b = Vector::new(1.11, 2.22, 3.33);
        let expected = Vector::new(2.34, 6.78, 11.22);
        assert_eq!(expected, a + b);
    }

    #[test]
    fn identity() {
        let a = Vector::new(1.23, 4.56, 7.89);
        let b = Vector::default();
        assert_eq!(a + b, a);
        assert_eq!(b + a, a);
    }

    #[test]
    fn commutative() {
        let a = Vector::new(1.23, 4.56, 7.89);
        let b = Vector::new(1.11, 2.22, 3.33);
        assert_eq!(a + b, b + a);
    }

    #[test]
    fn associative() {
        let a = Vector::new(1.23, 4.56, 7.89);
        let b = Vector::new(1.11, 2.22, 3.33);
        let c = Vector::new(5.55, 6.66, 7.77);
        assert_eq!(a + (b + c), (a + b) + c);
        assert_eq!(c + (a + b), (c + a) + b);
    }
}

#[cfg(test)]
mod benchs_add {
    use super::*;

    #[bench]
    fn closure(bench: &mut Bencher) {
        let a = Vector::new(1.23, 4.56, 7.89);
        let b = Vector::new(1.11, 2.22, 3.33);
        bench.iter(|| (0..N).fold(a, |a, _| a + b));
    }

    #[bench]
    fn identity(bench: &mut Bencher) {
        let a = Vector::new(1.23, 4.56, 7.89);
        let b = Vector::default();
        bench.iter(|| (0..N).fold(a, |a, _| a + b));
    }

    #[bench]
    fn associative(bench: &mut Bencher) {
        let a = Vector::new(1.23, 4.56, 7.89);
        let b = Vector::new(1.11, 2.22, 3.33);
        let c = Vector::new(5.55, 6.66, 7.77);
        bench.iter(|| (0..N).fold(a, |a, _| a + (b + c)));
    }
}

impl Sub for Vector {
    /// The resulting type after applying the `-` operator.
    type Output = Self;

    /// Performs the `-` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let a = Vector::new(1.23, 4.56, 7.89);
    /// let b = Vector::new(1.11, 2.22, 3.33);
    /// let expected = Vector::new(0.12, 2.34, 4.56);
    /// assert_eq!(expected, a - b);
    /// ```
    #[must_use]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::zip_for_each_collect(self, rhs, |a, b| a - b)
    }
}

#[cfg(test)]
mod tests_sub {
    use super::*;

    #[test]
    fn not_closure() {
        let a = Vector::new(1.23, 4.56, 7.89);
        let b = Vector::new(1.11, 2.22, 3.33);
        let expected = Vector::new(0.12, 2.34, 4.56);
        assert_eq!(expected, a - b);
    }

    #[test]
    fn not_identity() {
        let a = Vector::new(1.23, 4.56, 7.89);
        let b = Vector::default();
        let ab = Vector::new(1.23, 4.56, 7.89);
        assert_eq!(ab, a - b);
        assert_ne!(ab, b - a);
        let ba = Vector::new(-1.23, -4.56, -7.89);
        assert_eq!(ba, b - a);
    }

    #[test]
    fn not_commutative() {
        let a = Vector::new(1.23, 4.56, 7.89);
        let b = Vector::new(1.11, 2.22, 3.33);
        assert_ne!(a - b, b - a);

        let ab = Vector::new(0.12, 2.34, 4.56);
        let ba = Vector::new(-0.12, -2.34, -4.56);
        assert_eq!(ab, a - b);
        assert_eq!(ba, b - a);
    }

    #[test]
    fn not_associative() {
        let a = Vector::new(1.23, 4.56, 7.89);
        let b = Vector::new(1.11, 2.22, 3.33);
        let c = Vector::new(5.55, 6.66, 7.77);
        assert_ne!(a - (b - c), (a - b) - c);
        assert_ne!(c - (a - b), (c - a) - b);

        let a_bc = Vector::new(5.67, 9.0, 12.33);
        let ab_c = Vector::new(-5.43, -4.32, -3.21);
        let c_ab = Vector::new(5.43, 4.32, 3.21);
        let ca_b = Vector::new(3.21, -0.120_000_124, -3.45);
        assert_eq!(a_bc, a - (b - c));
        assert_eq!(ab_c, (a - b) - c);
        assert_eq!(c_ab, c - (a - b));
        assert_eq!(ca_b, (c - a) - b);
    }
}

#[cfg(test)]
mod benchs_sub {
    use super::*;

    #[bench]
    fn not_closure(bench: &mut Bencher) {
        let a = Vector::new(1.23, 4.56, 7.89);
        let b = Vector::new(1.11, 2.22, 3.33);
        bench.iter(|| (0..N).fold(a, |a, _| a - b));
    }

    #[bench]
    fn not_identity(bench: &mut Bencher) {
        let a = Vector::new(1.23, 4.56, 7.89);
        let b = Vector::default();
        bench.iter(|| (0..N).fold(a, |a, _| a - b));
    }

    #[bench]
    fn not_associative(bench: &mut Bencher) {
        let a = Vector::new(1.23, 4.56, 7.89);
        let b = Vector::new(1.11, 2.22, 3.33);
        let c = Vector::new(5.55, 6.66, 7.77);
        bench.iter(|| (0..N).fold(a, |a, _| a - (b - c)));
    }
}

impl Neg for Vector {
    type Output = Self;

    /// Performs the unary `-` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let result = -Vector::new(1.11, -2.22, 3.33);
    /// let expected = Vector::new(-1.11, 2.22, -3.33);
    /// assert_eq!(expected, result);
    /// ```
    fn neg(self) -> Self::Output {
        let mut result = self;
        result.iter_mut().for_each(|x| *x = -(*x));
        result
    }
}

#[cfg(test)]
mod tests_neg {
    use super::*;

    #[test]
    fn neg() {
        assert_eq!(
            Vector::new(-1.23, -4.56, -7.89),
            -Vector::new(1.23, 4.56, 7.89)
        );
        assert_eq!(
            Vector::new(1.23, -4.56, 7.89),
            -Vector::new(-1.23, 4.56, -7.89)
        );
    }

    #[test]
    fn double_neg() {
        assert_eq!(
            Vector::new(1.23, 4.56, 7.89),
            -(-Vector::new(1.23, 4.56, 7.89))
        );
        assert_eq!(
            Vector::new(-1.23, 4.56, -7.89),
            -(-Vector::new(-1.23, 4.56, -7.89))
        );
    }
}

#[cfg(test)]
mod benchs_neg {
    use super::*;

    #[bench]
    fn neg(bench: &mut Bencher) {
        let a = Vector::new(-1.23, -4.56, -7.89);
        bench.iter(|| (0..N).fold(a, |a, _| -a));
    }

    #[bench]
    fn double_neg(bench: &mut Bencher) {
        let a = Vector::new(-1.23, -4.56, -7.89);
        bench.iter(|| (0..N).fold(a, |a, _| -(-a)));
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    /// Performs the `*` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let result = Vector::new(1.11, -2.22, 3.33) * 100.1;
    /// let expected = Vector::new(111.111, -222.222, 333.333);
    /// assert_eq!(expected, result);
    /// ```
    fn mul(self, rhs: f32) -> Self::Output {
        let mut result = self;
        result.iter_mut().for_each(|x| *x *= rhs);
        result
    }
}

#[cfg(test)]
mod tests_mul {
    use super::*;

    #[test]
    fn closure() {
        let result = Vector::new(1.11, -2.22, 3.33) * 100.1;
        let expected = Vector::new(111.111, -222.222, 333.333);
        assert_eq!(result, expected);
    }

    #[test]
    fn identity() {
        let a = Vector::new(1.23, 4.56, 7.89);
        let b = 1.0;
        assert_eq!(a * b, a);
    }
}

#[cfg(test)]
mod benchs_mul {
    use super::*;

    #[allow(clippy::cast_precision_loss)]
    #[bench]
    fn closure(bench: &mut Bencher) {
        let a = Vector::new(1.11, -2.22, 3.33);
        bench.iter(|| (0..N).fold(a, |a, b| a * b as f32));
    }
}

impl Div<f32> for Vector {
    type Output = Self;

    /// Performs the `*` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// let result = Vector::new(1.11, -2.22, 3.33) / 11.1;
    /// let expected = Vector::new(0.1, -0.2, 0.3);
    /// assert_eq!(expected, result);
    ///
    /// assert!((Vector::new(1.23, 4.56, 7.89) / 0.0).tuple[0..3]
    ///     .iter()
    ///     .all(|&f| f.is_infinite()));
    /// assert!((Vector::new(0.0, 0.0, 0.0) / 0.0)
    ///     .into_iter()
    ///     .all(|f| f.is_nan()));
    /// ```
    fn div(self, rhs: f32) -> Self::Output {
        let mut result = self;
        result.iter_mut().for_each(|x| *x /= rhs);
        result
    }
}

#[cfg(test)]
mod tests_div {
    use super::*;

    #[test]
    fn closure() {
        let result = Vector::new(1.11, -2.22, 3.33) / 10.0;
        let expected = Vector::new(0.111, -0.222, 0.333);
        assert_eq!(result, expected);
    }

    #[test]
    fn identity() {
        let a = Vector::new(1.23, 4.56, 7.89);
        let b = 1.0;
        assert_eq!(a / b, a);
    }

    #[test]
    fn div_by_zero() {
        assert!((Vector::new(1.23, 4.56, 7.89) / 0.0).tuple[0..3]
            .iter()
            .all(|&f| f.is_infinite()));
        assert!((Vector::new(0.0, 0.0, 0.0) / 0.0)
            .into_iter()
            .all(f32::is_nan));
    }
}

#[cfg(test)]
mod benchs_div {
    use super::*;

    #[allow(clippy::cast_precision_loss)]
    #[bench]
    fn closure(bench: &mut Bencher) {
        let a = Vector::new(1.11, -2.22, 3.33);
        bench.iter(|| (0..N).fold(a, |a, b| a / b as f32));
    }
}

pub trait Magnitude {
    /// The resulting type
    type Output;

    /// Calculate the magnitude of the Vector
    #[must_use]
    fn magnitude(self) -> Self::Output;
}
impl Magnitude for Vector {
    /// The resulting magnitude type
    type Output = f32;

    /// Calculate the magnitude of the Vector
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// # use crate::rusty_ray_tracer::core3d::vector::Magnitude;
    /// assert_eq!(0.0, Vector::new(0.0, 0.0, 0.0).magnitude());
    /// assert_eq!(1.0, Vector::new(1.0, 0.0, 0.0).magnitude());
    /// assert_eq!(1.0, Vector::new(0.0, 1.0, 0.0).magnitude());
    /// assert_eq!(1.0, Vector::new(0.0, 0.0, 1.0).magnitude());
    /// assert_eq!(1.0, Vector::new(-1.0, 0.0, 0.0).magnitude());
    /// assert_eq!(1.0, Vector::new(0.0, -1.0, 0.0).magnitude());
    /// assert_eq!(1.0, Vector::new(0.0, 0.0, -1.0).magnitude());
    /// ```
    #[must_use]
    #[allow(clippy::suboptimal_flops)]
    fn magnitude(self) -> Self::Output {
        let magnitude_squared = Self::into_iter(self).fold(0.0, |acc, v| acc + (v * v));
        f32::sqrt(magnitude_squared)
    }
}

#[cfg(test)]
mod tests_magnitude {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(0.0, Vector::new(0.0, 0.0, 0.0).magnitude());
        assert_eq!(1.0, Vector::new(1.0, 0.0, 0.0).magnitude());
        assert_eq!(1.0, Vector::new(0.0, 1.0, 0.0).magnitude());
        assert_eq!(1.0, Vector::new(0.0, 0.0, 1.0).magnitude());
        assert_eq!(
            1.0,
            Vector::new(0.577_350_3, 0.577_350_3, 0.577_350_28).magnitude()
        );

        assert_eq!(3.741_657_5, Vector::new(1.0, 2.0, 3.0).magnitude());
        assert_eq!(3.741_657_5, Vector::new(-1.0, -2.0, -3.0).magnitude());
        assert_eq!(9.195_575, Vector::new(1.23, 4.56, 7.89).magnitude());
        assert_eq!(4.153_239_7, Vector::new(1.11, 2.22, 3.33).magnitude());
    }
}

#[cfg(test)]
mod benchs_magnitude {
    use super::*;

    #[allow(clippy::cast_precision_loss)]
    #[bench]
    fn test(bench: &mut Bencher) {
        bench.iter(|| {
            (0..N).fold(0.0, |a, b| {
                a + Vector::new((b + 1) as f32, (b + 2) as f32, (b + 3) as f32).magnitude()
            })
        });
    }
}

pub trait Normalize: Magnitude<Output = f32> + Div<f32, Output = Self> + Copy + Sized {
    /// Normalize the Vector to unit length.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// # use crate::rusty_ray_tracer::core3d::vector::*;
    /// assert_eq!(Vector::new(1.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0).normalize());
    /// assert_eq!(Vector::new(0.0, 1.0, 0.0), Vector::new(0.0, 1.0, 0.0).normalize());
    /// assert_eq!(Vector::new(0.0, 0.0, 1.0), Vector::new(0.0, 0.0, 1.0).normalize());
    /// assert_eq!(Vector::new(0.57735029, 0.57735029, 0.57735028), Vector::new(0.57735029, 0.57735029, 0.57735028).normalize());
    /// ```
    #[must_use]
    fn normalize(self) -> Self {
        self / self.magnitude()
    }
}

impl Normalize for Vector {}

#[cfg(test)]
mod tests_normalize {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0).normalize()
        );
        assert_eq!(
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 1.0, 0.0).normalize()
        );
        assert_eq!(
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(0.0, 0.0, 1.0).normalize()
        );
        assert_eq!(
            Vector::new(0.577_350_3, 0.577_350_3, 0.577_350_28),
            Vector::new(0.577_350_3, 0.577_350_3, 0.577_350_28).normalize()
        );

        assert_eq!(
            Vector::new(0.267_261_24, 0.534_522_5, 0.801_783_7),
            Vector::new(1.0, 2.0, 3.0).normalize()
        );
        assert_eq!(
            Vector::new(-0.267_261_24, -0.534_522_5, -0.801_783_7),
            Vector::new(-1.0, -2.0, -3.0).normalize()
        );
    }

    #[test]
    fn tests_normalize_zero_vector() {
        assert!(Vector::new(0.0, 0.0, 0.0)
            .normalize()
            .into_iter()
            .all(f32::is_nan));
    }
}

impl DotProduct for Vector {}

#[cfg(test)]
mod tests_dot_product {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            0.0,
            Vector::new(0.0, 0.0, 0.0).dot(Vector::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            1.0,
            Vector::new(1.0, 0.0, 0.0).dot(Vector::new(1.0, 0.0, 0.0))
        );
        assert_eq!(
            1.0,
            Vector::new(0.0, 1.0, 0.0).dot(Vector::new(0.0, 1.0, 0.0))
        );
        assert_eq!(
            1.0,
            Vector::new(0.0, 0.0, 1.0).dot(Vector::new(0.0, 0.0, 1.0))
        );
        assert_eq!(
            1.0,
            Vector::new(0.577_350_3, 0.577_350_28, 0.577_350_28).dot(Vector::new(
                0.577_350_3,
                0.577_350_28,
                0.577_350_28
            ))
        );

        assert_eq!(
            14.0,
            Vector::dot(Vector::new(1.0, 2.0, 3.0), Vector::new(1.0, 2.0, 3.0))
        );
        assert_eq!(
            14.0,
            Vector::dot(Vector::new(-1.0, -2.0, -3.0), Vector::new(-1.0, -2.0, -3.0))
        );
        assert_eq!(
            -14.0,
            Vector::dot(Vector::new(1.0, 2.0, 3.0), Vector::new(-1.0, -2.0, -3.0))
        );

        assert_eq!(
            32.0,
            Vector::dot(Vector::new(1.0, 2.0, 3.0), Vector::new(4.0, 5.0, 6.0))
        );
    }
}

#[cfg(test)]
mod benchs_dot_product {
    use super::*;

    #[allow(clippy::cast_precision_loss)]
    #[bench]
    fn test(bench: &mut Bencher) {
        bench.iter(|| {
            (0..N).fold(1.0, |a, b| {
                let a = Vector::new(a, a, a);
                let b = Vector::new(b as f32, b as f32, b as f32);
                a.dot(b)
            })
        });
    }
}

pub trait CrossProduct {
    /// Calculate Dot Product on two Vectors
    #[must_use]
    fn cross(self, other: Self) -> Self;
}
impl CrossProduct for Vector {
    /// Calculate Dot Product on two Vectors
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// # use crate::rusty_ray_tracer::core3d::vector::*;
    /// assert_eq!(Vector::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0).cross(Vector::new(0.0, 0.0, 0.0)));
    /// assert_eq!(Vector::new(0.0, 0.0, 0.0), Vector::new(1.0, 1.0, 1.0).cross(Vector::new(0.0, 0.0, 0.0)));
    /// assert_eq!(Vector::new(0.0, 0.0, 0.0), Vector::new(1.0, 1.0, 1.0).cross(Vector::new(-1.0, -1.0, -1.0)));
    ///
    /// assert_eq!(Vector::new(0.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0).cross(Vector::new(1.0, 0.0, 0.0)));
    /// assert_eq!(Vector::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0).cross(Vector::new(0.0, 1.0, 0.0)));
    /// assert_eq!(Vector::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0).cross(Vector::new(0.0, 0.0, 1.0)));
    /// assert_eq!(Vector::new(0.0, 0.0, 0.0), Vector::new(0.57735029, 0.57735028, 0.57735028).cross(Vector::new(0.57735029, 0.57735028, 0.57735028)));
    ///
    /// assert_eq!(Vector::new(0.0, 0.0, 1.0), Vector::new(1.0, 0.0, 0.0).cross(Vector::new(0.0, 1.0, 0.0)));
    /// assert_eq!(Vector::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0).cross(Vector::new(0.0, 0.0, 1.0)));
    /// assert_eq!(Vector::new(0.0, 1.0, 0.0), Vector::new(0.0, 0.0, 1.0).cross(Vector::new(1.0, 0.0, 0.0)));
    ///
    /// assert_eq!(Vector::new(-1.0, 0.0, 1.0), Vector::new(1.0, 0.0, 1.0).cross(Vector::new(0.0, 1.0, 0.0)));
    /// assert_eq!(Vector::new(1.0, -1.0, 0.0), Vector::new(1.0, 1.0, 0.0).cross(Vector::new(0.0, 0.0, 1.0)));
    /// assert_eq!(Vector::new(0.0, 1.0, -1.0), Vector::new(0.0, 1.0, 1.0).cross(Vector::new(1.0, 0.0, 0.0)));
    /// ```
    #[must_use]
    fn cross(self, other: Self) -> Self {
        Self::new(
            self.get_array()[1].mul_add(
                other.get_array()[2],
                -self.get_array()[2] * other.get_array()[1],
            ),
            self.get_array()[2].mul_add(
                other.get_array()[0],
                -self.get_array()[0] * other.get_array()[2],
            ),
            self.get_array()[0].mul_add(
                other.get_array()[1],
                -self.get_array()[1] * other.get_array()[0],
            ),
        )
        // Self::into_iter(self)
        //     .zip(other.into_iter())
        //     .fold(0.0, |acc, v| acc + (v.0 * v.1))
    }
}

#[cfg(test)]
mod tests_cross_product {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 0.0, 0.0).cross(Vector::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 1.0, 1.0).cross(Vector::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 1.0, 1.0).cross(Vector::new(-1.0, -1.0, -1.0))
        );

        assert_eq!(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0).cross(Vector::new(1.0, 0.0, 0.0))
        );
        assert_eq!(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0).cross(Vector::new(0.0, 1.0, 0.0))
        );
        assert_eq!(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 0.0, 1.0).cross(Vector::new(0.0, 0.0, 1.0))
        );
        assert_eq!(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(0.577_350_3, 0.577_350_28, 0.577_350_28).cross(Vector::new(
                0.577_350_3,
                0.577_350_28,
                0.577_350_28
            ))
        );

        assert_eq!(
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(1.0, 0.0, 0.0).cross(Vector::new(0.0, 1.0, 0.0))
        );
        assert_eq!(
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0).cross(Vector::new(0.0, 0.0, 1.0))
        );
        assert_eq!(
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 0.0, 1.0).cross(Vector::new(1.0, 0.0, 0.0))
        );

        assert_eq!(
            Vector::new(-1.0, 0.0, 1.0),
            Vector::new(1.0, 0.0, 1.0).cross(Vector::new(0.0, 1.0, 0.0))
        );
        assert_eq!(
            Vector::new(1.0, -1.0, 0.0),
            Vector::new(1.0, 1.0, 0.0).cross(Vector::new(0.0, 0.0, 1.0))
        );
        assert_eq!(
            Vector::new(0.0, 1.0, -1.0),
            Vector::new(0.0, 1.0, 1.0).cross(Vector::new(1.0, 0.0, 0.0))
        );

        assert_eq!(
            Vector::new(0.0, 0.0, 0.0),
            Vector::cross(Vector::new(1.0, 2.0, 3.0), Vector::new(1.0, 2.0, 3.0))
        );
        assert_eq!(
            Vector::new(0.0, 0.0, 0.0),
            Vector::cross(Vector::new(-1.0, -2.0, -3.0), Vector::new(-1.0, -2.0, -3.0))
        );
        assert_eq!(
            Vector::new(0.0, 0.0, 0.0),
            Vector::cross(Vector::new(1.0, 2.0, 3.0), Vector::new(-1.0, -2.0, -3.0))
        );

        assert_eq!(
            Vector::new(-3.0, 6.0, -3.0),
            Vector::cross(Vector::new(1.0, 2.0, 3.0), Vector::new(4.0, 5.0, 6.0))
        );

        assert_eq!(
            Vector::new(3.0, -6.0, 3.0),
            Vector::cross(Vector::new(4.0, 5.0, 6.0), Vector::new(1.0, 2.0, 3.0))
        );
    }

    #[test]
    fn cross_cross() {
        let a = Vector::new(1.0, 0.0, 1.0).normalize();
        let b = Vector::new(0.0, 1.0, 0.0).normalize();
        assert_eq!(a, b.cross(a).cross(b));
        assert_eq!(b, a.cross(b).cross(a));
    }
}

#[cfg(test)]
mod benchs_cross_product {
    use super::*;

    #[allow(clippy::cast_precision_loss)]
    #[bench]
    fn test(bench: &mut Bencher) {
        let a = Vector::new(1.11, -2.22, 3.33);
        bench.iter(|| {
            (0..N).fold(a, |a, b| {
                let b = Vector::new(b as f32, b as f32, b as f32);
                a.cross(b)
            })
        });
    }
}
