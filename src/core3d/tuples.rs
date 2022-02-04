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
