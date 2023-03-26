use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use super::{array_base::ArrayBase, coordinates4::Coordinates4, tuple::Tuple, vector::Vector};
use float_cmp::{approx_eq, ApproxEq};

/// A Point in 3D (x,y,z) space is a 4 unit (x,y,z,w) set with the `w` value being 1.0 to allow translations from matrices

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub tuple: [f32; 4],
}
impl Point {
    /// Creates a new Point
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::point::Point;
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    ///
    /// let point = Point::new(1.0, 2.0, 3.0);
    /// assert_eq!(1.0, point.tuple[0]);
    /// assert_eq!(2.0, point.tuple[1]);
    /// assert_eq!(3.0, point.tuple[2]);
    /// assert_eq!(1.0, point.tuple[3]);
    /// assert!(point.is_point() == true);
    /// assert!(point.is_vector() == false);
    /// assert!(point.is_valid());
    /// ```
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            tuple: [x, y, z, 1.0],
        }
    }
}

#[cfg(test)]
mod tests_point {
    use super::*;

    #[test]
    fn new() {
        let point = Point::new(1.0, 2.0, 3.0);
        assert_eq!([1.0, 2.0, 3.0, 1.0], point.tuple);
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn clone() {
        let point = Point::new(1.0, 2.0, 3.0);
        let point_copy = point;
        let point_clone = point_copy.clone();
        assert_eq!([1.0, 2.0, 3.0, 1.0], point_copy.tuple);
        assert_eq!([1.0, 2.0, 3.0, 1.0], point_clone.tuple);
    }

    #[test]
    fn debug_fmt() {
        let point = Point::new(1.0, 2.0, 3.0);
        assert_eq!(
            "Point { tuple: [1.0, 2.0, 3.0, 1.0] }",
            format!("{point:?}")
        );
    }
}

impl Default for Point {
    /// Creates a new Point with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::point::Point;
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    ///
    /// let point = Point::default();
    /// assert_eq!(0.0, point.tuple[0]);
    /// assert_eq!(0.0, point.tuple[1]);
    /// assert_eq!(0.0, point.tuple[2]);
    /// assert_eq!(1.0, point.tuple[3]);
    /// assert!(point.is_point() == true);
    /// assert!(point.is_vector() == false);
    /// assert!(point.is_valid());
    /// ```
    fn default() -> Self {
        Self::new(Default::default(), Default::default(), Default::default())
    }
}

impl From<[f32; 3]> for Point {
    /// Creates a new Point from an array of scaler values
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// # use crate::rusty_ray_tracer::core3d::point::Point;
    /// let point = Point::from([1.0, 2.0, 3.0]);
    /// assert_eq!([1.0, 2.0, 3.0, 1.0], point.tuple);
    /// ```
    fn from(arr: [f32; 3]) -> Self {
        Self::new(arr[0], arr[1], arr[2])
    }
}

impl From<Tuple> for Point {
    /// Creates a new Point from a Tuple
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// # use crate::rusty_ray_tracer::core3d::point::Point;
    /// let point = Point::from(Tuple::from([1.0, 2.0, 3.0, 1.0]));
    /// assert_eq!([1.0, 2.0, 3.0, 1.0], point.tuple);
    /// ```
    ///
    /// ```
    /// # use std::panic;
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// # use crate::rusty_ray_tracer::core3d::point::Point;
    /// let tuple = Tuple::from([1.0, 2.0, 3.0, 4.0]);
    /// assert!(panic::catch_unwind(|| Point::from(tuple)).is_err());
    /// ```
    fn from(tuple: Tuple) -> Self {
        debug_assert!(tuple.is_point());
        Self::new(tuple.x(), tuple.y(), tuple.z())
    }
}

impl From<Point> for Tuple {
    /// Creates a new Tuple from a Point
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// # use crate::rusty_ray_tracer::core3d::point::Point;
    /// let result = Tuple::from(Point::from([1.0, 2.0, 3.0]));
    /// assert_eq!([1.0, 2.0, 3.0, 1.0], result.tuple);
    /// ```
    fn from(point: Point) -> Self {
        debug_assert!(point.is_point());
        Self::from(point.tuple)
    }
}

#[cfg(test)]
mod tests_from {
    use super::*;
    use std::panic;

    #[test]
    fn from_array() {
        let point = Point::from([1.0, 2.0, 3.0]);
        assert_eq!([1.0, 2.0, 3.0, 1.0], point.tuple);
    }

    #[test]
    fn from_tuple() {
        let point = Point::from(Tuple::new(1.0, 2.0, 3.0, 1.0));
        assert_eq!([1.0, 2.0, 3.0, 1.0], point.tuple);

        let tuple = Tuple::from([1.0, 2.0, 3.0, 4.0]);
        panic::catch_unwind(|| Point::from(tuple)).unwrap_err();
    }

