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
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
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
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
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
