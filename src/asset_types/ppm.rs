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
    const fn get_stride(&self) -> usize {
        (self.width * COMPONENTS_PER_COLOR) as usize
    }

    #[must_use]
    const fn is_color_index_new_line(&self, i: usize) -> bool {
        // assert!(i < self.colors.len());  // len() is non-const
        let stride = self.get_stride();
        i % stride == 0
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

mod display_helpers {
    use super::PPM;

    fn get_last_line_length(lines: &String) -> usize {
        lines.len() - lines.rfind('\n').map_or(0, |i| i + 1)
    }

    #[cfg(test)]
    mod tests_get_last_line_length {
        use super::*;

        #[test]
        fn test() {
            assert_eq!(0, get_last_line_length(&"".to_string()));
            assert_eq!(6, get_last_line_length(&"FooBar".to_string()));
            assert_eq!(3, get_last_line_length(&"Foo\nBar".to_string()));
            assert_eq!(0, get_last_line_length(&"Foo\nBar\n".to_string()));
            assert_eq!(
                23,
                get_last_line_length(
                    &"P3\n4 3\n255\n0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0".to_string()
                )
            );
        }
    }

    const PPM_MAX_LINE_LENGTH: usize = 70;
    fn is_join_forming_long_line(lines: &String, append_size: usize) -> bool {
        let line_len = get_last_line_length(lines) + append_size;
        line_len >= PPM_MAX_LINE_LENGTH
    }

    #[cfg(test)]
    mod tests_is_join_forming_long_line {
        use super::*;

        #[test]
        fn test() {
            assert_eq!(
                false,
                is_join_forming_long_line(&"".to_string(), " 255".to_string().len())
            );
            assert_eq!(
                false,
                is_join_forming_long_line(&"FooBar".to_string(), " 255".to_string().len())
            );
            assert_eq!(
                false,
                is_join_forming_long_line(&"Foo\nBar".to_string(), " 255".to_string().len())
            );
            assert_eq!(
                false,
                is_join_forming_long_line(&"Foo\nBar\n".to_string(), " 255".to_string().len())
            );
            assert_eq!(
                false,
                is_join_forming_long_line(
                    &"\
P3
4 3
255
0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0"
                        .to_string(),
                    "255".to_string().len()
                )
            );
            assert_eq!(
                false,
                is_join_forming_long_line(
                    &"\
P3
8 2
255
255 255 255 255 255 255 255 255 255 255 255 255 255 255 255 255 255
255 255 255 255 255 255 255 
255 255 255 255 255 255 255 255 255 255 255 255 255 255 255 255 255"
                        .to_string(),
                    " 0".to_string().len()
                )
            );
            assert_eq!(
                true,
                is_join_forming_long_line(
                    &"\
P3
8 2
255
255 255 255 255 255 255 255 255 255 255 255 255 255 255 255 255 255
255 255 255 255 255 255 255 
255 255 255 255 255 255 255 255 255 255 255 255 255 255 255 255 255"
                        .to_string(),
                    " 16".to_string().len()
                )
            );
            assert_eq!(
                true,
                is_join_forming_long_line(
                    &"\
P3
8 2
255
255 255 255 255 255 255 255 255 255 255 255 255 255 255 255 255 255
255 255 255 255 255 255 255 
255 255 255 255 255 255 255 255 255 255 255 255 255 255 255 255 255"
                        .to_string(),
                    " 255".to_string().len()
                )
            );
        }
    }

    #[must_use]
    fn should_start_new_line(ppm: &PPM, i: usize, lines: &String, append_size: usize) -> bool {
        ppm.is_color_index_new_line(i) || is_join_forming_long_line(lines, append_size)
    }

    #[cfg(test)]
    mod tests_should_start_new_line {
        use super::*;

        #[test]
        fn start_new_line_for_new_row() {
            let ppm = PPM::new(4, 3, 255, vec![0u32; 144]);
            let mut acc = String::new();
            let mut i = 0;
            for _y in 0..3 {
                assert_eq!(true, should_start_new_line(&ppm, i, &acc, 4));
                acc += "\n255";
                i += 1;
                for _o in 1..4 * 3 {
                    assert_eq!(false, should_start_new_line(&ppm, i, &acc, 4));
                    acc += " 255";
                    i += 1;
                }
            }
            assert_eq!(4 * 3 * 3, i);
        }

        #[test]
        fn start_new_line_for_long_lines() {
            let ppm = PPM::new(8, 3, 255, vec![0u32; 144]);
            let mut acc = String::new();
            let mut i = 0;
            for _y in 0..3 {
                assert_eq!(true, should_start_new_line(&ppm, i, &acc, 4));
                acc += "\n255";
                i += 1;
                for o in 1..8 * 3 {
                    let new_line = o == 17;
                    assert_eq!(new_line, should_start_new_line(&ppm, i, &acc, 4));
                    if new_line {
                        acc += "\n255";
                    } else {
                        acc += " 255";
                    }
                    i += 1;
                }
            }
            assert_eq!(8 * 3 * 3, i);
        }
    }

    #[must_use]
    pub fn get_colors_string_prefix(
        ppm: &PPM,
        i: usize,
        acc: &String,
        append: &String,
    ) -> &'static str {
        if should_start_new_line(ppm, i, acc, append.len() + 1) {
            "\n"
        } else {
            " "
        }
    }

    #[cfg(test)]
    mod tests_get_colors_string_prefix {
        use super::*;

        #[test]
        fn get_newline_for_new_row() {
            let ppm = PPM::new(4, 3, 255, vec![0u32; 144]);
            let mut acc = String::new();
            let mut i = 0;
            for _y in 0..3 {
                assert_eq!(
                    "\n",
                    get_colors_string_prefix(&ppm, i, &acc, &"255".to_string())
                );
                acc += "\n255";
                i += 1;
                for _o in 1..4 * 3 {
                    assert_eq!(
                        " ",
                        get_colors_string_prefix(&ppm, i, &acc, &"255".to_string())
                    );
                    acc += " 255";
                    i += 1;
                }
            }
            assert_eq!(4 * 3 * 3, i);
        }

        #[test]
        fn get_newline_for_long_lines() {
            let ppm = PPM::new(8, 3, 255, vec![0u32; 144]);
            let mut acc = String::new();
            let mut i = 0;
            for _y in 0..3 {
                assert_eq!(
                    "\n",
                    get_colors_string_prefix(&ppm, i, &acc, &"255".to_string())
                );
                acc += "\n255";
                i += 1;
                for o in 1..8 * 3 {
                    let new_line = if o == 17 { "\n" } else { " " };
                    assert_eq!(
                        new_line,
                        get_colors_string_prefix(&ppm, i, &acc, &"255".to_string())
                    );
                    acc += &(new_line.to_owned() + "255");
                    i += 1;
                }
            }
            assert_eq!(8 * 3 * 3, i);
        }
    }
}

