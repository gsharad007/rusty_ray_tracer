#[cfg(test)]
extern crate test;
#[cfg(test)]
use test::{black_box, Bencher};
#[cfg(test)]
const N: i32 = 1000;

use super::{array_base::ArrayBase, coordinates4::Coordinates4};
use core::ops::Add;
use float_cmp::{approx_eq, ApproxEq};
use std::{
    fmt::Display,
    ops::{Div, Mul, Neg, Sub},
};

#[derive(Copy, Clone, Default, Debug)]
pub struct Tuple {
    pub tuple: [f32; 4],
}
impl Tuple {
    /// Creates a new tuple from x, y, z, w scaler values
    ///
    /// # Examples
    ///
    /// ```
    /// # use rusty_ray_tracer::core3d::tuple::Tuple;
    ///
    /// let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
    /// assert_eq!([1.0, 2.0, 3.0, 4.0], a.tuple);
    /// ```
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            tuple: [x, y, z, w],
        }
    }
}

#[cfg(test)]
mod tests_tuple {
    use super::*;

    #[test]
    fn new() {
        let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!([1.0, 2.0, 3.0, 4.0], a.tuple);
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn clone() {
        let a = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let a_copy = a;
        let a_clone = a_copy.clone();
        assert_eq!([1.0, 2.0, 3.0, 1.0], a_copy.tuple);
        assert_eq!([1.0, 2.0, 3.0, 1.0], a_clone.tuple);
    }

    #[test]
    fn debug_fmt() {
        let a = Tuple::new(1.0, 2.0, 3.0, 1.0);
        assert_eq!("Tuple { tuple: [1.0, 2.0, 3.0, 1.0] }", format!("{a:?}"));
    }
}

#[cfg(test)]
mod benchs_tuple {
    use super::*;

    const A: Tuple = Tuple::new(1.0, 2.0, 3.0, 4.0);
    const B: Tuple = Tuple::new(5.0, 6.0, 7.0, 8.0);

    #[bench]
    fn new(bench: &mut Bencher) {
        bench.iter(|| {
            (0..N).fold(A, |a, b| {
                Tuple::new(
                    a.tuple[0] + b as f32,
                    a.tuple[1] + b as f32,
                    a.tuple[2] + b as f32,
                    a.tuple[3] + b as f32,
                )
            })
        });
    }

    #[bench]
    fn copy(bench: &mut Bencher) {
        bench.iter(|| (0..N).fold(A, |a, i| if i == 0 { a } else { B }));
    }

    #[bench]
    #[allow(clippy::clone_on_copy)]
    fn clone(bench: &mut Bencher) {
        bench.iter(|| (0..N).fold(A, |a, i| if i == 0 { a.clone() } else { B.clone() }));
    }

    #[bench]
    fn debug_fmt(bench: &mut Bencher) {
        bench.iter(|| (0..N).fold(String::default(), |_, _| format!("{A:?}")));
    }
}

impl From<[f32; 4]> for Tuple {
    /// Creates a new tuple from an array of scaler values
    ///
    /// # Examples
    ///
    /// ```
    /// # use rusty_ray_tracer::core3d::tuple::Tuple;
    ///
    /// let a = Tuple::from([1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!([1.0, 2.0, 3.0, 4.0], a.tuple);
    /// ```
    fn from(arr: [f32; 4]) -> Self {
        Self { tuple: arr }
    }
}

#[cfg(test)]
mod tests_from {
    use super::*;

    #[test]
    fn from_array() {
        let a = Tuple::from([1.0, 2.0, 3.0, 4.0]);
        assert_eq!([1.0, 2.0, 3.0, 4.0], a.tuple);
    }
}

#[cfg(test)]
mod benchs_from {
    use super::*;

    #[bench]
    fn from_array(bench: &mut Bencher) {
        let arr = black_box([1.0, 2.0, 3.0, 4.0]);
        bench.iter(|| (0..N).map(|_| Tuple::from(arr)));
    }
}

impl ArrayBase for Tuple {
    type Item = f32;
    // type SizedArray = [f32; 4];

    /// Returns base array consuming
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
    /// assert_eq!([1.0, 2.0, 3.0, 4.0], a.get_array());
    /// ```
    fn get_array(self) -> [f32; 4] {
        self.tuple
    }

