use core::iter::Zip;
use core::ops::Add;
use core::slice::{Iter, IterMut};
use float_cmp::approx_eq;
use float_cmp::ApproxEq;

// /// This Tuple in 3D (x,y,z) space is a 4 unit set (x,y,z,w) to allow passing in either point or vector
// #[derive(Debug, Default)]
// pub struct Tuple([f32; 4]);

// impl Tuple {
//     #[must_use]
//     pub fn from_array(t: [f32; 4]) -> Tuple {
//         Tuple(t)
//     }
// }

/// # 3D Coordinates
/// ## Left Handed Coordinates (default setup here)
/// With the y axis pointing up, and the x axis pointing to the right, the z axis can be defined to point away from
/// you.
/// ## Right Handed Coordinates
/// With the y axis pointing up, and the x axis pointing to the right, the z axis can be defined to point toward you.
pub trait Tuple: Sized {
    /// Gets the specified dimension from the tuple
    #[must_use]
    fn get_at(&self, dim: usize) -> f32;

    /// Gets the first dimension (right) from the coordinate system
    #[must_use]
    fn x(&self) -> f32 {
        self.get_at(0)
    }

    /// Gets the second dimension (up) from the coordinate system
    #[must_use]
    fn y(&self) -> f32 {
        self.get_at(1)
    }

    /// Gets the third dimension (away) from the coordinate system
    #[must_use]
    fn z(&self) -> f32 {
        self.get_at(2)
    }

    /// Gets the fourth element `w` (usually 1.0 for absolute points, 0.0 for vectors) which helps with matrix
    /// multiplication
    #[must_use]
    fn w(&self) -> f32 {
        self.get_at(3)
    }

    /// Checks if the tuple is a point.
    ///
    /// Returns `true` if is a point. `false` otherwise
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::tuples::Tuple;
    /// # use crate::rusty_ray_tracer::core3d::tuples::Tuplef32;
    /// let tuple = Tuplef32::new(1.23, 4.56, 7.89, 1.0);
    /// assert!(tuple.is_point() == true);
    /// ```
    #[must_use]
    fn is_point(&self) -> bool {
        self.w() == 1.0
    }

    /// Checks if the tuple is a vector.
    ///
    /// Returns `true` if is a vector. `false` otherwise
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::tuples::Tuple;
    /// # use crate::rusty_ray_tracer::core3d::tuples::Tuplef32;
    /// let tuple = Tuplef32::new(1.23, 4.56, 7.89, 0.0);
    /// assert!(tuple.is_vector() == true);
    /// ```
    #[must_use]
    fn is_vector(&self) -> bool {
        self.w() == 0.0
    }

    /// Debug asserts to make sure the tuple is valid
    fn check_tuple(&self) {
        debug_assert!(self.is_point() || self.is_vector());
    }

    /// .
    ///
    /// # Examples
    ///
    /// ```
    /// // Example template not implemented for trait functions
    /// ```
    fn iter(&self) -> Iter<'_, f32>;

    fn iter_mut(&mut self) -> IterMut<'_, f32>;

    // Combines both Tuples into one using a closure
    #[must_use]
    fn zip_for_each_collect(a: Self, b: Self, f: impl Fn(f32, f32) -> f32) -> Self {
        let mut result = a;
        result
            .iter_mut()
            .zip(b.iter())
            .for_each(|(i, j)| *i = f(*i, *j));
        result.check_tuple();
        result
    }

    // Combines both Tuples into one using a closure
    #[must_use]
    fn zip<'a, 'b>(a: &'a Self, b: &'b Self) -> Zip<Iter<'a, f32>, Iter<'b, f32>> {
        a.iter().zip(b.iter())
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Tuplef32 {
    coords: [f32; 4],
}
impl Tuplef32 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Tuplef32 {
        Tuplef32 {
            coords: [x, y, z, w],
        }
    }
}
impl Tuple for Tuplef32 {
    fn get_at(&self, index: usize) -> f32 {
        self.coords[index]
    }
    fn iter(&self) -> Iter<'_, f32> {
        self.coords.iter()
    }
    fn iter_mut(&mut self) -> IterMut<'_, f32> {
        self.coords.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_array() {
        let tuple = Tuplef32::new(0.0, 1.0, 2.0, 3.0);
        assert_eq!(0.0, tuple.get_at(0));
        assert_eq!(1.0, tuple.get_at(1));
        assert_eq!(2.0, tuple.get_at(2));
        assert_eq!(3.0, tuple.get_at(3));
    }

