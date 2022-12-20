use std::vec;

use crate::core3d::color::Color;

#[derive(Default, Debug)]
pub struct Canvas {
    pub width: u16,
    pub height: u16,

    pub raw_buffer: Vec<Color>,
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
    #[must_use]
    pub fn new(width: u16, height: u16) -> Self {
        let size = width as usize * height as usize;
        Self {
            width,
            height,
            raw_buffer: vec::from_elem(Color::default(), size),
        }
    }
}

#[cfg(test)]
mod tests_canvas {
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

    #[test]
    fn debug_fmt() {
        let canvas = Canvas::new(2, 2);
        assert_eq!("Canvas { width: 2, height: 2, raw_buffer: [Color { tuple: [0.0, 0.0, 0.0, 0.0] }, Color { tuple: [0.0, 0.0, 0.0, 0.0] }, Color { tuple: [0.0, 0.0, 0.0, 0.0] }, Color { tuple: [0.0, 0.0, 0.0, 0.0] }] }", format!("{:?}", canvas));
    }
}

/// Calculates the raw buffer index from given 2d coordinates
const fn get_2dbuffer_index(x: u16, y: u16, width: u16, height: u16) -> Option<usize> {
    debug_assert!(x < width);
    debug_assert!(y < height);
    if x >= width || y >= height {
        return None;
    }
    Some(y as usize * width as usize + x as usize)
}

#[cfg(test)]
mod tests_buffer {
    use super::*;

    #[test]
    fn calc_2dbuffer_index() {
        assert_eq!(Some(0), get_2dbuffer_index(0, 0, 8, 8));
        assert_eq!(Some(1), get_2dbuffer_index(1, 0, 8, 8));
        assert_eq!(Some(8), get_2dbuffer_index(0, 1, 8, 8));
        assert_eq!(Some(9), get_2dbuffer_index(1, 1, 8, 8));
        assert_eq!(Some(63), get_2dbuffer_index(7, 7, 8, 8));
    }
}

impl Canvas {
    /// Calculates the raw buffer index from given 2d coordinates
    const fn get_2dbuffer_index(&self, x: u16, y: u16) -> Option<usize> {
        get_2dbuffer_index(x, y, self.width, self.height)
    }

    /// Get the pixel color at coordinates x, y
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
    ///         let pixel = canvas.get_pixel_at(x, y);
    ///         assert_eq!(0.0, pixel.r());
    ///         assert_eq!(0.0, pixel.g());
    ///         assert_eq!(0.0, pixel.b());
    ///     }
    /// }
    /// ```
    #[must_use]
    pub fn get_pixel_at(&self, x: u16, y: u16) -> Color {
        let idx = self
            .get_2dbuffer_index(x, y)
            .expect("Canvas Coordinates out of range!");
        self.raw_buffer[idx]
    }

    /// Sets the pixel color at coordinates x, y
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
    ///         let pixel = canvas.get_pixel_at(x, y);
    ///         assert_eq!(0.0, pixel.r());
    ///         assert_eq!(0.0, pixel.g());
    ///         assert_eq!(0.0, pixel.b());
    ///     }
    /// }
    /// ```
    pub fn set_pixel_at(&mut self, x: u16, y: u16, color: Color) {
        let idx = self
            .get_2dbuffer_index(x, y)
            .expect("Canvas Coordinates out of range!");
        self.raw_buffer[idx] = color;
    }
}