    /// Returns base array reference
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
    /// assert_eq!([1.0, 2.0, 3.0, 4.0], *a.get_array_ref());
    /// ```
    fn get_array_ref(&self) -> &[f32; 4] {
        &self.tuple
    }

    /// Returns a mutable base array reference
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// let mut a = Tuple::new(1.0, 2.0, 3.0, 4.0);
    /// assert_eq!([1.0, 2.0, 3.0, 4.0], *a.get_array_mut());
    /// a.get_array_mut()[0] += 10.0;
    /// a.get_array_mut()[1] += 10.0;
    /// a.get_array_mut()[2] += 10.0;
    /// a.get_array_mut()[3] += 10.0;
    /// assert_eq!([11.0, 12.0, 13.0, 14.0], *a.get_array_mut());
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
        let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!([1.0, 2.0, 3.0, 4.0], *a.get_array_ref());
        assert_eq!([1.0, 2.0, 3.0, 4.0], a.get_array());
        assert_eq!([1.0, 2.0, 3.0, 4.0], *a.get_array_ref());
    }

    #[test]
    fn get_array_mut() {
        let mut a = Tuple::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!([1.0, 2.0, 3.0, 4.0], *a.get_array_mut());
        a.get_array_mut()[0] += 10.0;
        a.get_array_mut()[1] += 10.0;
        a.get_array_mut()[2] += 10.0;
        a.get_array_mut()[3] += 10.0;
        assert_eq!([11.0, 12.0, 13.0, 14.0], *a.get_array_mut());
        assert_eq!([11.0, 12.0, 13.0, 14.0], a.get_array());
        assert_eq!([11.0, 12.0, 13.0, 14.0], *a.get_array_ref());
    }
}

impl Coordinates4 for Tuple {}

#[cfg(test)]
mod tests_coordinates4 {
    use super::*;
    use crate::core3d::coordinates4::Coordinates4;

    #[test]
    fn assign_array() {
        let a = Tuple::from([3.0, 2.0, 1.0, 0.0]);
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
        let a = Tuple::new(1.0, 2.0, 3.0, 0.0);
        assert_eq!(1.0, a.x());
        assert_eq!(2.0, a.y());
        assert_eq!(3.0, a.z());
        assert_eq!(0.0, a.w());
        assert!(a.is_vector());
        assert!(!a.is_point());
        assert!(a.is_valid());
    }
}

impl Display for Tuple {
    /// Returns a string representation of the Point object as [{x}, {y}, {z}, {w}]
    ///
    /// ```
    /// use rusty_ray_tracer::core3d::tuple::Tuple;
    ///
    /// let a = Tuple::new(1.0, 2.0, 3.0, 0.0);
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
        let a = Tuple::new(1.0, 2.0, 3.0, 0.0);
        assert_eq!("[1, 2, 3, 0]", format!("{a}"));
    }
}

#[cfg(test)]
mod benchs_display {
    use super::*;

    #[bench]
    fn display(bench: &mut Bencher) {
        let a = Tuple::new(1.0, 2.0, 3.0, 0.0);
        bench.iter(|| (0..N).fold(String::default(), |_, _| format!("{a}")));
    }
}

impl PartialEq for Tuple {
    /// Performs the `=` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// let a = Tuple::new(1.23, 4.56, 7.89, 0.0);
    /// let b = Tuple::new(1.23, 4.56, 7.89, 0.0);
    /// assert_eq!(a, b);
    /// ```
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// let a = Tuple::new(1.23, 4.56, 7.89, 1.000000);
    /// let b = Tuple::new(1.23, 4.56, 7.89, 1.000001);
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
            Tuple::new(1.23, 4.56, 7.89, 0.000_000_000_000_00),
            Tuple::new(1.23, 4.56, 7.89, 0.000_000_000_000_01)
        );
        assert_eq!(
            Tuple::new(1.23, 4.56, 7.89, 0.000_000_0),
            Tuple::new(1.23, 4.56, 7.89, 0.000_000_1)
        );
        assert_eq!(
            Tuple::new(1.23, 4.56, 7.89, 1.000_000_0),
            Tuple::new(1.23, 4.56, 7.89, 1.000_000_1)
        );
        assert_eq!(
            Tuple::new(1.23, 4.56, 7.89, 1_000_000.0),
            Tuple::new(1.23, 4.56, 7.89, 1_000_000.1)
        );
    }

    #[test]
    fn ne() {
        assert_ne!(
            Tuple::new(1.23, 4.56, 7.89, 0.000_010),
            Tuple::new(1.23, 4.56, 7.89, 0.000_011)
        );
        assert_ne!(
            Tuple::new(1.23, 4.56, 7.89, 1.000_000),
            Tuple::new(1.23, 4.56, 7.89, 1.000_001)
        );
        assert_ne!(
            Tuple::new(1.23, 4.56, 7.89, 100_000.0),
            Tuple::new(1.23, 4.56, 7.89, 100_000.1)
        );
    }
}

