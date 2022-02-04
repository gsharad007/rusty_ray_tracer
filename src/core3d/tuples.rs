pub type Tuple = [f32; 4];

pub trait Coord3D {
    #[must_use]
    fn x(&self) -> f32;

    #[must_use]
    fn y(&self) -> f32;

    #[must_use]
    fn z(&self) -> f32;

    #[must_use]
    fn w(&self) -> f32;

    #[must_use]
    fn is_point(&self) -> bool;

    #[must_use]
    fn is_vector(&self) -> bool;
}

impl Coord3D for Tuple {
    fn x(&self) -> f32 {
        self[0]
    }

    fn y(&self) -> f32 {
        self[1]
    }

    fn z(&self) -> f32 {
        self[2]
    }

    fn w(&self) -> f32 {
        self[3]
    }

    fn is_point(&self) -> bool {
        self.w() == 1.0
    }

    fn is_vector(&self) -> bool {
        self.w() == 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_array() {
        let tuple: Tuple = [0.0, 1.0, 2.0, 3.0];
        assert_eq!(0.0, tuple[0]);
        assert_eq!(1.0, tuple[1]);
        assert_eq!(2.0, tuple[2]);
        assert_eq!(3.0, tuple[3]);
    }

    #[test]
    fn assign_default() {
        let tuple: Tuple = Default::default();
        assert_eq!(0.0, tuple[0]);
        assert_eq!(0.0, tuple[1]);
        assert_eq!(0.0, tuple[2]);
        assert_eq!(0.0, tuple[3]);
    }

    #[test]
    fn check_xyzw() {
        let tuple: Tuple = [1.23, 4.56, 7.89, 10.11];
        assert_eq!(1.23, tuple.x());
        assert_eq!(4.56, tuple.y());
        assert_eq!(7.89, tuple.z());
        assert_eq!(10.11, tuple.w());
    }

    #[test]
    fn is_point() {
        let tuple: Tuple = [1.23, 4.56, 7.89, 1.0];
        assert!(tuple.is_point() == true);
        assert!(tuple.is_vector() == false);
    }

    #[test]
    fn is_vector() {
        let tuple: Tuple = [1.23, 4.56, 7.89, 0.0];
        assert!(tuple.is_vector() == true);
        assert!(tuple.is_point() == false);
    }
}

pub type PointF32 = Tuple;

// pub trait Point {
#[must_use]
pub fn new_point(x: f32, y: f32, z: f32) -> PointF32 {
    [x, y, z, 1.0]
}
// }
// impl Point for PointF32 {}

#[cfg(test)]
mod tests_point {
    use super::*;

    #[test]
    fn assign_array() {
        let point: PointF32 = [4.0, 3.0, 2.0, 1.0];
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

pub type VectorF32 = Tuple;

#[must_use]
pub fn new_vector(x: f32, y: f32, z: f32) -> VectorF32 {
    [x, y, z, 0.0]
}

#[cfg(test)]
mod tests_vector {
    use super::*;

    #[test]
    fn assign_array() {
        let vector: VectorF32 = [3.0, 2.0, 1.0, 0.0];
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
