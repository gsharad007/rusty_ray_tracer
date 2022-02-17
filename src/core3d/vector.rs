use super::{array_base::ArrayBase, coordinates4::Coordinates4};
use crate::core3d::tuples::Tuple;

/// A Vector in 3D (x,y,z) space is a 4 unit (x,y,z,w) set with the `w` value being 0.0 to ignore translations from matrices

#[derive(Copy, Clone, Default, Debug)]
pub struct Vector {
    pub coords: [f32; 4],
}
impl Vector {
    /// Creates a new Vector
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::tuples::*;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// # use crate::rusty_ray_tracer::core3d::coordinates4::Coordinates4;
    ///
    /// let vector = Vector::new(1.0, 2.0, 3.0);
    /// assert_eq!(1.0, vector.x());
    /// assert_eq!(2.0, vector.y());
    /// assert_eq!(3.0, vector.z());
    /// assert_eq!(0.0, vector.w());
    /// assert!(vector.is_vector() == true);
    /// assert!(vector.is_point() == false);
    /// assert!(vector.is_valid());
    /// ```
    #[must_use]
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector {
            coords: [x, y, z, 0.0],
        }
    }
}

impl From<[f32; 3]> for Vector {
    /// Creates a new vector from an array of scaler values
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::rusty_ray_tracer::core3d::tuples::Tuple;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let vector = Vector::from([1.0, 2.0, 3.0]);
    /// assert_eq!([1.0, 2.0, 3.0, 0.0], vector.coords);
    /// ```
    fn from(arr: [f32; 3]) -> Self {
        Vector::new(arr[0], arr[1], arr[2])
    }
}

impl From<Tuple> for Vector {
    /// Creates a new Vector from a Tuple
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::tuples::Tuple;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let vector = Vector::from(Tuple::from([1.0, 2.0, 3.0, 0.0]));
    /// assert_eq!([1.0, 2.0, 3.0, 0.0], vector.coords);
    /// ```
    ///
    /// ```
    /// # use std::panic;
    /// # use crate::rusty_ray_tracer::core3d::tuples::Tuple;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let tuple = Tuple::from([1.0, 2.0, 3.0, 4.0]);
    /// assert!(panic::catch_unwind(|| Vector::from(tuple)).is_err());
    /// ```
    fn from(tuple: Tuple) -> Self {
        debug_assert!(tuple.is_vector());
        Vector::new(tuple.x(), tuple.y(), tuple.z())
    }
}

impl From<Vector> for Tuple {
    /// Creates a new Tuple from a Vector
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::tuples::Tuple;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let vector = Vector::from(Tuple::from([1.0, 2.0, 3.0, 0.0]));
    /// assert_eq!([1.0, 2.0, 3.0, 0.0], vector.coords);
    /// ```
    ///
    /// ```
    /// # use std::panic;
    /// # use crate::rusty_ray_tracer::core3d::tuples::Tuple;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let tuple = Tuple::from([1.0, 2.0, 3.0, 4.0]);
    /// assert!(panic::catch_unwind(|| Vector::from(tuple)).is_err());
    /// ```
    fn from(vector: Vector) -> Self {
        debug_assert!(vector.is_vector());
        Tuple::from(vector.coords)
    }
}

impl ArrayBase for Vector {
    type Item = f32;
    // type SizedArray = [f32; 4];

    /// Returns base array consuming
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let vector = Vector::new(1.0, 2.0, 3.0);
    /// assert_eq!([1.0, 2.0, 3.0, 0.0], vector.get_array());
    /// ```
    fn get_array(self) -> [f32; 4] {
        self.coords
    }

    /// Returns base array reference
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let vector = Vector::new(1.0, 2.0, 3.0);
    /// assert_eq!([1.0, 2.0, 3.0, 0.0], *vector.get_array_ref());
    /// ```
    fn get_array_ref(&self) -> &[f32; 4] {
        &self.coords
    }

    /// Returns a mutable base array reference
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::vector::Vector;
    /// let mut vector = Vector::new(1.0, 2.0, 3.0);
    /// assert_eq!([1.0, 2.0, 3.0, 0.0], *vector.get_array_mut());
    /// vector.get_array_mut()[0] += 10.0;
    /// vector.get_array_mut()[1] += 10.0;
    /// vector.get_array_mut()[2] += 10.0;
    /// vector.get_array_mut()[3] += 10.0;
    /// assert_eq!([11.0, 12.0, 13.0, 10.0], *vector.get_array_mut());
    /// ```
    fn get_array_mut(&mut self) -> &mut [f32; 4] {
        &mut self.coords
    }
}
impl Coordinates4 for Vector {}

#[cfg(test)]
mod tests_vector {
    use super::*;
    use crate::core3d::coordinates4::Coordinates4;

    #[test]
    fn assign_array() {
        let vector: Vector = Tuple::from([3.0, 2.0, 1.0, 0.0]).into();
        assert_eq!(3.0, vector.x());
        assert_eq!(2.0, vector.y());
        assert_eq!(1.0, vector.z());
        assert_eq!(0.0, vector.w());
        assert!(vector.is_vector() == true);
        assert!(vector.is_point() == false);
        assert!(vector.is_valid());
    }

    #[test]
    fn create_new() {
        let vector = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(1.0, vector.x());
        assert_eq!(2.0, vector.y());
        assert_eq!(3.0, vector.z());
        assert_eq!(0.0, vector.w());
        assert!(vector.is_vector() == true);
        assert!(vector.is_point() == false);
        assert!(vector.is_valid());
    }
}