    #[test]
    #[should_panic]
    fn invalid_get_at() {
        let tuple = Tuplef32::new(0.0, 1.0, 2.0, 3.0);
        assert_eq!(0.0, tuple.get_at(4));
    }

    #[test]
    fn assign_default() {
        let tuple: Tuplef32 = Default::default();
        assert_eq!(0.0, tuple.get_at(0));
        assert_eq!(0.0, tuple.get_at(1));
        assert_eq!(0.0, tuple.get_at(2));
        assert_eq!(0.0, tuple.get_at(3));
    }

    #[test]
    fn check_xyzw() {
        let tuple = Tuplef32::new(1.23, 4.56, 7.89, 10.11);
        assert_eq!(1.23, tuple.x());
        assert_eq!(4.56, tuple.y());
        assert_eq!(7.89, tuple.z());
        assert_eq!(10.11, tuple.w());
    }

    #[test]
    fn is_point() {
        let tuple = Tuplef32::new(1.23, 4.56, 7.89, 1.0);
        tuple.check_tuple();
        assert!(tuple.is_point() == true);
        assert!(tuple.is_vector() == false);
    }

    #[test]
    fn is_vector() {
        let tuple = Tuplef32::new(1.23, 4.56, 7.89, 0.0);
        tuple.check_tuple();
        assert!(tuple.is_vector() == true);
        assert!(tuple.is_point() == false);
    }

    #[test]
    #[should_panic]
    fn invalid_point_or_vector() {
        let tuple = Tuplef32::new(1.23, 4.56, 7.89, 1.01);
        assert!(tuple.is_point() == false);
        assert!(tuple.is_vector() == false);
        tuple.check_tuple();
    }
}

impl PartialEq for Tuplef32 {
    /// Performs the `=` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use rusty_ray_tracer::core3d::tuples::Tuple;
    /// # use rusty_ray_tracer::core3d::tuples::Tuplef32;
    /// let a = Tuplef32::new(1.23, 4.56, 7.89, 0.0);
    /// let b = Tuplef32::new(1.23, 4.56, 7.89, 0.0);
    /// assert_eq!(a, b);
    /// ```
    ///
    /// ```
    /// # use rusty_ray_tracer::core3d::tuples::Tuple;
    /// # use rusty_ray_tracer::core3d::tuples::Tuplef32;
    /// let a = Tuplef32::new(1.23, 4.56, 7.89, 0.0);
    /// let b = Tuplef32::new(1.23, 4.56, 7.89, 1.0);
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
        {
            let a = Tuplef32::new(1.23, 4.56, 7.89, 0.000000000000);
            let b = Tuplef32::new(1.23, 4.56, 7.89, 0.000000000001);
            assert_eq!(a, b);
        }
        {
            let a = Tuplef32::new(1.23, 4.56, 7.89, 1.0000000);
            let b = Tuplef32::new(1.23, 4.56, 7.89, 1.0000001);
            assert_eq!(a, b);
        }
        {
            let a = Tuplef32::new(1.23, 4.56, 7.89, 1000000.0);
            let b = Tuplef32::new(1.23, 4.56, 7.89, 1000000.1);
            assert_eq!(a, b);
        }
    }

    #[test]
    fn ne() {
        {
            let a = Tuplef32::new(1.23, 4.56, 7.89, 0.000000);
            let b = Tuplef32::new(1.23, 4.56, 7.89, 1.000001);
            assert_ne!(a, b);
        }
        {
            let a = Tuplef32::new(1.23, 4.56, 7.89, 0.000000);
            let b = Tuplef32::new(1.23, 4.56, 7.89, 1.000001);
            assert_ne!(a, b);
        }
        {
            let a = Tuplef32::new(1.23, 4.56, 7.89, 0.000000);
            let b = Tuplef32::new(1.23, 4.56, 7.89, 1.000001);
            assert_ne!(a, b);
        }
    }
}

impl ApproxEq for Tuplef32 {
    type Margin = float_cmp::F32Margin;