impl ApproxEq for Tuple {
    type Margin = <f32 as ApproxEq>::Margin;

    /// Performs the `~=` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// # use float_cmp::ApproxEq;
    /// let a = Tuple::new(1.23, 4.56, 7.89, 0.000000000000);
    /// let b = Tuple::new(1.23, 4.56, 7.89, 0.000000000001);
    /// assert!(a.approx_eq(b, <Tuple as ApproxEq>::Margin::default()));
    /// ```
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// # use float_cmp::ApproxEq;
    /// let a = Tuple::new(1.23, 4.56, 7.89, 1.0000000);
    /// let b = Tuple::new(1.23, 4.56, 7.89, 1.0000001);
    /// assert!(a.approx_eq(b, <Tuple as ApproxEq>::Margin::default().ulps(2)));
    /// ```
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// # use float_cmp::ApproxEq;
    /// let a = Tuple::new(1.23, 4.56, 7.89, 0.0);
    /// let b = Tuple::new(1.23, 4.56, 7.89, 1.0);
    /// assert!(a.approx_eq(b, <Tuple as ApproxEq>::Margin::default().epsilon(1.0)));
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
            Tuple,
            Tuple::new(1.23, 4.56, 7.89, 0.000_000_000_000),
            Tuple::new(1.23, 4.56, 7.89, 0.000_000_000_001)
        );
        assert_approx_eq!(
            Tuple,
            Tuple::new(1.23, 4.56, 7.89, 1.000_000_0),
            Tuple::new(1.23, 4.56, 7.89, 1.000_000_1),
            ulps = 2
        );
        assert_approx_eq!(
            Tuple,
            Tuple::new(1.23, 4.56, 7.89, 0.0),
            Tuple::new(1.23, 4.56, 7.89, 1.0),
            epsilon = 1.0
        );
    }

    #[test]
    fn ne() {
        {
            let a = Tuple::new(1.23, 4.56, 7.89, 1.000_000);
            let b = Tuple::new(1.23, 4.56, 7.89, 1.000_001);
            assert!(a.approx_ne(b, <Tuple as ApproxEq>::Margin::default()));
        }
        {
            let a = Tuple::new(1.23, 4.56, 7.89, 1.000_000);
            let b = Tuple::new(1.23, 4.56, 7.89, 1.000_001);
            assert!(a.approx_ne(b, <Tuple as ApproxEq>::Margin::default().ulps(2)));
        }
        {
            let a = Tuple::new(1.23, 4.56, 7.89, 0.000_000_0);
            let b = Tuple::new(1.23, 4.56, 7.89, 1.000_000_1);
            assert!(a.approx_ne(b, <Tuple as ApproxEq>::Margin::default().epsilon(1.0)));
        }
    }
}

impl Add for Tuple {
    /// The resulting type after applying the `+` operator.
    type Output = Self;

    /// Performs the `+` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// let a = Tuple::new(1.23, 4.56, 7.89, 0.0);
    /// let b = Tuple::new(1.11, 2.22, 3.33, 1.0);
    /// let expected = Tuple::new(2.34, 6.78, 11.22, 1.0);
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
        let a = Tuple::new(1.23, 4.56, 7.89, 0.0);
        let b = Tuple::new(1.11, 2.22, 3.33, 1.0);
        let expected = Tuple::new(2.34, 6.78, 11.22, 1.0);
        assert_eq!(expected, a + b);
    }

    #[test]
    fn identity() {
        let a = Tuple::new(1.23, 4.56, 7.89, 0.0);
        let b = Tuple::default();
        assert_eq!(a + b, a);
        assert_eq!(b + a, a);
    }

    #[test]
    fn commutative() {
        let a = Tuple::new(1.23, 4.56, 7.89, 0.0);
        let b = Tuple::new(1.11, 2.22, 3.33, 1.0);
        assert_eq!(a + b, b + a);
    }

    #[test]
    fn associative() {
        let a = Tuple::new(1.23, 4.56, 7.89, 1.01);
        let b = Tuple::new(1.11, 2.22, 3.33, 4.44);
        let c = Tuple::new(5.55, 6.66, 7.77, 8.88);
        assert_eq!(a + (b + c), (a + b) + c);
        assert_eq!(c + (a + b), (c + a) + b);
    }
}

