use crate::core3d::color::Color;

#[derive(Default, Debug)]
pub struct Canvas {
    pub width: u16,
    pub height: u16,
}

impl Canvas {
    /// Creates a new Canvas with dimensions width, height
    ///
    /// # Examples
    ///
    /// ```
    /// # use rusty_ray_tracer::graphics2d::canvas::Canvas;
    /// let canvas = Canvas::new(10, 20);
    /// assert_eq!(10, canvas.width);
    /// assert_eq!(20, canvas.height);
    /// ```
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

#[cfg(test)]
mod tests_tuple {
    use super::*;

    #[test]
    fn new() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(10, canvas.width);
        assert_eq!(20, canvas.height);

        let canvas = Canvas::new(960, 540);
        assert_eq!(960, canvas.width);
        assert_eq!(540, canvas.height);
    }
}

impl Canvas {
    /// Creates a new Canvas with dimensions width, height
    ///
    /// # Examples
    ///
    /// ```
    /// # use rusty_ray_tracer::graphics2d::canvas::Canvas;
    /// # use crate::rusty_ray_tracer::core3d::color_rgb::ColorRGB;
    /// let canvas = Canvas::new(8, 8);
    /// assert_eq!(8, canvas.width);
    /// assert_eq!(8, canvas.height);
    /// for y in 0..canvas.height
    /// {
    ///     for x in 0..canvas.width
    ///     {
    ///         let pixel = canvas.get_pixel(x, y);
    ///         assert_eq!(0.0, pixel.r());
    ///         assert_eq!(0.0, pixel.g());
    ///         assert_eq!(0.0, pixel.b());
    ///     }
    /// }
    /// ```
    pub fn get_pixel(&self, _x: u16, _y: u16) -> Color {
        Color::default()
    }
}
