use std::fmt::Display;

use crate::{core3d::color_rgb::ColorRGB, graphics2d::canvas::Canvas};

#[derive(Default, Debug)]
pub struct PPM {
    pub version: &'static str,
    pub width: u32,
    pub height: u32,
    pub max_color: u32,

    pub colors: Vec<u32>,
}

const COMPONENTS_PER_COLOR: u32 = 3;
impl PPM {
    /// Creates a new ppm from x, y, z, w scaler values
    #[must_use]
    const fn new(width: u32, height: u32, max_color: u32, colors: Vec<u32>) -> Self {
        Self {
            version: "P3",
            width,
            height,
            max_color,
            colors,
        }
    }

    #[must_use]
    const fn is_color_index_new_line(&self, i: usize) -> bool {
        // assert!(i < self.colors.len());  // len() is non-const
        let stride = (self.width * COMPONENTS_PER_COLOR) as usize;
        i % stride == 0
    }

    #[must_use]
    const fn get_colors_string_prefix(&self, i: usize) -> &str {
        if self.is_color_index_new_line(i) {
            "\n"
        } else {
            ""
        }
    }

    #[must_use]
    const fn is_color_index_last_in_line(&self, i: usize) -> bool {
        // assert!(i < self.colors.len());  // len() is non-const
        let stride = (self.width * COMPONENTS_PER_COLOR) as usize;
        i % stride == stride - 1
    }

    #[must_use]
    const fn get_colors_string_postfix(&self, i: usize) -> &str {
        if self.is_color_index_last_in_line(i) {
            ""
        } else {
            " "
        }
    }
}

#[cfg(test)]
mod tests_ppm {
    use super::*;

    #[test]
    fn new() {
        let ppm = PPM::new(16, 9, 255, vec![0u32; 432]);
        assert_eq!("P3", ppm.version);
        assert_eq!(16, ppm.width);
        assert_eq!(9, ppm.height);
        assert_eq!(255, ppm.max_color);
        assert_eq!(16 * 9 * 3, ppm.colors.len());
        assert!(ppm.colors.iter().all(|&c| c == 0u32));
    }

    #[test]
    fn is_color_index_new_line() {
        let ppm = PPM::new(4, 3, 255, vec![0u32; 144]);
        let mut i = 0;
        for _y in 0..3 {
            assert_eq!(true, ppm.is_color_index_new_line(i));
            i += 1;
            for _o in 1..(4 * COMPONENTS_PER_COLOR) {
                assert_eq!(false, ppm.is_color_index_new_line(i));
                i += 1;
            }
        }
        assert_eq!(4 * 3 * 3, i);
    }

    #[test]
    fn get_colors_string_prefix() {
        let ppm = PPM::new(4, 3, 255, vec![0u32; 144]);
        let mut i = 0;
        for _y in 0..3 {
            assert_eq!("\n", ppm.get_colors_string_prefix(i));
            i += 1;
            for _o in 1..4 * 3 {
                assert_eq!("", ppm.get_colors_string_prefix(i));
                i += 1;
            }
        }
        assert_eq!(4 * 3 * 3, i);
    }

    #[test]
    fn is_color_index_last_in_line() {
        let ppm = PPM::new(4, 3, 255, vec![0u32; 144]);
        let mut i = 0;
        for _y in 0..3 {
            for _o in 0..(4 * 3) - 1 {
                assert_eq!(false, ppm.is_color_index_last_in_line(i));
                i += 1;
            }
            assert_eq!(true, ppm.is_color_index_last_in_line(i));
            i += 1;
        }
        assert_eq!(4 * 3 * 3, i);
    }

    #[test]
    fn get_colors_string_postfix() {
        let ppm = PPM::new(4, 3, 255, vec![0u32; 144]);
        let mut i = 0;
        for _y in 0..3 {
            for _o in 0..4 * 3 - 1 {
                assert_eq!(" ", ppm.get_colors_string_postfix(i));
                i += 1;
            }
            assert_eq!("", ppm.get_colors_string_postfix(i));
            i += 1;
        }
        assert_eq!(4 * 3 * 3, i);
    }
}

