use std::ops::{Add, Mul, Sub};

use float_cmp::{approx_eq, ApproxEq};

use super::{array_base::ArrayBase, color_rgb::ColorRGB};

#[derive(Copy, Clone, Default, Debug)]
pub struct Color {
    pub tuple: [f32; 4],
}
impl Color {
    /// Creates a new Color
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::color::Color;
    /// # use crate::rusty_ray_tracer::core3d::color_rgb::ColorRGB;
    ///
    /// let color = Color::new(1.0, 2.0, 3.0);
    /// assert_eq!(1.0, color.tuple[0]);
    /// assert_eq!(2.0, color.tuple[1]);
    /// assert_eq!(3.0, color.tuple[2]);
    /// assert_eq!(1.0, color.tuple[3]);
    /// ```
    #[must_use]
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            tuple: [r, g, b, 1.0],
        }
    }

    // /// Creates a new Color
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// # use crate::rusty_ray_tracer::core3d::color::Color;
    // /// # use crate::rusty_ray_tracer::core3d::color_rgb::ColorRGB;
    // ///
    // /// let color = Color::nnew_with_alphaw(1.0, 2.0, 3.0, 4.0);
    // /// assert_eq!(1.0, color.tuple[0]);
    // /// assert_eq!(2.0, color.tuple[1]);
    // /// assert_eq!(3.0, color.tuple[2]);
    // /// assert_eq!(4.0, color.tuple[3]);
    // /// ```
    // #[must_use]
    // pub fn new_with_alpha(r: f32, g: f32, b: f32, a: f32) -> Color {
    //     Color {
    //         tuple: [r, g, b, a],
    //     }
    // }
}

#[cfg(test)]
mod tests_color {
    use super::*;

    #[test]
    fn new() {
        let color = Color::new(1.0, 2.0, 3.0);
        assert_eq!([1.0, 2.0, 3.0, 1.0], color.tuple);
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn copy_clone() {
        let color = Color::new(1.0, 2.0, 3.0);
        let color_copy = color;
        let color_clone = color.clone();
        assert_eq!([1.0, 2.0, 3.0, 1.0], color_copy.tuple);
        assert_eq!([1.0, 2.0, 3.0, 1.0], color_clone.tuple);
    }
}

impl From<[f32; 3]> for Color {
    /// Creates a new Color from an array of scaler values
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// # use crate::rusty_ray_tracer::core3d::color::Color;
    /// let color = Color::from([1.0, 2.0, 3.0]);
    /// assert_eq!([1.0, 2.0, 3.0, 1.0], color.tuple);
    /// ```
    fn from(arr: [f32; 3]) -> Self {
        Self::new(arr[0], arr[1], arr[2])
    }
}

// impl From<[f32; 4]> for Color {
//     /// Creates a new Color from an array of scaler values
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
//     /// # use crate::rusty_ray_tracer::core3d::color::Color;
//     /// let color = Color::from([1.0, 2.0, 3.0, 4.0]);
//     /// assert_eq!([1.0, 2.0, 3.0, 4.0], color.tuple);
//     /// ```
//     fn from(arr: [f32; 4]) -> Self {
//         Color::new_with_alpha(arr[0], arr[1], arr[2], arr[3])
//     }
// }

// impl From<Tuple> for Color {
//     /// Creates a new Color from a Tuple
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
//     /// # use crate::rusty_ray_tracer::core3d::color::Color;
//     /// let color = Color::from(Tuple::from([1.0, 2.0, 3.0, 1.0]));
//     /// assert_eq!([1.0, 2.0, 3.0, 1.0], color.tuple);
//     /// ```
//     ///
//     /// ```
//     /// # use std::panic;
//     /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
//     /// # use crate::rusty_ray_tracer::core3d::color::Color;
//     /// let tuple = Tuple::from([1.0, 2.0, 3.0, 4.0]);
//     /// assert_eq!([1.0, 2.0, 3.0, 4.0], Color::from(tuple).tuple);
//     /// ```
//     fn from(tuple: Tuple) -> Self {
//         Color::new_with_alpha(tuple.x(), tuple.y(), tuple.z(), tuple.w())
//     }
// }

// impl From<Color> for Tuple {
//     /// Creates a new Tuple from a Color
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
//     /// # use crate::rusty_ray_tracer::core3d::color::Color;
//     /// let result = Tuple::from(Color::from([1.0, 2.0, 3.0]));
//     /// assert_eq!([1.0, 2.0, 3.0, 1.0], result.tuple);
//     /// ```
//     fn from(color: Color) -> Self {
//         Tuple::from(color.tuple)
//     }
// }

// #[cfg(test)]
// mod tests_from {
//     use super::*;

//     #[test]
//     fn from_array() {
//         let color = Color::from([1.0, 2.0, 3.0]);
//         assert_eq!([1.0, 2.0, 3.0, 1.0], color.tuple);
//     }

//     #[test]
//     fn from_tuple() {
//         let color = Color::from(Tuple::new(1.0, 2.0, 3.0, 1.0));
//         assert_eq!([1.0, 2.0, 3.0, 1.0], color.tuple);

//         let tuple = Tuple::from([1.0, 2.0, 3.0, 4.0]);
//         assert_eq!([1.0, 2.0, 3.0, 4.0], Color::from(tuple).tuple);
//     }

//     #[test]
//     fn into_tuple() {
//         let tuple = Tuple::from(Color::new(1.0, 2.0, 3.0));
//         assert_eq!([1.0, 2.0, 3.0, 1.0], tuple.tuple);

//         let tuple: Tuple = Color::new(1.0, 2.0, 3.0).into();
//         assert_eq!([1.0, 2.0, 3.0, 1.0], tuple.tuple);
//     }
// }

impl ArrayBase for Color {
    type Item = f32;
    // type SizedArray = [f32; 4];

