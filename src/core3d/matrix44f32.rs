use super::tuple::Tuple;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Matrix44f32 {
    pub rows: [Tuple; 4],
}