    #[test]
    fn into_tuple() {
        let tuple = Tuple::from(Point::new(1.0, 2.0, 3.0));
        assert_eq!([1.0, 2.0, 3.0, 1.0], tuple.tuple);

        let tuple: Tuple = Point::new(1.0, 2.0, 3.0).into();
        assert_eq!([1.0, 2.0, 3.0, 1.0], tuple.tuple);
    }
}

impl ArrayBase for Point {
    type Item = f32;
    // type SizedArray = [f32; 4];

    /// Returns base array consuming
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::point::Point;
    /// let point = Point::new(1.0, 2.0, 3.0);
    /// assert_eq!([1.0, 2.0, 3.0, 1.0], point.get_array());
    /// ```
    fn get_array(self) -> [f32; 4] {
        self.tuple
    }

    /// Returns base array reference
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::point::Point;
    /// let point = Point::new(1.0, 2.0, 3.0);
    /// assert_eq!([1.0, 2.0, 3.0, 1.0], *point.get_array_ref());
    /// ```
    fn get_array_ref(&self) -> &[f32; 4] {
        &self.tuple
    }

    /// Returns a mutable base array reference
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::point::Point;
    /// let mut point = Point::new(1.0, 2.0, 3.0);
    /// assert_eq!([1.0, 2.0, 3.0, 1.0], *point.get_array_mut());
    /// point.get_array_mut()[0] += 10.0;
    /// point.get_array_mut()[1] += 10.0;
    /// point.get_array_mut()[2] += 10.0;
    /// point.get_array_mut()[3] += 10.0;
    /// assert_eq!([11.0, 12.0, 13.0, 11.0], *point.get_array_mut());
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
        let point = Point::new(1.0, 2.0, 3.0);
        assert_eq!([1.0, 2.0, 3.0, 1.0], point.get_array());
        assert_eq!([1.0, 2.0, 3.0, 1.0], *point.get_array_ref());
    }

    #[test]
    fn get_array_mut() {
        let mut point = Point::new(1.0, 2.0, 3.0);
        assert_eq!([1.0, 2.0, 3.0, 1.0], *point.get_array_mut());
        point.get_array_mut()[0] += 10.0;
        point.get_array_mut()[1] += 10.0;
        point.get_array_mut()[2] += 10.0;
        point.get_array_mut()[3] += 10.0;
        assert_eq!([11.0, 12.0, 13.0, 11.0], *point.get_array_mut());
        assert_eq!([11.0, 12.0, 13.0, 11.0], point.get_array());
        assert_eq!([11.0, 12.0, 13.0, 11.0], *point.get_array_ref());
    }
}

impl Coordinates4 for Point {}

#[cfg(test)]
mod tests_coordinates4 {
    use super::*;
    use crate::core3d::coordinates4::Coordinates4;

    #[test]
    fn assign_array() {
        let point: Point = Point::from([3.0, 2.0, 1.0]);
        assert_eq!(3.0, point.x());
        assert_eq!(2.0, point.y());
        assert_eq!(1.0, point.z());
        assert_eq!(1.0, point.w());
        assert!(point.is_point());
        assert!(!point.is_vector());
        assert!(point.is_valid());
    }

    #[test]
    fn create_new() {
        let point = Point::new(1.0, 2.0, 3.0);
        assert_eq!(1.0, point.x());
        assert_eq!(2.0, point.y());
        assert_eq!(3.0, point.z());
        assert_eq!(1.0, point.w());
        assert!(point.is_point());
        assert!(!point.is_vector());
        assert!(point.is_valid());
    }
}

impl Display for Point {
    /// Returns a string representation of the Point object as [{x}, {y}, {z}, {w}]
    ///
    /// ```
    /// use rusty_ray_tracer::core3d::point::Point;
    ///
    /// let point = Point::new(1.0, 2.0, 3.0);
    /// assert_eq!("[1, 2, 3, 1]", format!("{}", point));
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
    fn eq() {
        let point = Point::new(1.0, 2.0, 3.0);
        assert_eq!("[1, 2, 3, 1]", format!("{point}"));
    }
}

