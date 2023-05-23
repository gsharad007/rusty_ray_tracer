#![allow(clippy::needless_pass_by_value)]

use std::collections::HashMap;

use cucumber::{given, then, World};
use rusty_ray_tracer::core3d::{
    matrix::{Invert, Matrix44f32},
    matrix_rotations::Rotations,
    matrix_scaling::Scaling,
    matrix_shearing::Shearing,
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
    matrices: HashMap<String, Matrix44f32>,
    p: Point,
    v: Vector,
}

impl TheWorld {
    fn get_matrix(&self, name: &str) -> &Matrix44f32 {
        self.matrices.get(&name.to_string()).unwrap()
    }

    fn get_matrix_mut(&mut self, name: &str) -> &mut Matrix44f32 {
        self.matrices
            .entry(name.to_string())
            .or_insert_with(Matrix44f32::default)
    }
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(TheWorld::run("tests/features/transformations.feature"));
}

#[given(expr = r"transform ← translation\({float}, {float}, {float}\)")]
fn matrix_translation(world: &mut TheWorld, x: f32, y: f32, z: f32) {
    *world.get_matrix_mut("transform") = Matrix44f32::translation(x, y, z);
}

#[given(expr = r"transform ← scaling\({float}, {float}, {float}\)")]
fn matrix_scaling(world: &mut TheWorld, x: f32, y: f32, z: f32) {
    *world.get_matrix_mut("transform") = Matrix44f32::scaling(x, y, z);
}

#[given(expr = r"{word} ← rotation_{word}\(π \/ {float}\)")]
fn matrix_rotation_x(world: &mut TheWorld, name: String, axis: String, div: f32) {
    let angle = std::f32::consts::PI / div;
    *world.get_matrix_mut(&name) = match axis.as_str() {
        "x" => Matrix44f32::rotation_around_x_axis(angle),
        "y" => Matrix44f32::rotation_around_y_axis(angle),
        "z" => Matrix44f32::rotation_around_z_axis(angle),
        _ => unreachable!("Invalid axis"),
    };
}

#[given(expr = r"transform ← shearing\({float}, {float}, {float}, {float}, {float}, {float}\)")]
fn matrix_shearing(
    world: &mut TheWorld,
    x_y: f32,
    x_z: f32,
    y_x: f32,
    y_z: f32,
    z_x: f32,
    z_y: f32,
) {
    *world.get_matrix_mut("transform") = Matrix44f32::shearing(x_y, x_z, y_x, y_z, z_x, z_y);
}

#[given(expr = r"p ← {point}")]
fn p_point(world: &mut TheWorld, point: CapturePoint) {
    world.p = *point;
}

#[given(expr = r"v ← {vector}")]
fn v_vector(world: &mut TheWorld, vector: CaptureVector) {
    world.v = *vector;
}

#[given(expr = r"inv ← inverse\({word}\)")]
fn inv_inverse_transform(world: &mut TheWorld, name: String) {
    let matrix = world.get_matrix(&name);
    *world.get_matrix_mut("inv") = matrix.inverse().unwrap();
}

#[then(expr = r"{word} * p = {point}")]
fn matrix_transform_p_eq_point(world: &mut TheWorld, name: String, point: CapturePoint) {
    let matrix = world.get_matrix(&name);
    let result = matrix.transform(world.p);
    assert_eq!(result, *point);
}

#[then(expr = r"{word} * v = {vector}")]
fn matrix_transform_v_eq_vector(world: &mut TheWorld, name: String, vector: CaptureVector) {
    let matrix = world.get_matrix(&name);
    let result = matrix.transform(world.v);
    assert_eq!(result, *vector);
}

#[then(expr = r"transform * v = v")]
fn transform_v_eq_v(world: &mut TheWorld) {
    let matrix = world.get_matrix("transform");
    let result = matrix.transform(world.v);
    assert_eq!(result, world.v);
}