    /// Returns base array consuming
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::color::Color;
    /// let color = Color::new(1.0, 2.0, 3.0);
    /// assert_eq!([1.0, 2.0, 3.0, 1.0], color.get_array());
    /// ```
    fn get_array(self) -> [f32; 4] {
        self.tuple
    }

    /// Returns base array reference
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::color::Color;
    /// let color = Color::new(1.0, 2.0, 3.0);
    /// assert_eq!([1.0, 2.0, 3.0, 1.0], *color.get_array_ref());
    /// ```
    fn get_array_ref(&self) -> &[f32; 4] {
        &self.tuple
    }

    /// Returns a mutable base array reference
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::color::Color;
    /// let mut color = Color::new(1.0, 2.0, 3.0);
    /// assert_eq!([1.0, 2.0, 3.0, 1.0], *color.get_array_mut());
    /// color.get_array_mut()[0] += 10.0;
    /// color.get_array_mut()[1] += 10.0;
    /// color.get_array_mut()[2] += 10.0;
    /// color.get_array_mut()[3] += 10.0;
    /// assert_eq!([11.0, 12.0, 13.0, 11.0], *color.get_array_mut());
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
        let color = Color::new(1.0, 2.0, 3.0);
        assert_eq!([1.0, 2.0, 3.0, 1.0], *color.get_array_ref());
        assert_eq!([1.0, 2.0, 3.0, 1.0], color.get_array());
        assert_eq!([1.0, 2.0, 3.0, 1.0], *color.get_array_ref());
    }

    #[test]
    fn get_array_mut() {
        let mut color = Color::new(1.0, 2.0, 3.0);
        assert_eq!([1.0, 2.0, 3.0, 1.0], *color.get_array_mut());
        color.get_array_mut()[0] += 10.0;
        color.get_array_mut()[1] += 10.0;
        color.get_array_mut()[2] += 10.0;
        color.get_array_mut()[3] += 10.0;
        assert_eq!([11.0, 12.0, 13.0, 11.0], *color.get_array_mut());
        assert_eq!([11.0, 12.0, 13.0, 11.0], color.get_array());
        assert_eq!([11.0, 12.0, 13.0, 11.0], *color.get_array_ref());
    }
}

impl ColorRGB for Color {}

