use derive_more::Display;

use crate::{core3d::color_rgb::ColorRGB, graphics2d::canvas::Canvas};

#[derive(Display, Default, Debug)]
#[display(fmt = "{}\n{} {}\n{}\n{:?}", version, width, height, max_color, colors)]
pub struct PPM {
    pub version: &'static str,
    pub width: u32,
    pub height: u32,
    pub max_color: u32,

    pub colors: Vec<u32>,
}

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
}

#[cfg(test)]
mod tests_ppm {
    use super::*;

    #[test]
    fn new() {
        let ppm = PPM::new(16, 9, 255, vec![0u32; 144]);
        assert_eq!("P3", ppm.version);
        assert_eq!(16, ppm.width);
        assert_eq!(9, ppm.height);
        assert_eq!(255, ppm.max_color);
        assert_eq!(144, ppm.colors.len());
        assert!(ppm.colors.iter().all(|&c| c == 0u32));
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
        PPM::new(
            canvas.width as u32,
            canvas.height as u32,
            255,
            canvas
                .raw_buffer
                .iter()
                .flat_map(|c| [c.r(), c.g(), c.b()])
                .map(|c| (c * 255.0) as u32)
                .collect(),
        )
    }
}
