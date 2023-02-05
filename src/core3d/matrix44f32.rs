use super::tuple::Tuple;

#[derive(Default, Debug)]
pub struct Matrix44f32 {
    pub rows: [Tuple; 4],
}