#[cfg(test)]
mod benchs_add {
    use super::*;

    const A: Tuple = Tuple::new(1.23, 4.56, 7.89, 10.11);
    const B: Tuple = Tuple::new(1.11, 2.22, 3.33, 4.44);
    const C: Tuple = Tuple::new(5.55, 6.66, 7.77, 8.88);

    #[bench]
    fn closure(bench: &mut Bencher) {
        bench.iter(|| (0..N).fold(A, |a, _| a + B));
    }

    #[bench]
    fn identity(bench: &mut Bencher) {
        let b = Tuple::default();
        bench.iter(|| (0..N).fold(A, |a, _| a + b));
    }

    #[bench]
    fn associative(bench: &mut Bencher) {
        bench.iter(|| (0..N).fold(A, |a, _| a + (black_box(B) + black_box(C))));
    }
}

impl Sub for Tuple {
    /// The resulting type after applying the `-` operator.
    type Output = Self;

    /// Performs the `-` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// let a = Tuple::new(1.23, 4.56, 7.89, 10.11);
    /// let b = Tuple::new(1.11, 2.22, 3.33, 4.44);
    /// let expected = Tuple::new(0.12, 2.34, 4.56, 5.67);
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
        let a = Tuple::new(1.23, 4.56, 7.89, 10.11);
        let b = Tuple::new(1.11, 2.22, 3.33, 4.44);
        let expected = Tuple::new(0.12, 2.34, 4.56, 5.67);
        assert_eq!(expected, a - b);
    }

    #[test]
    fn not_identity() {
        let a = Tuple::new(1.23, 4.56, 7.89, 10.11);
        let b = Tuple::default();
        let ab = Tuple::new(1.23, 4.56, 7.89, 10.11);
        assert_eq!(ab, a - b);
        assert_ne!(ab, b - a);
        let ba = Tuple::new(-1.23, -4.56, -7.89, -10.11);
        assert_eq!(ba, b - a);
    }

    #[test]
    fn not_commutative() {
        let a = Tuple::new(1.23, 4.56, 7.89, 10.11);
        let b = Tuple::new(1.11, 2.22, 3.33, 4.44);
        assert_ne!(a - b, b - a);

        let ab = Tuple::new(0.12, 2.34, 4.56, 5.67);
        let ba = Tuple::new(-0.12, -2.34, -4.56, -5.67);
        assert_eq!(ab, a - b);
        assert_eq!(ba, b - a);
    }

    #[test]
    fn not_associative() {
        let a = Tuple::new(1.23, 4.56, 7.89, 10.11);
        let b = Tuple::new(1.11, 2.22, 3.33, 4.44);
        let c = Tuple::new(5.55, 6.66, 7.77, 8.88);
        assert_ne!(a - (b - c), (a - b) - c);
        assert_ne!(c - (a - b), (c - a) - b);

        let a_bc = Tuple::new(5.67, 9.0, 12.33, 14.55);
        let ab_c = Tuple::new(-5.43, -4.32, -3.21, -3.21);
        let c_ab = Tuple::new(5.43, 4.32, 3.21, 3.21);
        let ca_b = Tuple::new(3.21, -0.120_000_124, -3.45, -5.67);
        assert_eq!(a_bc, a - (b - c));
        assert_eq!(ab_c, (a - b) - c);
        assert_eq!(c_ab, c - (a - b));
        assert_eq!(ca_b, (c - a) - b);
    }
}

#[cfg(test)]
mod benchs_sub {
    use super::*;

    const A: Tuple = Tuple::new(1.23, 4.56, 7.89, 10.11);
    const B: Tuple = Tuple::new(1.11, 2.22, 3.33, 4.44);
    const C: Tuple = Tuple::new(5.55, 6.66, 7.77, 8.88);

    #[bench]
    fn not_closure(bench: &mut Bencher) {
        bench.iter(|| (0..N).fold(A, |a, _| a - B));
    }