impl From<&Canvas> for PPM {
    /// Creates a new PPM file from an canvas
    ///
    /// # Examples
    ///
    /// ```
    // /// # use rusty_ray_tracer::core3d::ppm::PPM;
    // ///
    // /// let ppm = PPM::from([1.0, 2.0, 3.0, 4.0]);
    // /// assert_eq!([1.0, 2.0, 3.0, 4.0], ppm.ppm);
    /// ```
    fn from(canvas: &Canvas) -> Self {
        Self::new(
            u32::from(canvas.width),
            u32::from(canvas.height),
            255,
            canvas
                .raw_buffer
                .iter()
                .flat_map(|c| [c.r(), c.g(), c.b()])
                .map(|c| num::clamp(c, 0.0, 1.0))
                .map(|c| f32::round(c * 255.0) as u32)
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests_from {
    use crate::core3d::color;

    use super::*;

    #[test]
    fn new() {
        let ppm = PPM::from(&Canvas::new(16, 9));
        assert_eq!("P3", ppm.version);
        assert_eq!(16, ppm.width);
        assert_eq!(9, ppm.height);
        assert_eq!(255, ppm.max_color);
        assert_eq!(16 * 9 * 3, ppm.colors.len());
        assert!(ppm.colors.iter().all(|&c| c == 0u32));
    }

    #[test]
    fn new_with_data() {
        let mut canvas = Canvas::new(4, 3);
        canvas.set_pixel_at(0, 0, color::Color::new(1.0, 0.0, 0.0));
        canvas.set_pixel_at(1, 1, color::Color::new(0.0, 1.0, 0.0));
        canvas.set_pixel_at(2, 2, color::Color::new(0.0, 0.0, 1.0));
        let ppm = PPM::from(&canvas);
        assert_eq!("P3", ppm.version);
        assert_eq!(4, ppm.width);
        assert_eq!(3, ppm.height);
        assert_eq!(255, ppm.max_color);
        assert_eq!(4 * 3 * 3, ppm.colors.len());

        assert_eq!(ppm.colors[0 * 3 + 0], 255);
        assert_eq!(ppm.colors[0 * 3 + 1], 0);
        assert_eq!(ppm.colors[0 * 3 + 2], 0);

        assert_eq!(ppm.colors[5 * 3 + 0], 0);
        assert_eq!(ppm.colors[5 * 3 + 1], 255);
        assert_eq!(ppm.colors[5 * 3 + 2], 0);

        assert_eq!(ppm.colors[10 * 3 + 0], 0);
        assert_eq!(ppm.colors[10 * 3 + 1], 0);
        assert_eq!(ppm.colors[10 * 3 + 2], 255);
    }

    #[test]
    fn new_with_oor_data() {
        let mut canvas = Canvas::new(4, 3);
        canvas.set_pixel_at(0, 0, color::Color::new(3.0, 0.5, -0.2));
        canvas.set_pixel_at(1, 1, color::Color::new(-1.0, 1.5, 0.0));
        canvas.set_pixel_at(2, 2, color::Color::new(-0.5, 0.2, 1.5));
        let ppm = PPM::from(&canvas);
        assert_eq!("P3", ppm.version);
        assert_eq!(4, ppm.width);
        assert_eq!(3, ppm.height);
        assert_eq!(255, ppm.max_color);
        assert_eq!(4 * 3 * 3, ppm.colors.len());

        assert_eq!(ppm.colors[0 * 3 + 0], 255);
        assert_eq!(ppm.colors[0 * 3 + 1], 128);
        assert_eq!(ppm.colors[0 * 3 + 2], 0);

        assert_eq!(ppm.colors[5 * 3 + 0], 0);
        assert_eq!(ppm.colors[5 * 3 + 1], 255);
        assert_eq!(ppm.colors[5 * 3 + 2], 0);

        assert_eq!(ppm.colors[10 * 3 + 0], 0);
        assert_eq!(ppm.colors[10 * 3 + 1], 51);
        assert_eq!(ppm.colors[10 * 3 + 2], 255);
    }
}

// #[display(fmt = "{}\n{} {}\n{}\n{:?}", version, width, height, max_color, colors)]
impl Display for PPM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let colors = self
            .colors
            .iter()
            .enumerate()
            .fold(String::new(), |acc, (i, &c)| {
                let prefix = self.get_colors_string_prefix(i);
                let postfix = self.get_colors_string_postfix(i);
                acc + prefix + &c.to_string() + postfix
            });
        write!(
            f,
            "{}\n{} {}\n{}{}",
            self.version, self.width, self.height, self.max_color, colors
        )
    }
}

#[cfg(test)]
mod tests_display {
    use crate::core3d::color;

    use super::*;

    #[test]
    fn display_black() {
        let ppm = PPM::from(&Canvas::new(4, 3));
        let ppm_text = ppm.to_string();
        assert_eq!("P3\n4 3\n255\n0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0", ppm_text);
    }

    #[test]
    fn display_with_data() {
        let mut canvas = Canvas::new(4, 3);
        canvas.set_pixel_at(0, 0, color::Color::new(1.0, 0.0, 0.0));
        canvas.set_pixel_at(1, 1, color::Color::new(0.0, 1.0, 0.0));
        canvas.set_pixel_at(2, 2, color::Color::new(0.0, 0.0, 1.0));
        let ppm = PPM::from(&canvas);
        let ppm_text = ppm.to_string();
        assert_eq!("P3\n4 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 255 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 255 0 0 0", ppm_text);
    }

    #[test]
    fn display_with_oor_data() {
        let mut canvas = Canvas::new(4, 3);
        canvas.set_pixel_at(0, 0, color::Color::new(1.5, 0.0, 0.0));
        canvas.set_pixel_at(1, 1, color::Color::new(0.0, 0.5, 0.0));
        canvas.set_pixel_at(2, 2, color::Color::new(-0.5, 0.0, 1.0));
        let ppm = PPM::from(&canvas);
        let ppm_text = ppm.to_string();
        assert_eq!("P3\n4 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 255 0 0 0", ppm_text);
    }
}
