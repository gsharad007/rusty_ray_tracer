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
        assert!(tuple.is_point());
        assert!(!tuple.is_vector());
    }

    #[test]
    fn is_vector() {
        let tuple: Tuple = [1.23, 4.56, 7.89, 0.0];
        assert!(tuple.is_vector());
        assert!(!tuple.is_point());
    }
}