impl PartialEq for Point {
    /// Performs the `=` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::point::Point;
    /// let a = Point::new(1.23, 4.56, 0.0);
    /// let b = Point::new(1.23, 4.56, 0.0);
    /// assert_eq!(a, b);
    /// ```
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::point::Point;
    /// let a = Point::new(1.23, 4.56, 1.000000);
    /// let b = Point::new(1.23, 4.56, 1.000001);
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
            Point::new(1.23, 4.56, 0.000_000_000_000_00),
            Point::new(1.23, 4.56, 0.000_000_000_000_01)
        );
        assert_eq!(
            Point::new(1.23, 4.56, 0.000_000_0),
            Point::new(1.23, 4.56, 0.000_000_1)
        );
        assert_eq!(
            Point::new(1.23, 4.56, 1.000_000_0),
            Point::new(1.23, 4.56, 1.000_000_1)
        );
        assert_eq!(
            Point::new(1.23, 4.56, 1_000_000.0),
            Point::new(1.23, 4.56, 1_000_000.1)
        );
    }

    #[test]
    fn ne() {
        assert_ne!(
            Point::new(1.23, 4.56, 0.000_010),
            Point::new(1.23, 4.56, 0.000_011)
        );
        assert_ne!(
            Point::new(1.23, 4.56, 1.000_000),
            Point::new(1.23, 4.56, 1.000_001)
        );
        assert_ne!(
            Point::new(1.23, 4.56, 100_000.0),
            Point::new(1.23, 4.56, 100_000.1)
        );
    }
}

impl ApproxEq for Point {
    type Margin = <f32 as ApproxEq>::Margin;

    /// Performs the `~=` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::point::Point;
    /// # use float_cmp::ApproxEq;
    /// let a = Point::new(1.23, 4.56, 0.000000000000);
    /// let b = Point::new(1.23, 4.56, 0.000000000001);
    /// assert!(a.approx_eq(b, <Point as ApproxEq>::Margin::default()));
    /// ```
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::point::Point;
    /// # use float_cmp::ApproxEq;
    /// let a = Point::new(1.23, 4.56, 1.0000000);
    /// let b = Point::new(1.23, 4.56, 1.0000001);
    /// assert!(a.approx_eq(b, <Point as ApproxEq>::Margin::default().ulps(2)));
    /// ```
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::point::Point;
    /// # use float_cmp::ApproxEq;
    /// let a = Point::new(1.23, 4.56, 0.0);
    /// let b = Point::new(1.23, 4.56, 1.0);
    /// assert!(a.approx_eq(b, <Point as ApproxEq>::Margin::default().epsilon(1.0)));
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
            Point,
            Point::new(1.23, 4.56, 0.000_000_000_000),
            Point::new(1.23, 4.56, 0.000_000_000_001)
        );
        assert_approx_eq!(
            Point,
            Point::new(1.23, 4.56, 1.000_000_0),
            Point::new(1.23, 4.56, 1.000_000_1),
            ulps = 2
        );
        assert_approx_eq!(
            Point,
            Point::new(1.23, 4.56, 0.0),
            Point::new(1.23, 4.56, 1.0),
            epsilon = 1.0
        );
    }

    #[test]
    fn ne() {
        {
            let a = Point::new(1.23, 4.56, 1.000_000);
            let b = Point::new(1.23, 4.56, 1.000_001);
            assert!(a.approx_ne(b, <Point as ApproxEq>::Margin::default()));
        }
        {
            let a = Point::new(1.23, 4.56, 1.000_000);
            let b = Point::new(1.23, 4.56, 1.000_001);
            assert!(a.approx_ne(b, <Point as ApproxEq>::Margin::default().ulps(2)));
        }
        {
            let a = Point::new(1.23, 4.56, 0.000_000_0);
            let b = Point::new(1.23, 4.56, 1.000_000_1);
            assert!(a.approx_ne(b, <Point as ApproxEq>::Margin::default().epsilon(1.0)));
        }
    }
}

impl Sub for Point {
    /// The resulting type after applying the `+` operator.
    type Output = Vector;

    /// Performs the `+` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::point::Point;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let a = Point::new(1.23, 4.56, 7.89);
    /// let b = Point::new(1.11, 2.22, 3.33);
    /// let expected = Vector::new(0.12, 2.34, 4.56);
    /// assert_eq!(expected, a - b);
    /// ```
    #[must_use]
    fn sub(self, rhs: Self) -> Self::Output {
        Vector::from(Tuple::zip_for_each_collect(
            Tuple::from(self),
            rhs,
            |a, b| a - b,
        ))
    }
}

#[cfg(test)]
mod tests_sub {
    use super::*;

    #[test]
    fn not_closure() {
        let a = Point::new(1.23, 4.56, 7.89);
        let b = Point::new(1.11, 2.22, 3.33);
        let expected = Vector::new(0.12, 2.34, 4.56);
        assert_eq!(expected, a - b);
    }

    #[test]
    fn not_identity() {
        let a = Point::new(1.23, 4.56, 7.89);
        let b = Point::default();
        let ab = Vector::new(1.23, 4.56, 7.89);
        assert_eq!(ab, a - b);
        assert_ne!(ab, b - a);
        let ba = Vector::new(-1.23, -4.56, -7.89);
        assert_eq!(ba, b - a);
    }

