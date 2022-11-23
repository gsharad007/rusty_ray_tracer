use super::{tuple::Tuple, array_base::ArrayBase, color_rgba::ColorRGBA, coordinates4::Coordinates4};

#[derive(Copy, Clone, Debug)]
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
    /// # use crate::rusty_ray_tracer::core3d::color_rgba::ColorRGBA;
    ///
    /// let color = Color::new(1.0, 2.0, 3.0);
    /// assert_eq!(1.0, color.tuple[0]);
    /// assert_eq!(2.0, color.tuple[1]);
    /// assert_eq!(3.0, color.tuple[2]);
    /// assert_eq!(1.0, color.tuple[3]);
    /// ```
    #[must_use]
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color {
            tuple: [r, g, b, 1.0],
        }
    }

    /// Creates a new Color
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::color::Color;
    /// # use crate::rusty_ray_tracer::core3d::color_rgba::ColorRGBA;
    ///
    /// let color = Color::new(1.0, 2.0, 3.0);
    /// assert_eq!(1.0, color.tuple[0]);
    /// assert_eq!(2.0, color.tuple[1]);
    /// assert_eq!(3.0, color.tuple[2]);
    /// assert_eq!(1.0, color.tuple[3]);
    /// ```
    #[must_use]
    pub fn new_with_alpha(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {
            tuple: [r, g, b, a],
        }
    }
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
    fn copy() {
        let color = Color::new(1.0, 2.0, 3.0);
        assert_eq!([1.0, 2.0, 3.0, 1.0], color.tuple);
    }
}

impl Default for Color {
    /// Creates a new Color with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::color::Color;
    /// # use crate::rusty_ray_tracer::core3d::color_rgba::ColorRGBA;
    ///
    /// let color = Color::default();
    /// assert_eq!(0.0, color.tuple[0]);
    /// assert_eq!(0.0, color.tuple[1]);
    /// assert_eq!(0.0, color.tuple[2]);
    /// assert_eq!(1.0, color.tuple[3]);
    /// ```
    fn default() -> Self {
        Self::new(Default::default(), Default::default(), Default::default())
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
        Color::new(arr[0], arr[1], arr[2])
    }
}

impl From<[f32; 4]> for Color {
    /// Creates a new Color from an array of scaler values
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// # use crate::rusty_ray_tracer::core3d::color::Color;
    /// let color = Color::from([1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!([1.0, 2.0, 3.0, 4.0], color.tuple);
    /// ```
    fn from(arr: [f32; 4]) -> Self {
        Color::new_with_alpha(arr[0], arr[1], arr[2], arr[3])
    }
}

impl From<Tuple> for Color {
    /// Creates a new Color from a Tuple
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// # use crate::rusty_ray_tracer::core3d::color::Color;
    /// let color = Color::from(Tuple::from([1.0, 2.0, 3.0, 1.0]));
    /// assert_eq!([1.0, 2.0, 3.0, 1.0], color.tuple);
    /// ```
    ///
    /// ```
    /// # use std::panic;
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// # use crate::rusty_ray_tracer::core3d::color::Color;
    /// let tuple = Tuple::from([1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!([1.0, 2.0, 3.0, 4.0], Color::from(tuple).tuple);
    /// ```
    fn from(tuple: Tuple) -> Self {
        Color::new_with_alpha(tuple.x(), tuple.y(), tuple.z(), tuple.w())
    }
}

impl From<Color> for Tuple {
    /// Creates a new Tuple from a Color
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// # use crate::rusty_ray_tracer::core3d::color::Color;
    /// let result = Tuple::from(Color::from([1.0, 2.0, 3.0]));
    /// assert_eq!([1.0, 2.0, 3.0, 1.0], result.coords);
    /// ```
    fn from(color: Color) -> Self {
        Tuple::from(color.tuple)
    }
}

#[cfg(test)]
mod tests_from {
    use super::*;

    #[test]
    fn from_array() {
        let color = Color::from([1.0, 2.0, 3.0]);
        assert_eq!([1.0, 2.0, 3.0, 1.0], color.tuple);
    }

    #[test]
    fn from_tuple() {
        let color = Color::from(Tuple::new(1.0, 2.0, 3.0, 1.0));
        assert_eq!([1.0, 2.0, 3.0, 1.0], color.tuple);

        let tuple = Tuple::from([1.0, 2.0, 3.0, 4.0]);
        assert_eq!([1.0, 2.0, 3.0, 4.0], Color::from(tuple).tuple);
    }

    #[test]
    fn into_tuple() {
        let tuple = Tuple::from(Color::new(1.0, 2.0, 3.0));
        assert_eq!([1.0, 2.0, 3.0, 1.0], tuple.coords);

        let tuple: Tuple = Color::new(1.0, 2.0, 3.0).into();
        assert_eq!([1.0, 2.0, 3.0, 1.0], tuple.coords);
    }
}

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

impl ColorRGBA for Color {}

#[cfg(test)]
mod tests_colorrgba {
    use super::*;
    use crate::core3d::color_rgba::ColorRGBA;

    #[test]
    fn assign_array() {
        let color: Color = Color::from([3.0, 2.0, 1.0]);
        assert_eq!(3.0, color.r());
        assert_eq!(2.0, color.g());
        assert_eq!(1.0, color.b());
        assert_eq!(1.0, color.a());
    }

    #[test]
    fn create_new() {
        let color = Color::new(1.0, 2.0, 3.0);
        assert_eq!(1.0, color.r());
        assert_eq!(2.0, color.g());
        assert_eq!(3.0, color.b());
        assert_eq!(1.0, color.a());
    }
}