#[cfg(test)]
mod tests_colorrgba {
    use super::*;
    use crate::core3d::color_rgb::ColorRGB;

    #[test]
    fn assign_array() {
        let color: Color = Color::from([3.0, 2.0, 1.0]);
        assert_eq!(3.0, color.r());
        assert_eq!(2.0, color.g());
        assert_eq!(1.0, color.b());
        // assert_eq!(1.0, color.a());
    }

    #[test]
    fn create_new() {
        let color = Color::new(1.0, 2.0, 3.0);
        assert_eq!(1.0, color.r());
        assert_eq!(2.0, color.g());
        assert_eq!(3.0, color.b());
        // assert_eq!(1.0, color.a());
    }
}

impl PartialEq for Color {
    /// Performs the `=` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::color::Color;
    /// let a = Color::new(1.23, 4.56, 0.0);
    /// let b = Color::new(1.23, 4.56, 0.0);
    /// assert_eq!(a, b);
    /// ```
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::color::Color;
    /// let a = Color::new(1.23, 4.56, 1.000000);
    /// let b = Color::new(1.23, 4.56, 1.000001);
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
            Color::new(1.23, 4.56, 0.000_000_000_000_00),
            Color::new(1.23, 4.56, 0.000_000_000_000_01)
        );
        assert_eq!(
            Color::new(1.23, 4.56, 0.000_000_0),
            Color::new(1.23, 4.56, 0.000_000_1)
        );
        assert_eq!(
            Color::new(1.23, 4.56, 1.000_000_0),
            Color::new(1.23, 4.56, 1.000_000_1)
        );
        assert_eq!(
            Color::new(1.23, 4.56, 1_000_000.0),
            Color::new(1.23, 4.56, 1_000_000.1)
        );
    }

    #[test]
    fn ne() {
        assert_ne!(
            Color::new(1.23, 4.56, 0.000_010),
            Color::new(1.23, 4.56, 0.000_011)
        );
        assert_ne!(
            Color::new(1.23, 4.56, 1.000_000),
            Color::new(1.23, 4.56, 1.000_001)
        );
        assert_ne!(
            Color::new(1.23, 4.56, 100_000.0),
            Color::new(1.23, 4.56, 100_000.1)
        );
    }
}

impl ApproxEq for Color {
    type Margin = <f32 as ApproxEq>::Margin;

    /// Performs the `~=` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::color::Color;
    /// # use float_cmp::ApproxEq;
    /// let a = Color::new(1.23, 4.56, 0.000000000000);
    /// let b = Color::new(1.23, 4.56, 0.000000000001);
    /// assert!(a.approx_eq(b, <Color as ApproxEq>::Margin::default()));
    /// ```
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::color::Color;
    /// # use float_cmp::ApproxEq;
    /// let a = Color::new(1.23, 4.56, 1.0000000);
    /// let b = Color::new(1.23, 4.56, 1.0000001);
    /// assert!(a.approx_eq(b, <Color as ApproxEq>::Margin::default().ulps(2)));
    /// ```
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::color::Color;
    /// # use float_cmp::ApproxEq;
    /// let a = Color::new(1.23, 4.56, 0.0);
    /// let b = Color::new(1.23, 4.56, 1.0);
    /// assert!(a.approx_eq(b, <Color as ApproxEq>::Margin::default().epsilon(1.0)));
    /// ```
    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        Self::into_zip(self, other).all(|(a, b)| a.approx_eq(b, margin))
    }
}

#[cfg(test)]
mod tests_approx_eq {
    use super::*;
    use float_cmp::{assert_approx_eq, ApproxEq};
    use std::panic;

    #[test]
    fn eq() {
        assert_approx_eq!(
            Color,
            Color::new(1.23, 4.56, 0.000_000_000_000),
            Color::new(1.23, 4.56, 0.000_000_000_001)
        );
        assert_approx_eq!(
            Color,
            Color::new(1.23, 4.56, 1.000_000_0),
            Color::new(1.23, 4.56, 1.000_000_1),
            ulps = 2
        );
        assert_approx_eq!(
            Color,
            Color::new(1.23, 4.56, 0.0),
            Color::new(1.23, 4.56, 1.0),
            epsilon = 1.0
        );
    }

