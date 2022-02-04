use crate::core3d::tuples::Tuple;

/// A Vector in 3D (x,y,z) space is a 4 unit (x,y,z,w) set with the `w` value being 0.0 to ignore translations from matrices
pub type Vector = Tuple;

/// Creates a new Vector
///
/// ### Example
/// ```
/// use crate::rusty_ray_tracer::core3d::tuples::Tuple;
/// use crate::rusty_ray_tracer::core3d::vector::new_vector;
/// use crate::rusty_ray_tracer::core3d::tuples::Coord3D;
///
/// let vector = new_vector(1.0, 2.0, 3.0);
/// assert_eq!(1.0, vector.x());
/// assert_eq!(2.0, vector.y());
/// assert_eq!(3.0, vector.z());
/// assert_eq!(0.0, vector.w());
/// assert!(vector.is_vector() == true);
/// assert!(vector.is_point() == false);
/// ```
#[must_use]
pub fn new_vector(x: f32, y: f32, z: f32) -> Vector {
    [x, y, z, 0.0]
}

#[cfg(test)]
mod tests_vector {
    use super::*;
    use crate::core3d::tuples::Coord3D;

    #[test]
    fn assign_array() {
        let vector: Vector = [3.0, 2.0, 1.0, 0.0];
        assert_eq!(3.0, vector[0]);
        assert_eq!(2.0, vector[1]);
        assert_eq!(1.0, vector[2]);
        assert_eq!(0.0, vector[3]);
        assert!(vector.is_vector() == true);
        assert!(vector.is_point() == false);
    }

    #[test]
    fn create_new() {
        let vector = new_vector(1.0, 2.0, 3.0);
        assert_eq!(1.0, vector.x());
        assert_eq!(2.0, vector.y());
        assert_eq!(3.0, vector.z());
        assert_eq!(0.0, vector.w());
        assert!(vector.is_vector() == true);
        assert!(vector.is_point() == false);
    }
}
