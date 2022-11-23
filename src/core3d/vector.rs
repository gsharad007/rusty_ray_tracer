use std::ops::{Add, Div, Mul, Neg, Sub};

use super::{array_base::ArrayBase, coordinates4::Coordinates4};
use crate::core3d::tuple::Tuple;
use float_cmp::{approx_eq, ApproxEq};

/// A Vector in 3D (x,y,z) space is a 4 unit (x,y,z,w) set with the `w` value being 0.0 to ignore translations from matrices

#[derive(Copy, Clone, Default, Debug)]
pub struct Vector {
    pub coords: [f32; 4],
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
    /// let vector = Vector::new(1.0, 2.0, 3.0);
    /// assert_eq!(1.0, vector.x());
    /// assert_eq!(2.0, vector.y());
    /// assert_eq!(3.0, vector.z());
    /// assert_eq!(0.0, vector.w());
    /// assert!(vector.is_vector() == true);
    /// assert!(vector.is_point() == false);
    /// assert!(vector.is_valid());
    /// ```
    #[must_use]
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector {
            coords: [x, y, z, 0.0],
        }
    }
}

#[cfg(test)]
mod tests_vector {
    use super::*;

    #[test]
    fn new() {
        let vector = Vector::new(1.0, 2.0, 3.0);
        assert_eq!([1.0, 2.0, 3.0, 0.0], vector.coords);
    }
}