    #[test]
    fn not_commutative() {
        let a = Point::new(1.23, 4.56, 7.89);
        let b = Point::new(1.11, 2.22, 3.33);
        assert_ne!(a - b, b - a);

        let ab = Vector::new(0.12, 2.34, 4.56);
        let ba = Vector::new(-0.12, -2.34, -4.56);
        assert_eq!(ab, a - b);
        assert_eq!(ba, b - a);
    }
}

impl Add<Vector> for Point {
    /// The resulting type after applying the `+` operator.
    type Output = Self;

    /// Performs the `+` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::point::Point;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let a = Point::new(1.23, 4.56, 7.89);
    /// let b = Vector::new(1.11, 2.22, 3.33);
    /// let expected = Point::new(2.34, 6.78, 11.22);
    /// assert_eq!(expected, a + b);
    /// ```
    #[must_use]
    fn add(self, rhs: Vector) -> Self::Output {
        Self::zip_for_each_collect(self, rhs, |a, b| a + b)
    }
}

#[cfg(test)]
mod tests_add_vector {
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn not_closure() {
        let a = Point::new(1.23, 4.56, 7.89);
        let b = Vector::new(1.11, 2.22, 3.33);
        let expected = Point::new(2.34, 6.78, 11.22);
        assert_eq!(expected, a + b);
    }

    #[test]
    fn not_identity() {
        let a = Point::new(1.23, 4.56, 7.89);
        let b = Vector::default();
        assert_eq!(a + b, a);
    }

    #[test]
    fn not_associative() {
        let a = Point::new(1.23, 4.56, 7.89);
        let b = Point::new(1.11, 2.22, 3.33);
        let c = Point::new(5.55, 6.66, 7.77);
        assert_ne!(a + (b - c), c + (a - b));
        assert_ne!(b + (c - a), a + (b - c));

        let a_bc = Point::new(-3.21, 0.12, 3.45);
        let c_ab = Point::new(5.67, 9.00, 12.33);
        let b_ca = Point::new(5.43, 4.32, 3.21);
        assert_approx_eq!(Point, a_bc, a + (b - c), ulps = 49);
        assert_approx_eq!(Point, c_ab, c + (a - b));
        assert_approx_eq!(Point, b_ca, b + (c - a));

        // let a_bc = Point::new(-3.21, 0.12000036, 3.4499998);
        // let c_ab = Point::new( 5.67, 9.00, 12.33);
        // let b_ca = Point::new(5.4300003, 4.3199997, 3.21);
        // assert_approx_eq!(Point, a_bc, a + (b - c), ulps = 2);
        // assert_approx_eq!(Point, c_ab, c + (a - b), ulps = 2);
        // assert_approx_eq!(Point, b_ca, b + (c - a), ulps = 2);
    }
}

impl Sub<Vector> for Point {
    /// The resulting type after applying the `+` operator.
    type Output = Self;

    /// Performs the `-` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::point::Point;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let a = Point::new(1.23, 4.56, 7.89);
    /// let b = Vector::new(1.11, 2.22, 3.33);
    /// let expected = Point::new(0.12, 2.34, 4.56);
    /// assert_eq!(expected, a - b);
    /// ```
    #[must_use]
    fn sub(self, rhs: Vector) -> Self::Output {
        Self::zip_for_each_collect(self, rhs, |a, b| a - b)
    }
}

#[cfg(test)]
mod tests_sub_vector {
    use super::*;

    #[test]
    fn not_closure() {
        let a = Point::new(1.23, 4.56, 7.89);
        let b = Vector::new(1.11, 2.22, 3.33);
        let expected = Point::new(0.12, 2.34, 4.56);
        assert_eq!(expected, a - b);
    }

    #[test]
    fn not_identity() {
        let a = Point::new(1.23, 4.56, 7.89);
        let b = Vector::default();
        assert_eq!(a - b, a);
    }

    #[test]
    fn not_associative() {
        let a = Point::new(1.23, 4.56, 7.89);
        let b = Point::new(1.11, 2.22, 3.33);
        let c = Point::new(5.55, 6.66, 7.77);
        assert_ne!(a - (b - c), c - (a - b));
        assert_ne!(b - (c - a), a - (b - c));

        let a_bc = Point::new(5.67, 9.0, 12.33);
        let c_ab = Point::new(5.43, 4.32, 3.21);
        let b_ca = Point::new(-3.21, 0.120_000_124, 3.45);
        assert_eq!(a_bc, a - (b - c));
        assert_eq!(c_ab, c - (a - b));
        assert_eq!(b_ca, b - (c - a));
    }
}