    #[test]
    fn ne() {
        {
            let a = Color::new(1.23, 4.56, 1.000_000);
            let b = Color::new(1.23, 4.56, 1.000_001);
            assert!(a.approx_ne(b, <Color as ApproxEq>::Margin::default()));
        }
        {
            let a = Color::new(1.23, 4.56, 1.000_000);
            let b = Color::new(1.23, 4.56, 1.000_001);
            assert!(a.approx_ne(b, <Color as ApproxEq>::Margin::default().ulps(2)));
        }
        {
            let a = Color::new(1.23, 4.56, 0.000_000_0);
            let b = Color::new(1.23, 4.56, 1.000_000_1);
            assert!(a.approx_ne(b, <Color as ApproxEq>::Margin::default().epsilon(1.0)));
        }
    }
}

impl Add for Color {
    /// The resulting type after applying the `+` operator.
    type Output = Self;

    /// Performs the `+` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::color::Color;
    /// let a = Color::new(1.23, 4.56, 7.89);
    /// let b = Color::new(1.11, 2.22, 3.33);
    /// let expected = Color::new(2.34, 6.78, 11.22);
    /// assert_eq!(expected, a + b);
    /// ```
    #[must_use]
    fn add(self, rhs: Self) -> Self {
        let result = Self::zip_for_each_collect(self, rhs, |a, b| a + b);
        // HACK: Alpha is not supported, Alpha requires blending and does not directly translate to Addition or Subtraction
        // let a = result.a().clamp(0.0, 1.0);
        Self::new(result.r(), result.g(), result.b())
    }
}

#[cfg(test)]
mod tests_add {
    use super::*;

    #[test]
    fn closure() {
        let a = Color::new(1.23, 4.56, 7.89);
        let b = Color::new(1.11, 2.22, 3.33);
        let expected = Color::new(2.34, 6.78, 11.22);
        assert_eq!(expected, a + b);
    }

    #[test]
    fn identity() {
        let a = Color::new(1.23, 4.56, 7.89);
        let b = Color::default();
        assert_eq!(a + b, a);
        assert_eq!(b + a, a);
    }

    #[test]
    fn commutative() {
        let a = Color::new(1.23, 4.56, 7.89);
        let b = Color::new(1.11, 2.22, 3.33);
        assert_eq!(a + b, b + a);
    }

    #[test]
    fn associative() {
        let a = Color::new(1.23, 4.56, 7.89);
        let b = Color::new(1.11, 2.22, 3.33);
        let c = Color::new(5.55, 6.66, 7.77);
        assert_eq!(a + (b + c), (a + b) + c);
        assert_eq!(c + (a + b), (c + a) + b);
    }
}

impl Sub for Color {
    /// The resulting type after applying the `-` operator.
    type Output = Self;

    /// Performs the `-` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::color::Color;
    /// let a = Color::new(1.23, 4.56, 7.89);
    /// let b = Color::new(1.11, 2.22, 3.33);
    /// let expected = Color::new(0.12, 2.34, 4.56);
    /// assert_eq!(expected, a - b);
    /// ```
    #[must_use]
    fn sub(self, rhs: Self) -> Self::Output {
        let result = Self::zip_for_each_collect(self, rhs, |a, b| a - b);
        // HACK: Alpha is not supported, Alpha requires blending and does not directly translate to Addition or Subtraction
        // let a = result.a().clamp(0.0, 1.0);
        Self::new(result.r(), result.g(), result.b())
    }
}

#[cfg(test)]
mod tests_sub {
    use super::*;

    #[test]
    fn not_closure() {
        let a = Color::new(1.23, 4.56, 7.89);
        let b = Color::new(1.11, 2.22, 3.33);
        let expected = Color::new(0.12, 2.34, 4.56);
        assert_eq!(expected, a - b);
    }