impl From<[f32; 3]> for Vector {
    /// Creates a new vector from an array of scaler values
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let vector = Vector::from([1.0, 2.0, 3.0]);
    /// assert_eq!([1.0, 2.0, 3.0, 0.0], vector.coords);
    /// ```
    fn from(arr: [f32; 3]) -> Self {
        Vector::new(arr[0], arr[1], arr[2])
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
    /// let vector = Vector::from(Tuple::from([1.0, 2.0, 3.0, 0.0]));
    /// assert_eq!([1.0, 2.0, 3.0, 0.0], vector.coords);
    /// ```
    ///
    /// ```
    /// # use std::panic;
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let tuple = Tuple::from([1.0, 2.0, 3.0, 4.0]);
    /// assert!(panic::catch_unwind(|| Vector::from(tuple)).is_err());
    /// ```
    fn from(tuple: Tuple) -> Self {
        debug_assert!(tuple.is_vector());
        Vector::new(tuple.x(), tuple.y(), tuple.z())
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
    /// let vector = Vector::from(Tuple::from([1.0, 2.0, 3.0, 0.0]));
    /// assert_eq!([1.0, 2.0, 3.0, 0.0], vector.coords);
    /// ```
    ///
    /// ```
    /// # use std::panic;
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let tuple = Tuple::from([1.0, 2.0, 3.0, 4.0]);
    /// assert!(panic::catch_unwind(|| Vector::from(tuple)).is_err());
    /// ```
    fn from(vector: Vector) -> Self {
        debug_assert!(vector.is_vector());
        Tuple::from(vector.coords)
    }
}

#[cfg(test)]
mod tests_from {
    use super::*;
    use std::panic;

    #[test]
    fn from_array() {
        let vector = Vector::from([1.0, 2.0, 3.0]);
        assert_eq!([1.0, 2.0, 3.0, 0.0], vector.coords);
    }

    #[test]
    fn from_tuple() {
        let vector = Vector::from(Tuple::new(1.0, 2.0, 3.0, 0.0));
        assert_eq!([1.0, 2.0, 3.0, 0.0], vector.coords);

        let tuple = Tuple::from([1.0, 2.0, 3.0, 4.0]);
        assert!(panic::catch_unwind(|| Vector::from(tuple)).is_err());
    }

    #[test]
    fn into_tuple() {
        let tuple = Tuple::from(Vector::new(1.0, 2.0, 3.0));
        assert_eq!([1.0, 2.0, 3.0, 0.0], tuple.coords);

        let tuple: Tuple = Vector::new(1.0, 2.0, 3.0).into();
        assert_eq!([1.0, 2.0, 3.0, 0.0], tuple.coords);
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
    /// let vector = Vector::new(1.0, 2.0, 3.0);
    /// assert_eq!([1.0, 2.0, 3.0, 0.0], vector.get_array());
    /// ```
    fn get_array(self) -> [f32; 4] {
        self.coords
    }

    /// Returns base array reference
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let vector = Vector::new(1.0, 2.0, 3.0);
    /// assert_eq!([1.0, 2.0, 3.0, 0.0], *vector.get_array_ref());
    /// ```
    fn get_array_ref(&self) -> &[f32; 4] {
        &self.coords
    }

    /// Returns a mutable base array reference
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let mut vector = Vector::new(1.0, 2.0, 3.0);
    /// assert_eq!([1.0, 2.0, 3.0, 0.0], *vector.get_array_mut());
    /// vector.get_array_mut()[0] += 10.0;
    /// vector.get_array_mut()[1] += 10.0;
    /// vector.get_array_mut()[2] += 10.0;
    /// vector.get_array_mut()[3] += 10.0;
    /// assert_eq!([11.0, 12.0, 13.0, 10.0], *vector.get_array_mut());
    /// ```
    fn get_array_mut(&mut self) -> &mut [f32; 4] {
        &mut self.coords
    }
}

#[cfg(test)]
mod tests_array_base {
    use super::*;

    #[test]
    fn get_array() {
        let vector = Vector::new(1.0, 2.0, 3.0);
        assert_eq!([1.0, 2.0, 3.0, 0.0], *vector.get_array_ref());
        assert_eq!([1.0, 2.0, 3.0, 0.0], vector.get_array());
        assert_eq!([1.0, 2.0, 3.0, 0.0], *vector.get_array_ref());
    }

    #[test]
    fn get_array_mut() {
        let mut vector = Vector::new(1.0, 2.0, 3.0);
        assert_eq!([1.0, 2.0, 3.0, 0.0], *vector.get_array_mut());
        vector.get_array_mut()[0] += 10.0;
        vector.get_array_mut()[1] += 10.0;
        vector.get_array_mut()[2] += 10.0;
        vector.get_array_mut()[3] += 10.0;
        assert_eq!([11.0, 12.0, 13.0, 10.0], *vector.get_array_mut());
        assert_eq!([11.0, 12.0, 13.0, 10.0], vector.get_array());
        assert_eq!([11.0, 12.0, 13.0, 10.0], *vector.get_array_ref());
    }
}

impl Coordinates4 for Vector {}

#[cfg(test)]
mod tests_coordinates4 {
    use super::*;
    use crate::core3d::coordinates4::Coordinates4;

    #[test]
    fn assign_array() {
        let vector: Vector = Vector::from([3.0, 2.0, 1.0]);
        assert_eq!(3.0, vector.x());
        assert_eq!(2.0, vector.y());
        assert_eq!(1.0, vector.z());
        assert_eq!(0.0, vector.w());
        assert!(vector.is_vector());
        assert!(!vector.is_point());
        assert!(vector.is_valid());
    }

    #[test]
    fn create_new() {
        let vector = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(1.0, vector.x());
        assert_eq!(2.0, vector.y());
        assert_eq!(3.0, vector.z());
        assert_eq!(0.0, vector.w());
        assert!(vector.is_vector());
        assert!(!vector.is_point());
        assert!(vector.is_valid());
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
            Vector::new(1.23, 4.56, 0.00000000000000),
            Vector::new(1.23, 4.56, 0.00000000000001)
        );
        assert_eq!(
            Vector::new(1.23, 4.56, 0.0000000),
            Vector::new(1.23, 4.56, 0.0000001)
        );
        assert_eq!(
            Vector::new(1.23, 4.56, 1.0000000),
            Vector::new(1.23, 4.56, 1.0000001)
        );
        assert_eq!(
            Vector::new(1.23, 4.56, 1000000.0),
            Vector::new(1.23, 4.56, 1000000.1)
        );
    }

    #[test]
    fn ne() {
        assert_ne!(
            Vector::new(1.23, 4.56, 0.000010),
            Vector::new(1.23, 4.56, 0.000011)
        );
        assert_ne!(
            Vector::new(1.23, 4.56, 1.000000),
            Vector::new(1.23, 4.56, 1.000001)
        );
        assert_ne!(
            Vector::new(1.23, 4.56, 100000.0),
            Vector::new(1.23, 4.56, 100000.1)
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
            Vector::new(1.23, 4.56, 0.000000000000),
            Vector::new(1.23, 4.56, 0.000000000001)
        );
        assert_approx_eq!(
            Vector,
            Vector::new(1.23, 4.56, 1.0000000),
            Vector::new(1.23, 4.56, 1.0000001),
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
            let a = Vector::new(1.23, 4.56, 1.000000);
            let b = Vector::new(1.23, 4.56, 1.000001);
            assert!(a.approx_ne(b, <Vector as ApproxEq>::Margin::default()));
        }
        {
            let a = Vector::new(1.23, 4.56, 1.000000);
            let b = Vector::new(1.23, 4.56, 1.000001);
            assert!(a.approx_ne(b, <Vector as ApproxEq>::Margin::default().ulps(2)));
        }
        {
            let a = Vector::new(1.23, 4.56, 0.0000000);
            let b = Vector::new(1.23, 4.56, 1.0000001);
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

impl Sub for Vector {
    /// The resulting type after applying the `-` operator.
    type Output = Vector;

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
        let ca_b = Vector::new(3.21, -0.120000124, -3.45);
        assert_eq!(a_bc, a - (b - c));
        assert_eq!(ab_c, (a - b) - c);
        assert_eq!(c_ab, c - (a - b));
        assert_eq!(ca_b, (c - a) - b);
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
    /// assert!((Vector::new(1.23, 4.56, 7.89) / 0.0).coords[0..3]
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
        assert!((Vector::new(1.23, 4.56, 7.89) / 0.0).coords[0..3]
            .iter()
            .all(|&f| f.is_infinite()));
        assert!((Vector::new(0.0, 0.0, 0.0) / 0.0)
            .into_iter()
            .all(|f| f.is_nan()));
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
            Vector::new(0.57735029, 0.57735029, 0.57735028).magnitude()
        );

        assert_eq!(
            3.7416573867739413855837487323165,
            Vector::new(1.0, 2.0, 3.0).magnitude()
        );
        assert_eq!(
            3.7416573867739413855837487323165,
            Vector::new(-1.0, -2.0, -3.0).magnitude()
        );
        assert_eq!(9.195575, Vector::new(1.23, 4.56, 7.89).magnitude());
        assert_eq!(4.1532397, Vector::new(1.11, 2.22, 3.33).magnitude());
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
            Vector::new(0.57735029, 0.57735029, 0.57735028),
            Vector::new(0.57735029, 0.57735029, 0.57735028).normalize()
        );

        assert_eq!(
            Vector::new(0.26726124, 0.5345225, 0.8017837),
            Vector::new(1.0, 2.0, 3.0).normalize()
        );
        assert_eq!(
            Vector::new(-0.26726124, -0.5345225, -0.8017837),
            Vector::new(-1.0, -2.0, -3.0).normalize()
        );
    }

    #[test]
    fn tests_normalize_zero_vector() {
        assert!(Vector::new(0.0, 0.0, 0.0)
            .normalize()
            .into_iter()
            .all(|f| f.is_nan()));
    }
}

pub trait DotProduct: ArrayBase<Item = f32> {
    /// Calculate Dot Product on two Vectors
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// # use crate::rusty_ray_tracer::core3d::vector::*;
    /// assert_eq!(0.0, Vector::new(0.0, 0.0, 0.0).dot(Vector::new(0.0, 0.0, 0.0)));
    /// assert_eq!(1.0, Vector::new(1.0, 0.0, 0.0).dot(Vector::new(1.0, 0.0, 0.0)));
    /// assert_eq!(1.0, Vector::new(0.0, 1.0, 0.0).dot(Vector::new(0.0, 1.0, 0.0)));
    /// assert_eq!(1.0, Vector::new(0.0, 0.0, 1.0).dot(Vector::new(0.0, 0.0, 1.0)));
    /// assert_eq!(1.0, Vector::new(0.57735029, 0.57735028, 0.57735028).dot(Vector::new(0.57735029, 0.57735028, 0.57735028)));
    /// ```
    #[must_use]
    fn dot(self, other: Self) -> f32 {
        Self::into_iter(self)
            .zip(other.into_iter())
            .fold(0.0, |acc, v| acc + (v.0 * v.1))
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
            Vector::new(0.57735029, 0.57735028, 0.57735028)
                .dot(Vector::new(0.57735029, 0.57735028, 0.57735028))
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
        Vector::new(
            (self.get_array()[1] * other.get_array()[2])
                - (self.get_array()[2] * other.get_array()[1]),
            (self.get_array()[2] * other.get_array()[0])
                - (self.get_array()[0] * other.get_array()[2]),
            (self.get_array()[0] * other.get_array()[1])
                - (self.get_array()[1] * other.get_array()[0]),
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
            Vector::new(0.57735029, 0.57735028, 0.57735028)
                .cross(Vector::new(0.57735029, 0.57735028, 0.57735028))
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
}
