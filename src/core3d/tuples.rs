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
/// todo! Matrix maybe a better name for this
pub trait Tuple: Sized + IntoIterator {
    /// Gets the specified dimension from the tuple
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::tuples::Tuple;
    /// # use rusty_ray_tracer::core3d::tuples::Tuplef32;
    /// let tuple = Tuplef32::new(0.0, 1.0, 2.0, 3.0);
    /// assert_eq!(0.0, tuple.get_at(0));
    /// assert_eq!(1.0, tuple.get_at(1));
    /// assert_eq!(2.0, tuple.get_at(2));
    /// assert_eq!(3.0, tuple.get_at(3));
    /// ```
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
    fn is_valid(&self) -> bool {
        debug_assert!(self.is_point() || self.is_vector());
        self.is_point() || self.is_vector()
    }

    #[must_use]
    fn iter(&self) -> Iter<'_, f32>;

    #[must_use]
    fn iter_mut(&mut self) -> IterMut<'_, f32>;

    // Combines both Tuples into one using a closure
    #[must_use]
    fn zip_for_each_collect<B>(a: Self, b: B, f: impl Fn(f32, f32) -> f32) -> Self
    where
        B: IntoIterator<Item = f32>,
    {
        let mut result = a;
        result
            .iter_mut()
            .zip(b.into_iter())
            .for_each(|(i, j)| *i = f(*i, j));
        result.is_valid();
        result
    }

    // Combines both Tuples into one using a closure
    fn zip<'a, 'b>(a: &'a Self, b: &'b Self) -> Zip<Iter<'a, f32>, Iter<'b, f32>> {
        a.iter().zip(b.iter())
    }