    #[test]
    fn not_identity() {
        let a = Color::new(1.23, 4.56, 7.89);
        let b = Color::default();
        let ab = Color::new(1.23, 4.56, 7.89);
        assert_eq!(ab, a - b);
        assert_ne!(ab, b - a);
        let ba = Color::new(-1.23, -4.56, -7.89);
        assert_eq!(ba, b - a);
    }

    #[test]
    fn not_commutative() {
        let a = Color::new(1.23, 4.56, 7.89);
        let b = Color::new(1.11, 2.22, 3.33);
        assert_ne!(a - b, b - a);

        let ab = Color::new(0.12, 2.34, 4.56);
        let ba = Color::new(-0.12, -2.34, -4.56);
        assert_eq!(ab, a - b);
        assert_eq!(ba, b - a);
    }

    #[test]
    fn not_associative() {
        let a = Color::new(1.23, 4.56, 7.89);
        let b = Color::new(1.11, 2.22, 3.33);
        let c = Color::new(5.55, 6.66, 7.77);
        assert_ne!(a - (b - c), (a - b) - c);
        assert_ne!(c - (a - b), (c - a) - b);

        let a_bc = Color::new(5.67, 9.0, 12.33);
        let ab_c = Color::new(-5.43, -4.32, -3.21);
        let c_ab = Color::new(5.43, 4.32, 3.21);
        let ca_b = Color::new(3.21, -0.120_000_124, -3.45);
        assert_eq!(a_bc, a - (b - c));
        assert_eq!(ab_c, (a - b) - c);
        assert_eq!(c_ab, c - (a - b));
        assert_eq!(ca_b, (c - a) - b);
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    /// Performs the `*` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::color::Color;
    /// let result = Color::new(1.11, -2.22, 3.33) * 100.1;
    /// let expected = Color::new(111.111, -222.222, 333.333);
    /// assert_eq!(expected, result);
    /// ```
    fn mul(self, rhs: f32) -> Self::Output {
        let mut result = self;
        result.iter_mut().for_each(|x| *x *= rhs);
        Self::new(result.r(), result.g(), result.b())
    }
}

#[cfg(test)]
mod tests_mul {
    use super::*;

    #[test]
    fn closure() {
        let result = Color::new(1.11, -2.22, 3.33) * 100.1;
        let expected = Color::new(111.111, -222.222, 333.333);
        assert_eq!(result, expected);
    }

    #[test]
    fn identity() {
        let a = Color::new(1.23, 4.56, 7.89);
        let b = 1.0;
        assert_eq!(a * b, a);
    }
}

impl Mul for Color {
    type Output = Self;

    /// Performs the Hadamard product (or Schur product) `*` operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::color::Color;
    /// let result = Color::new(1.11, -2.22, 3.33) * Color::new(100.1, 100.1, 100.1);
    /// let expected = Color::new(111.111, -222.222, 333.333);
    /// assert_eq!(expected, result);
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        Self::zip_for_each_collect(self, rhs, |a, b| a * b)
    }
}

#[cfg(test)]
mod tests_product {
    use super::*;

    #[test]
    fn closure() {
        let result = Color::new(1.11, -2.22, 3.33) * Color::new(100.1, 100.1, 100.1);
        let expected = Color::new(111.111, -222.222, 333.333);
        assert_eq!(result, expected);
    }

    #[test]
    fn identity() {
        let a = Color::new(1.23, 4.56, 7.89);
        let b = Color::new(1.0, 1.0, 1.0);
        assert_eq!(a * b, a);
        assert_eq!(b * a, a);
    }

    #[test]
    fn commutative() {
        let a = Color::new(1.23, 4.56, 7.89);
        let b = Color::new(1.11, 2.22, 3.33);
        assert_eq!(a * b, b * a);
    }

    #[test]
    fn associative() {
        let a = Color::new(1.23, 4.56, 7.89);
        let b = Color::new(1.11, 2.22, 3.33);
        let c = Color::new(5.55, 6.66, 7.77);
        assert_eq!(a * (b * c), (a * b) * c);
        assert_eq!(c * (a * b), (c * a) * b);
    }
}
