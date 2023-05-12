use cucumber::{given, then, World};
use rusty_ray_tracer::core3d::{
    matrix::{Invert, Matrix44f32},
    matrix_scaling::Scaling,
    matrix_transforms::Transform,
    matrix_translations::Translation,
    point::Point,
    vector::Vector,
};

mod captures;
use captures::CapturePoint;
use captures::CaptureVector;

#[derive(World, Default, Debug)]
pub struct TheWorld {
    transform: Matrix44f32,
    inv: Matrix44f32,
    p: Point,
    v: Vector,
}
impl TheWorld {}
// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(TheWorld::run("tests/features/transformations.feature"));
}

#[given(expr = r"transform ← translation\({float}, {float}, {float}\)")]
fn transform_translation(world: &mut TheWorld, x: f32, y: f32, z: f32) {
    world.transform = Matrix44f32::translation(x, y, z);
}

#[given(expr = r"transform ← scaling\({float}, {float}, {float}\)")]
fn transform_scaling(world: &mut TheWorld, x: f32, y: f32, z: f32) {
    world.transform = Matrix44f32::scaling(x, y, z);
}

#[given(expr = r"p ← {point}")]
fn p_point(world: &mut TheWorld, point: CapturePoint) {
    world.p = *point;
}

#[given(expr = r"v ← {vector}")]
fn v_vector(world: &mut TheWorld, vector: CaptureVector) {
    world.v = *vector;
}

#[given(expr = r"inv ← inverse\(transform\)")]
fn inv_inverse_transform(world: &mut TheWorld) {
    world.inv = world.transform.inverse().unwrap();
}

#[then(expr = "transform * p = {point}")]
fn transform_p_eq_point(world: &mut TheWorld, point: CapturePoint) {
    let result = world.transform.transform(world.p);
    assert_eq!(result, *point);
}

#[then(expr = r"inv * p = {point}")]
fn inv_p_eq_point(world: &mut TheWorld, point: CapturePoint) {
    let result = world.inv.transform(world.p);
    assert_eq!(result, *point);
}

#[then(expr = r"inv * v = {vector}")]
fn inv_p_eq_vector(world: &mut TheWorld, vector: CaptureVector) {
    let result = world.inv.transform(world.v);
    assert_eq!(result, *vector);
}

#[then(expr = r"transform * v = v")]
fn transform_v_eq_v(world: &mut TheWorld) {
    let result = world.transform.transform(world.v);
    assert_eq!(result, world.v);
}

#[then(expr = r"transform * v = {vector}")]
fn transform_v_eq_vector(world: &mut TheWorld, vector: CaptureVector) {
    let result = world.transform.transform(world.v);
    assert_eq!(result, *vector);
}
