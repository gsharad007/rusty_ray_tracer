use crate::core3d::array_base::ArrayBase;

/// # 3D Coordinates
/// ## Left Handed Coordinates (default setup here)
/// With the y axis pointing up, and the x axis pointing to the right, the z axis can be defined to point away from
/// you.
/// ## Right Handed Coordinates
/// With the y axis pointing up, and the x axis pointing to the right, the z axis can be defined to point toward you.
/// todo! Matrix maybe a better name for this
pub trait ColorRGB: ArrayBase<Item = f32> + Sized
where
    <Self as ArrayBase>::Item: PartialEq + Sized,
{
    /// Gets the specified dimension from the color
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::color_rgb::ColorRGB;
    /// # use rusty_ray_tracer::core3d::color::Color;
    /// let color = Color::new(0.0, 1.0, 2.0);
    /// assert_eq!(0.0, color.get_at(0));
    /// assert_eq!(1.0, color.get_at(1));
    /// assert_eq!(2.0, color.get_at(2));
    /// assert_eq!(1.0, color.get_at(3));
    /// ```
    #[must_use]
    fn get_at(&self, dim: usize) -> Self::Item {
        self.get_array_ref()[dim]
    }

    /// Gets the red color component
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::color_rgb::ColorRGB;
    /// # use rusty_ray_tracer::core3d::color::Color;
    /// let color = Color::new(1.23, 4.56, 7.89);
    /// assert_eq!(1.23, color.r());
    /// assert_eq!(4.56, color.g());
    /// assert_eq!(7.89, color.b());
    /// ```
    #[must_use]
    fn r(&self) -> Self::Item {
        self.get_at(0)
    }

    /// Gets the green color component
    #[must_use]
    fn g(&self) -> Self::Item {
        self.get_at(1)
    }

    /// Gets the blue color component
    #[must_use]
    fn b(&self) -> Self::Item {
        self.get_at(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[derive(Clone)]
    pub struct Color {
        tuple: [f32; 4],
    }
    impl ArrayBase for Color {
        type Item = f32;
        // type SizedArray = [f32; 4];

        fn get_array(self) -> [f32; 4] {
            self.tuple
        }

        fn get_array_ref(&self) -> &[f32; 4] {
            &self.tuple
        }

        fn get_array_mut(&mut self) -> &mut [f32; 4] {
            &mut self.tuple
        }
    }
    impl ColorRGB for Color {}

    #[test]
    fn assign_array() {
        let color = Color {
            tuple: [0.0, 1.0, 2.0, 3.0],
        };
        assert_eq!(0.0, color.get_at(0));
        assert_eq!(1.0, color.get_at(1));
        assert_eq!(2.0, color.get_at(2));
        assert_eq!(3.0, color.get_at(3));

        panic::catch_unwind(|| color.get_at(4)).unwrap_err();
        panic::catch_unwind(|| color.get_at(usize::MAX)).unwrap_err();

        let mut color_clone = color.clone();
        assert_eq!([0.0, 1.0, 2.0, 3.0], *color_clone.get_array_mut());
        assert_eq!([0.0, 1.0, 2.0, 3.0], color.get_array());
    }

    #[test]
    fn assign_default() {
        let color: Color = Color {
            tuple: Default::default(),
        };
        assert_eq!(0.0, color.get_at(0));
        assert_eq!(0.0, color.get_at(1));
        assert_eq!(0.0, color.get_at(2));
        assert_eq!(0.0, color.get_at(3));
    }

    #[test]
    fn check_rgba() {
        let color = Color {
            tuple: [1.23, 4.56, 7.89, 10.11],
        };
        assert_eq!(1.23, color.r());
        assert_eq!(4.56, color.g());
        assert_eq!(7.89, color.b());
    }
}
