use super::{array_base::ArrayBase, coordinates4::Coordinates4};
use core::ops::Add;
use float_cmp::{approx_eq, ApproxEq};

#[derive(Copy, Clone, Default, Debug)]
pub struct Tuple {
    pub coords: [f32; 4],
}
impl Tuple {
    /// Creates a new tuple from x, y, z, w scaler values
    ///
    /// # Examples
    ///
    /// ```
    /// # use rusty_ray_tracer::core3d::tuple::Tuple;
    ///
    /// let tuple = Tuple::new(1.0, 2.0, 3.0, 4.0);
    /// assert_eq!([1.0, 2.0, 3.0, 4.0], tuple.coords);
    /// ```
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Tuple {
            coords: [x, y, z, w],
        }
    }
}

#[cfg(test)]
mod tests_tuple {
    use super::*;

    #[test]
    fn new() {
        let tuple = Tuple::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!([1.0, 2.0, 3.0, 4.0], tuple.coords);
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
    /// let tuple = Tuple::from([1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!([1.0, 2.0, 3.0, 4.0], tuple.coords);
    /// ```
    fn from(arr: [f32; 4]) -> Self {
        Tuple { coords: arr }
    }
}

#[cfg(test)]
mod tests_from {
    use super::*;

    #[test]
    fn from_array() {
        let tuple = Tuple::from([1.0, 2.0, 3.0, 4.0]);
        assert_eq!([1.0, 2.0, 3.0, 4.0], tuple.coords);
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
    /// let tuple = Tuple::new(1.0, 2.0, 3.0, 4.0);
    /// assert_eq!([1.0, 2.0, 3.0, 4.0], tuple.get_array());
    /// ```
    fn get_array(self) -> [f32; 4] {
        self.coords
    }

    /// Returns base array reference
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// let tuple = Tuple::new(1.0, 2.0, 3.0, 4.0);
    /// assert_eq!([1.0, 2.0, 3.0, 4.0], *tuple.get_array_ref());
    /// ```
    fn get_array_ref(&self) -> &[f32; 4] {
        &self.coords
    }

    /// Returns a mutable base array reference
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// let mut tuple = Tuple::new(1.0, 2.0, 3.0, 4.0);
    /// assert_eq!([1.0, 2.0, 3.0, 4.0], *tuple.get_array_mut());
    /// tuple.get_array_mut()[0] += 10.0;
    /// tuple.get_array_mut()[1] += 10.0;
    /// tuple.get_array_mut()[2] += 10.0;
    /// tuple.get_array_mut()[3] += 10.0;
    /// assert_eq!([11.0, 12.0, 13.0, 14.0], *tuple.get_array_mut());
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
        let tuple = Tuple::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!([1.0, 2.0, 3.0, 4.0], *tuple.get_array_ref());
        assert_eq!([1.0, 2.0, 3.0, 4.0], tuple.clone().get_array());
        assert_eq!([1.0, 2.0, 3.0, 4.0], *tuple.get_array_ref());
    }

    #[test]
    fn get_array_mut() {
        let mut tuple = Tuple::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!([1.0, 2.0, 3.0, 4.0], *tuple.get_array_mut());
        tuple.get_array_mut()[0] += 10.0;
        tuple.get_array_mut()[1] += 10.0;
        tuple.get_array_mut()[2] += 10.0;
        tuple.get_array_mut()[3] += 10.0;
        assert_eq!([11.0, 12.0, 13.0, 14.0], *tuple.get_array_mut());
        assert_eq!([11.0, 12.0, 13.0, 14.0], tuple.clone().get_array());
        assert_eq!([11.0, 12.0, 13.0, 14.0], *tuple.get_array_ref());
    }
}

impl Coordinates4 for Tuple {}

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
            Tuple::new(1.23, 4.56, 7.89, 0.00000000000000),
            Tuple::new(1.23, 4.56, 7.89, 0.00000000000001)
        );
        assert_eq!(
            Tuple::new(1.23, 4.56, 7.89, 0.0000000),
            Tuple::new(1.23, 4.56, 7.89, 0.0000001)
        );
        assert_eq!(
            Tuple::new(1.23, 4.56, 7.89, 1.0000000),
            Tuple::new(1.23, 4.56, 7.89, 1.0000001)
        );
        assert_eq!(
            Tuple::new(1.23, 4.56, 7.89, 1000000.0),
            Tuple::new(1.23, 4.56, 7.89, 1000000.1)
        );
    }

    #[test]
    fn ne() {
        assert_ne!(
            Tuple::new(1.23, 4.56, 7.89, 0.000010),
            Tuple::new(1.23, 4.56, 7.89, 0.000011)
        );
        assert_ne!(
            Tuple::new(1.23, 4.56, 7.89, 1.000000),
            Tuple::new(1.23, 4.56, 7.89, 1.000001)
        );
        assert_ne!(
            Tuple::new(1.23, 4.56, 7.89, 100000.0),
            Tuple::new(1.23, 4.56, 7.89, 100000.1)
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
            Tuple::new(1.23, 4.56, 7.89, 0.000000000000),
            Tuple::new(1.23, 4.56, 7.89, 0.000000000001)
        );
        assert_approx_eq!(
            Tuple,
            Tuple::new(1.23, 4.56, 7.89, 1.0000000),
            Tuple::new(1.23, 4.56, 7.89, 1.0000001),
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
            let a = Tuple::new(1.23, 4.56, 7.89, 1.000000);
            let b = Tuple::new(1.23, 4.56, 7.89, 1.000001);
            assert!(a.approx_ne(b, <Tuple as ApproxEq>::Margin::default()));
        }
        {
            let a = Tuple::new(1.23, 4.56, 7.89, 1.000000);
            let b = Tuple::new(1.23, 4.56, 7.89, 1.000001);
            assert!(a.approx_ne(b, <Tuple as ApproxEq>::Margin::default().ulps(2)));
        }
        {
            let a = Tuple::new(1.23, 4.56, 7.89, 0.0000000);
            let b = Tuple::new(1.23, 4.56, 7.89, 1.0000001);
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
    fn test_add() {
        let a = Tuple::new(1.23, 4.56, 7.89, 0.0);
        let b = Tuple::new(1.11, 2.22, 3.33, 1.0);
        let expected = Tuple::new(2.34, 6.78, 11.22, 1.0);
        assert_eq!(expected, a + b);
    }

    #[test]
    fn test_add_identity() {
        let a = Tuple::new(1.23, 4.56, 7.89, 0.0);
        let b = Tuple::default();
        assert_eq!(a + b, a);
    }

    #[test]
    fn test_add_commutative() {
        let a = Tuple::new(1.23, 4.56, 7.89, 0.0);
        let b = Tuple::new(1.11, 2.22, 3.33, 1.0);
        assert_eq!(a + b, b + a);
    }

    #[test]
    fn test_add_associative() {
        let a = Tuple::new(1.23, 4.56, 7.89, 1.01);
        let b = Tuple::new(1.11, 2.22, 3.33, 4.44);
        let c = Tuple::new(5.55, 6.66, 7.77, 8.88);
        assert_eq!(a + (b + c), (a + b) + c);
        assert_eq!(c + (a + b), (c + a) + b);
    }
}