    // Combines both Tuples into one using a closure
    fn into_zip(
        a: Self,
        b: Self,
    ) -> Zip<<Self as IntoIterator>::IntoIter, <Self as IntoIterator>::IntoIter> {
        a.into_iter().zip(b.into_iter())
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
    use std::panic;

    #[test]
    fn assign_array() {
        let tuple = Tuplef32::new(0.0, 1.0, 2.0, 3.0);
        assert_eq!(0.0, tuple.get_at(0));
        assert_eq!(1.0, tuple.get_at(1));
        assert_eq!(2.0, tuple.get_at(2));
        assert_eq!(3.0, tuple.get_at(3));

        assert!(panic::catch_unwind(|| tuple.get_at(4)).is_err());
        assert!(panic::catch_unwind(|| tuple.get_at(usize::MAX)).is_err());
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
        assert!(tuple.is_valid());
        assert!(tuple.is_point() == true);
        assert!(tuple.is_vector() == false);
    }

    #[test]
    fn is_vector() {
        let tuple = Tuplef32::new(1.23, 4.56, 7.89, 0.0);
        assert!(tuple.is_valid());
        assert!(tuple.is_vector() == true);
        assert!(tuple.is_point() == false);
    }

    #[test]
    fn invalid_point_or_vector() {
        let tuple = Tuplef32::new(1.23, 4.56, 7.89, 1.01);
        assert!(tuple.is_point() == false);
        assert!(tuple.is_vector() == false);
        assert!(panic::catch_unwind(|| tuple.is_valid()).is_err());
    }
}

impl IntoIterator for Tuplef32 {
    type Item = f32;
    type IntoIter = std::array::IntoIter<Self::Item, 4_usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.coords.into_iter()
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
    /// let a = Tuplef32::new(1.23, 4.56, 7.89, 1.000000);
    /// let b = Tuplef32::new(1.23, 4.56, 7.89, 1.000001);
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
            Tuplef32::new(1.23, 4.56, 7.89, 0.00000000000000),
            Tuplef32::new(1.23, 4.56, 7.89, 0.00000000000001)
        );
        assert_eq!(
            Tuplef32::new(1.23, 4.56, 7.89, 0.0000000),
            Tuplef32::new(1.23, 4.56, 7.89, 0.0000001)
        );
        assert_eq!(
            Tuplef32::new(1.23, 4.56, 7.89, 1.0000000),
            Tuplef32::new(1.23, 4.56, 7.89, 1.0000001)
        );
        assert_eq!(
            Tuplef32::new(1.23, 4.56, 7.89, 1000000.0),
            Tuplef32::new(1.23, 4.56, 7.89, 1000000.1)
        );
    }

    #[test]
    fn ne() {
        assert_ne!(
            Tuplef32::new(1.23, 4.56, 7.89, 0.000010),
            Tuplef32::new(1.23, 4.56, 7.89, 0.000011)
        );
        assert_ne!(
            Tuplef32::new(1.23, 4.56, 7.89, 1.000000),
            Tuplef32::new(1.23, 4.56, 7.89, 1.000001)
        );
        assert_ne!(
            Tuplef32::new(1.23, 4.56, 7.89, 100000.0),
            Tuplef32::new(1.23, 4.56, 7.89, 100000.1)
        );
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
    /// # use float_cmp::ApproxEq;
    /// let a = Tuplef32::new(1.23, 4.56, 7.89, 0.000000000000);
    /// let b = Tuplef32::new(1.23, 4.56, 7.89, 0.000000000001);
    /// assert!(a.approx_eq(b, <Tuplef32 as ApproxEq>::Margin::default()));
    /// ```
    ///
    /// ```
    /// # use rusty_ray_tracer::core3d::tuples::Tuple;
    /// # use rusty_ray_tracer::core3d::tuples::Tuplef32;
    /// # use float_cmp::ApproxEq;
    /// let a = Tuplef32::new(1.23, 4.56, 7.89, 1.0000000);
    /// let b = Tuplef32::new(1.23, 4.56, 7.89, 1.0000001);
    /// assert!(a.approx_eq(b, <Tuplef32 as ApproxEq>::Margin::default().ulps(2)));
    /// ```
    ///
    /// ```
    /// # use rusty_ray_tracer::core3d::tuples::Tuple;
    /// # use rusty_ray_tracer::core3d::tuples::Tuplef32;
    /// # use float_cmp::ApproxEq;
    /// let a = Tuplef32::new(1.23, 4.56, 7.89, 0.0);
    /// let b = Tuplef32::new(1.23, 4.56, 7.89, 1.0);
    /// assert!(a.approx_eq(b, <Tuplef32 as ApproxEq>::Margin::default().epsilon(1.0)));
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
            Tuplef32,
            Tuplef32::new(1.23, 4.56, 7.89, 0.000000000000),
            Tuplef32::new(1.23, 4.56, 7.89, 0.000000000001)
        );
        assert_approx_eq!(
            Tuplef32,
            Tuplef32::new(1.23, 4.56, 7.89, 1.0000000),
            Tuplef32::new(1.23, 4.56, 7.89, 1.0000001),
            ulps = 2
        );
        assert_approx_eq!(
            Tuplef32,
            Tuplef32::new(1.23, 4.56, 7.89, 0.0),
            Tuplef32::new(1.23, 4.56, 7.89, 1.0),
            epsilon = 1.0
        );
    }

    #[test]
    fn ne() {
        {
            let a = Tuplef32::new(1.23, 4.56, 7.89, 1.000000);
            let b = Tuplef32::new(1.23, 4.56, 7.89, 1.000001);
            assert!(a.approx_ne(b, <Tuplef32 as ApproxEq>::Margin::default()));
        }
        {
            let a = Tuplef32::new(1.23, 4.56, 7.89, 1.000000);
            let b = Tuplef32::new(1.23, 4.56, 7.89, 1.000001);
            assert!(a.approx_ne(b, <Tuplef32 as ApproxEq>::Margin::default().ulps(2)));
        }
        {
            let a = Tuplef32::new(1.23, 4.56, 7.89, 0.0000000);
            let b = Tuplef32::new(1.23, 4.56, 7.89, 1.0000001);
            assert!(a.approx_ne(b, <Tuplef32 as ApproxEq>::Margin::default().epsilon(1.0)));
        }
    }
}

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
