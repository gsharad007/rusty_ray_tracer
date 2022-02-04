use crate::core3d::tuples::Tuple;

/// A Point in 3D (x,y,z) space is a 4 unit set (x,y,z,w) with the `w` value being 1.0 to allow translations from matrices
pub type Point = Tuple;

/// Creates a new Point
///
/// ### Example
/// ```
/// use crate::rusty_ray_tracer::core3d::tuples::Tuple;
/// use crate::rusty_ray_tracer::core3d::point::new_point;
/// use crate::rusty_ray_tracer::core3d::tuples::Coord3D;
///
/// let point = new_point(1.0, 2.0, 3.0);
/// assert_eq!(1.0, point.x());
/// assert_eq!(2.0, point.y());
/// assert_eq!(3.0, point.z());
/// assert_eq!(1.0, point.w());
/// assert!(point.is_point() == true);
/// assert!(point.is_vector() == false);
/// ```
#[must_use]
pub fn new_point(x: f32, y: f32, z: f32) -> Point {
    [x, y, z, 1.0]
}

#[cfg(test)]
mod tests_point {
    use super::*;
    use crate::core3d::tuples::Coord3D;

    #[test]
    fn assign_array() {
        let point: Point = [4.0, 3.0, 2.0, 1.0];
        assert_eq!(4.0, point[0]);
        assert_eq!(3.0, point[1]);
        assert_eq!(2.0, point[2]);
        assert_eq!(1.0, point[3]);
        assert!(point.is_point() == true);
        assert!(point.is_vector() == false);
    }

    #[test]
    fn create_new() {
        let point = new_point(1.0, 2.0, 3.0);
        assert_eq!(1.0, point.x());
        assert_eq!(2.0, point.y());
        assert_eq!(3.0, point.z());
        assert_eq!(1.0, point.w());
        assert!(point.is_point() == true);
        assert!(point.is_vector() == false);
    }
}
