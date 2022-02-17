use crate::core3d::array_base::ArrayBase;
use crate::core3d::coordinates4::Coordinates4;
use core::ops::Add;
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

#[derive(Copy, Clone, Default, Debug)]
pub struct Tuple {
    pub coords: [f32; 4],
}
impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Tuple {
            coords: [x, y, z, w],
        }
    }
}
impl ArrayBase for Tuple {
    type Item = f32;
    // type SizedArray = [f32; 4];

    fn get_array(self) -> [f32; 4] {
        self.coords
    }

    fn get_array_ref(&self) -> &[f32; 4] {
        &self.coords
    }

    fn get_array_mut(&mut self) -> &mut [f32; 4] {
        &mut self.coords
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
    /// # use crate::rusty_ray_tracer::core3d::tuples::Tuple;
    /// let a = Tuple::new(1.23, 4.56, 7.89, 0.0);
    /// let b = Tuple::new(1.23, 4.56, 7.89, 0.0);
    /// assert_eq!(a, b);
    /// ```
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::tuples::Tuple;
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

impl ApproxEq for Tuple
{
    type Margin = <f32 as ApproxEq>::Margin;

    /// Performs the `~=` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::tuples::Tuple;
    /// # use float_cmp::ApproxEq;
    /// let a = Tuple::new(1.23, 4.56, 7.89, 0.000000000000);
    /// let b = Tuple::new(1.23, 4.56, 7.89, 0.000000000001);
    /// assert!(a.approx_eq(b, <Tuple as ApproxEq>::Margin::default()));
    /// ```
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::tuples::Tuple;
    /// # use float_cmp::ApproxEq;
    /// let a = Tuple::new(1.23, 4.56, 7.89, 1.0000000);
    /// let b = Tuple::new(1.23, 4.56, 7.89, 1.0000001);
    /// assert!(a.approx_eq(b, <Tuple as ApproxEq>::Margin::default().ulps(2)));
    /// ```
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::tuples::Tuple;
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

impl Add for Tuple
{
    /// The resulting type after applying the `+` operator.
    type Output = Self;

    /// Performs the `+` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::tuples::Tuple;
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