    /// Performs the `~=` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use rusty_ray_tracer::core3d::tuples::Tuple;
    /// # use rusty_ray_tracer::core3d::tuples::Tuplef32;
    /// # use float_cmp::assert_approx_eq;
    /// let a = Tuplef32::new(1.23, 4.56, 7.89, 0.000000000000);
    /// let b = Tuplef32::new(1.23, 4.56, 7.89, 0.000000000001);
    /// assert_approx_eq!(Tuplef32, a, b);
    /// ```
    ///
    /// ```
    /// # use rusty_ray_tracer::core3d::tuples::Tuple;
    /// # use rusty_ray_tracer::core3d::tuples::Tuplef32;
    /// # use float_cmp::assert_approx_eq;
    /// let a = Tuplef32::new(1.23, 4.56, 7.89, 1.0000000);
    /// let b = Tuplef32::new(1.23, 4.56, 7.89, 1.0000001);
    /// assert_approx_eq!(Tuplef32, a, b, ulps = 2);
    /// ```
    ///
    /// ```
    /// # use rusty_ray_tracer::core3d::tuples::Tuple;
    /// # use rusty_ray_tracer::core3d::tuples::Tuplef32;
    /// # use float_cmp::assert_approx_eq;
    /// let a = Tuplef32::new(1.23, 4.56, 7.89, 0.0);
    /// let b = Tuplef32::new(1.23, 4.56, 7.89, 1.0);
    /// assert_approx_eq!(Tuplef32, a, b, epsilon = 1.0);
    /// ```
    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        Self::zip(&self, &other).all(|(a, b)| a.approx_eq(*b, margin))
    }
}

#[cfg(test)]
mod tests_approx_eq {
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn eq() {
        {
            let a = Tuplef32::new(1.23, 4.56, 7.89, 0.000000000000);
            let b = Tuplef32::new(1.23, 4.56, 7.89, 0.000000000001);
            assert_approx_eq!(Tuplef32, a, b);
        }
        {
            let a = Tuplef32::new(1.23, 4.56, 7.89, 1.0000000);
            let b = Tuplef32::new(1.23, 4.56, 7.89, 1.0000001);
            assert_approx_eq!(Tuplef32, a, b, ulps = 2);
        }
        {
            let a = Tuplef32::new(1.23, 4.56, 7.89, 0.0);
            let b = Tuplef32::new(1.23, 4.56, 7.89, 1.0);
            assert_approx_eq!(Tuplef32, a, b, epsilon = 1.0);
        }
    }

    #[test]
    #[should_panic]
    fn ne_1() {
        let a = Tuplef32::new(1.23, 4.56, 7.89, 1.000000);
        let b = Tuplef32::new(1.23, 4.56, 7.89, 1.000001);
        assert_approx_eq!(Tuplef32, a, b);
    }

    #[test]
    #[should_panic]
    fn ne_2() {
        let a = Tuplef32::new(1.23, 4.56, 7.89, 1.000000);
        let b = Tuplef32::new(1.23, 4.56, 7.89, 1.000001);
        assert_approx_eq!(Tuplef32, a, b, ulps = 2);
    }

    #[test]
    #[should_panic]
    fn ne_3() {
        let a = Tuplef32::new(1.23, 4.56, 7.89, 0.0000000);
        let b = Tuplef32::new(1.23, 4.56, 7.89, 1.0000001);
        assert_approx_eq!(Tuplef32, a, b, epsilon = 1.0);
    }
}

// pub trait Coord3DMath{
//     #[must_use]
//     fn
// }

impl Add for Tuplef32 {
    /// The resulting type after applying the `+` operator.
    type Output = Self;

    /// Performs the `+` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use rusty_ray_tracer::core3d::tuples::Tuple;
    /// # use rusty_ray_tracer::core3d::tuples::Tuplef32;
    /// let a = Tuplef32::new(1.23, 4.56, 7.89, 0.0);
    /// let b = Tuplef32::new(1.11, 2.22, 3.33, 1.0);
    /// let expected = Tuplef32::new(2.34, 6.78, 11.22, 1.0);
    /// assert_eq!(expected, a + b);
    /// ```
    #[must_use]
    fn add(self, rhs: Self) -> Self {
        Self::zip_for_each_collect(self, rhs, |a, b| a + b)
    }
}
