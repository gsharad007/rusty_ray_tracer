use crate::core3d::array_base::ArrayBase;

/// # 3D Coordinates
/// ## Left Handed Coordinates (default setup here)
/// With the y axis pointing up, and the x axis pointing to the right, the z axis can be defined to point away from
/// you.
/// ## Right Handed Coordinates
/// With the y axis pointing up, and the x axis pointing to the right, the z axis can be defined to point toward you.
/// todo! Matrix maybe a better name for this
pub trait Coordinates4: ArrayBase<Item = f32> + Sized
where
    <Self as ArrayBase>::Item: PartialEq + Sized,
{
    /// Gets the specified dimension from the tuple
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use rusty_ray_tracer::core3d::tuple::Tuple;
    /// let tuple = Tuple::new(0.0, 1.0, 2.0, 3.0);
    /// assert_eq!(0.0, tuple.get_at(0));
    /// assert_eq!(1.0, tuple.get_at(1));
    /// assert_eq!(2.0, tuple.get_at(2));
    /// assert_eq!(3.0, tuple.get_at(3));
    /// ```
    #[must_use]
    fn get_at(&self, dim: usize) -> Self::Item {
        self.get_array_ref()[dim]
    }

    /// Gets the first dimension (right) from the coordinate system
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use rusty_ray_tracer::core3d::tuple::Tuple;
    /// let tuple = Tuple::new(1.23, 4.56, 7.89, 10.11);
    /// assert_eq!(1.23, tuple.x());
    /// assert_eq!(4.56, tuple.y());
    /// assert_eq!(7.89, tuple.z());
    /// assert_eq!(10.11, tuple.w());
    /// ```
    #[must_use]
    fn x(&self) -> Self::Item {
        self.get_at(0)
    }

    /// Gets the second dimension (up) from the coordinate system
    #[must_use]
    fn y(&self) -> Self::Item {
        self.get_at(1)
    }

    /// Gets the third dimension (away) from the coordinate system
    #[must_use]
    fn z(&self) -> Self::Item {
        self.get_at(2)
    }

    /// Gets the fourth element `w` (usually 1.0 for absolute points, 0.0 for vectors) which helps with matrix
    /// multiplication
    #[must_use]
    fn w(&self) -> Self::Item {
        self.get_at(3)
    }

    /// Checks if the tuple is a point.
    ///
    /// Returns `true` if is a point. `false` otherwise
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// let tuple = Tuple::new(1.23, 4.56, 7.89, 1.0);
    /// assert!(tuple.is_point() == true);
    /// ```
    #[must_use]
    fn is_point(&self) -> bool {
        self.w() == 1.0
    }

    /// Checks if the tuple is a vector.
    ///
    /// Returns `true` if is a vector. `false` otherwise
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    /// # use crate::rusty_ray_tracer::core3d::tuple::Tuple;
    /// let tuple = Tuple::new(1.23, 4.56, 7.89, 0.0);
    /// assert!(tuple.is_vector() == true);
    /// ```
    #[must_use]
    fn is_vector(&self) -> bool {
        self.w() == 0.0
    }

    /// Debug asserts to make sure the tuple is valid
    fn is_valid(&self) -> bool {
        debug_assert!(self.is_point() || self.is_vector());
        self.is_point() || self.is_vector()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[derive(Clone)]
    pub struct Tuple {
        coords: [f32; 4],
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

    #[test]
    fn assign_array() {
        let tuple = Tuple {
            coords: [0.0, 1.0, 2.0, 3.0],
        };
        assert_eq!(0.0, tuple.get_at(0));
        assert_eq!(1.0, tuple.get_at(1));
        assert_eq!(2.0, tuple.get_at(2));
        assert_eq!(3.0, tuple.get_at(3));

        panic::catch_unwind(|| tuple.get_at(4)).unwrap_err();
        panic::catch_unwind(|| tuple.get_at(usize::MAX)).unwrap_err();

        let mut tuple_clone = tuple.clone();
        assert_eq!([0.0, 1.0, 2.0, 3.0], *tuple_clone.get_array_mut());
        assert_eq!([0.0, 1.0, 2.0, 3.0], tuple.get_array());
    }

    #[test]
    fn assign_default() {
        let tuple: Tuple = Tuple {
            coords: Default::default(),
        };
        assert_eq!(0.0, tuple.get_at(0));
        assert_eq!(0.0, tuple.get_at(1));
        assert_eq!(0.0, tuple.get_at(2));
        assert_eq!(0.0, tuple.get_at(3));
    }

    #[test]
    fn check_xyzw() {
        let tuple = Tuple {
            coords: [1.23, 4.56, 7.89, 10.11],
        };
        assert_eq!(1.23, tuple.x());
        assert_eq!(4.56, tuple.y());
        assert_eq!(7.89, tuple.z());
        assert_eq!(10.11, tuple.w());
    }

    #[test]
    fn is_point() {
        let tuple = Tuple {
            coords: [1.23, 4.56, 7.89, 1.0],
        };
        assert!(tuple.is_valid());
        assert!(tuple.is_point());
        assert!(!tuple.is_vector());
    }

    #[test]
    fn is_vector() {
        let tuple = Tuple {
            coords: [1.23, 4.56, 7.89, 0.0],
        };
        assert!(tuple.is_valid());
        assert!(tuple.is_vector());
        assert!(!tuple.is_point());
    }

    #[test]
    fn invalid_point_or_vector() {
        let tuple = Tuple {
            coords: [1.23, 4.56, 7.89, 1.01],
        };
        assert!(!tuple.is_point());
        assert!(!tuple.is_vector());
        panic::catch_unwind(|| tuple.is_valid()).unwrap_err();
    }
}