// #[display(fmt = "{}\n{} {}\n{}\n{:?}", version, width, height, max_color, colors)]
impl Display for PPM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let colors = self.colors.iter().map(|&c| c.to_string()).enumerate().fold(
            String::new(),
            |acc, (i, c)| {
                let prefix = display_helpers::get_colors_string_prefix(self, i, &acc, &c);
                acc + prefix + &c
            },
        );
        write!(
            f,
            "{}\n{} {}\n{}{}",
            self.version, self.width, self.height, self.max_color, colors
        )
    }
}

#[cfg(test)]
mod tests_display {
    use crate::core3d::color::{self, Color};

    use super::*;

    #[test]
    fn display_black() {
        let ppm = PPM::from(&Canvas::new(4, 3));
        let ppm_text = ppm.to_string();
        assert_eq!(
            "\
P3
4 3
255
0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0",
            ppm_text
        );
    }

    #[test]
    fn display_with_data() {
        let mut canvas = Canvas::new(4, 3);
        canvas.set_pixel_at(0, 0, color::Color::new(1.0, 0.0, 0.0));
        canvas.set_pixel_at(1, 1, color::Color::new(0.0, 1.0, 0.0));
        canvas.set_pixel_at(2, 2, color::Color::new(0.0, 0.0, 1.0));
        let ppm = PPM::from(&canvas);
        let ppm_text = ppm.to_string();
        assert_eq!(
            "\
P3
4 3
255
255 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 255 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 255 0 0 0",
            ppm_text
        );
    }

    #[test]
    fn display_with_oor_data() {
        let mut canvas = Canvas::new(4, 3);
        canvas.set_pixel_at(0, 0, color::Color::new(1.5, 0.0, 0.0));
        canvas.set_pixel_at(1, 1, color::Color::new(0.0, 0.5, 0.0));
        canvas.set_pixel_at(2, 2, color::Color::new(-0.5, 0.0, 1.0));
        let ppm = PPM::from(&canvas);
        let ppm_text = ppm.to_string();
        assert_eq!(
            "\
P3
4 3
255
255 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 255 0 0 0",
            ppm_text
        );
    }

    #[test]
    fn display_with_long_line_data() {
        let mut canvas = Canvas::new(16, 16);
        for d in 0..16 {
            canvas.set_pixel_at(
                d,
                d,
                Color::new(0.06666 * f32::from(d), 0.06666 * f32::from(15 - d), 0.0),
            );
        }
        let ppm = PPM::from(&canvas);
        let ppm_text = ppm.to_string();
        assert_eq!(
            "\
P3
16 16
255
0 255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 17 238 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 34 221 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 51 204 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 68 187 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 85 170 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 102 153 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 119 136 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 136 119 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 153 102 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 170 85 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 187
68 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 204 51 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 221 34 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 238 17 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 255 0 0",
            ppm_text
        );
    }
}
