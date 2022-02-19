use std::ops::Sub;

use super::{array_base::ArrayBase, coordinates4::Coordinates4, vector::Vector};
use crate::core3d::tuple::Tuple;

/// A Point in 3D (x,y,z) space is a 4 unit (x,y,z,w) set with the `w` value being 1.0 to allow translations from matrices

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub coords: [f32; 4],
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
    /// assert_eq!(1.0, point.x());
    /// assert_eq!(2.0, point.y());
    /// assert_eq!(3.0, point.z());
    /// assert_eq!(1.0, point.w());
    /// assert!(point.is_point() == true);
    /// assert!(point.is_vector() == false);
    /// assert!(point.is_valid());
    /// ```
    #[must_use]
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point {
            coords: [x, y, z, 1.0],
        }
    }
}

#[cfg(test)]
mod tests_point {
    use super::*;

    #[test]
    fn new() {
        let point = Point::new(1.0, 2.0, 3.0);
        assert_eq!([1.0, 2.0, 3.0, 1.0], point.coords);
    }
}

impl Default for Point {
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
    /// assert_eq!([1.0, 2.0, 3.0, 1.0], point.coords);
    /// ```
    fn from(arr: [f32; 3]) -> Self {
        Point::new(arr[0], arr[1], arr[2])
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
    /// assert_eq!([1.0, 2.0, 3.0, 1.0], point.coords);
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
        Point::new(tuple.x(), tuple.y(), tuple.z())
    }
}

impl From<Point> for Tuple {
    /// Creates a new Tuple from a Puple
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// # use crate::rusty_ray_tracer::core3d::point::Point;
    /// let result = Tuple::from(Point::from([1.0, 2.0, 3.0]));
    /// assert_eq!([1.0, 2.0, 3.0, 1.0], result.coords);
    /// ```
    fn from(point: Point) -> Self {
        Tuple::from(point.coords)
    }
}

#[cfg(test)]
mod tests_from {
    use super::*;
    use std::panic;

    #[test]
    fn from_array() {
        let point = Point::from([1.0, 2.0, 3.0]);
        assert_eq!([1.0, 2.0, 3.0, 1.0], point.coords);
    }

    #[test]
    fn from_tuple() {
        let point = Point::from(Tuple::new(1.0, 2.0, 3.0, 1.0));
        assert_eq!([1.0, 2.0, 3.0, 1.0], point.coords);

        let tuple = Tuple::from([1.0, 2.0, 3.0, 4.0]);
        assert!(panic::catch_unwind(|| Point::from(tuple)).is_err());
    }

    #[test]
    fn into_tuple() {
        let tuple = Tuple::from(Point::new(1.0, 2.0, 3.0));
        assert_eq!([1.0, 2.0, 3.0, 1.0], tuple.coords);

        let tuple: Tuple = Point::new(1.0, 2.0, 3.0).into();
        assert_eq!([1.0, 2.0, 3.0, 1.0], tuple.coords);
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
        self.coords
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
        &self.coords
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
        &mut self.coords
    }
}

#[cfg(test)]
mod tests_array_base {
    use super::*;

    #[test]
    fn get_array() {
        let point = Point::new(1.0, 2.0, 3.0);
        assert_eq!([1.0, 2.0, 3.0, 1.0], *point.get_array_ref());
        assert_eq!([1.0, 2.0, 3.0, 1.0], point.clone().get_array());
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
        assert_eq!([11.0, 12.0, 13.0, 11.0], point.clone().get_array());
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
        assert!(point.is_point() == true);
        assert!(point.is_vector() == false);
        assert!(point.is_valid());
    }

    #[test]
    fn create_new() {
        let point = Point::new(1.0, 2.0, 3.0);
        assert_eq!(1.0, point.x());
        assert_eq!(2.0, point.y());
        assert_eq!(3.0, point.z());
        assert_eq!(1.0, point.w());
        assert!(point.is_point() == true);
        assert!(point.is_vector() == false);
        assert!(point.is_valid());
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
        Vector::from(Tuple::from(Self::zip_for_each_collect(
            self,
            rhs,
            |a, b| a - b,
        )))
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
        let expected = Vector::new(1.23, 4.56, 7.89);
        assert_eq!(a - b, expected);
        assert_ne!(b - a, expected);
    }

    #[test]
    fn not_commutative() {
        let a = Point::new(1.23, 4.56, 7.89);
        let b = Point::new(1.11, 2.22, 3.33);
        assert_ne!(a - b, b - a);
    }

    // #[test]
    // fn not_associative() {
    //     let a = Point::new(1.23, 4.56, 7.89);
    //     let b = Point::new(1.11, 2.22, 3.33);
    //     let c = Point::new(5.55, 6.66, 7.77);
    //     assert_ne!(a - (b - c), (a - b) - c);
    //     assert_ne!(c - (a - b), (c - a) - b);
    // }
}