    #[bench]
    fn not_identity(bench: &mut Bencher) {
        let b = Tuple::default();
        bench.iter(|| (0..N).fold(A, |a, _| a - b));
    }

    #[bench]
    fn not_associative(bench: &mut Bencher) {
        bench.iter(|| (0..N).fold(A, |a, _| a - (black_box(B) - black_box(C))));
    }
}

impl Neg for Tuple {
    type Output = Self;

    /// Performs the unary `-` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// let result = -Tuple::new(1.11, -2.22, 3.33, 0.0);
    /// let expected = Tuple::new(-1.11, 2.22, -3.33, 0.0);
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
            Tuple::new(-1.23, -4.56, -7.89, 0.0),
            -Tuple::new(1.23, 4.56, 7.89, 0.0)
        );
        assert_eq!(
            Tuple::new(1.23, -4.56, 7.89, -1.0),
            -Tuple::new(-1.23, 4.56, -7.89, 1.0)
        );
    }

    #[test]
    fn double_neg() {
        assert_eq!(
            Tuple::new(1.23, 4.56, 7.89, 0.0),
            -(-Tuple::new(1.23, 4.56, 7.89, 0.0))
        );
        assert_eq!(
            Tuple::new(-1.23, 4.56, -7.89, 1.0),
            -(-Tuple::new(-1.23, 4.56, -7.89, 1.0))
        );
    }
}

#[cfg(test)]
mod benchs_neg {
    use super::*;

    const A: Tuple = Tuple::new(-1.23, -4.56, -7.89, 0.0);

    #[bench]
    fn neg(bench: &mut Bencher) {
        bench.iter(|| (0..N).fold(A, |a, _| -a));
    }

    #[bench]
    fn double_neg(bench: &mut Bencher) {
        bench.iter(|| (0..N).fold(A, |a, _| -(-a)));
    }
}

impl Mul<f32> for Tuple {
    type Output = Self;

    /// Performs the `*` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// let result = Tuple::new(1.11, -2.22, 3.33, 0.0) * 100.1;
    /// let expected = Tuple::new(111.111, -222.222, 333.333, 0.0);
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
        let result = Tuple::new(1.11, -2.22, 3.33, 0.0) * 100.1;
        let expected = Tuple::new(111.111, -222.222, 333.333, 0.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn identity() {
        let a = Tuple::new(1.23, 4.56, 7.89, 0.0);
        let b = 1.0;
        assert_eq!(a * b, a);
    }
}

#[cfg(test)]
mod benchs_mul {
    use super::*;

    const A: Tuple = Tuple::new(1.11, -2.22, 3.33, 0.0);

    #[bench]
    fn closure(bench: &mut Bencher) {
        bench.iter(|| (0..N).fold(A, |a, b| a * b as f32));
    }
}

impl Div<f32> for Tuple {
    type Output = Self;

    /// Performs the `*` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// let result = Tuple::new(1.11, -2.22, 3.33, 0.0) / 11.1;
    /// let expected = Tuple::new(0.1, -0.2, 0.3, 0.0);
    /// assert_eq!(expected, result);
    ///
    /// assert!((Tuple::new(1.23, 4.56, 7.89, 10.11) / 0.0).tuple[0..3]
    ///     .iter()
    ///     .all(|&f| f.is_infinite()));
    /// assert!((Tuple::new(0.0, 0.0, 0.0, 0.00) / 0.0)
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
        let result = Tuple::new(1.11, -2.22, 3.33, 0.0) / 10.0;
        let expected = Tuple::new(0.111, -0.222, 0.333, 0.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn identity() {
        let a = Tuple::new(1.23, 4.56, 7.89, 0.0);
        let b = 1.0;
        assert_eq!(a / b, a);
    }

    #[test]
    fn div_by_zero() {
        assert!((Tuple::new(1.23, 4.56, 7.89, 10.11) / 0.0).tuple[0..3]
            .iter()
            .all(|&f| f.is_infinite()));
        assert!((Tuple::new(0.0, 0.0, 0.0, 0.0) / 0.0)
            .into_iter()
            .all(f32::is_nan));
    }
}

#[cfg(test)]
mod benchs_div {
    use super::*;

    const A: Tuple = Tuple::new(1.11, -2.22, 3.33, 0.0);

    #[bench]
    fn closure(bench: &mut Bencher) {
        bench.iter(|| (0..N).fold(A, |a, b| a / b as f32));
    }
}
