use std::collections::HashMap;

use cucumber::{gherkin::Step, given, then, World};
use rusty_ray_tracer::core3d::matrix::Transpose;
use rusty_ray_tracer::core3d::{matrix::Matrix44f32, tuple::Tuple};

mod captures;
use crate::captures::CaptureTuple;

#[derive(World, Default, Debug)]
pub struct TheWorld {
    m: Matrix44f32,
    matrices: HashMap<String, Matrix44f32>,
    tuples: HashMap<String, Tuple>,
}
impl TheWorld {
    fn get_matrix(&mut self, name: &str) -> &mut Matrix44f32 {
        self.matrices
            .entry(name.to_string())
            .or_insert_with(Matrix44f32::default)
    }

    fn get_tuple(&mut self, name: &str) -> &mut Tuple {
        self.tuples
            .entry(name.to_string())
            .or_insert_with(Tuple::default)
    }
}
// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(TheWorld::run("tests/features/matrices.feature"));
}

fn parse_step_to_table(step: &Step) -> Vec<Vec<f32>> {
    step.table
        .as_ref()
        .unwrap()
        .rows
        .iter()
        .map(|r| {
            r.iter()
                .map(|i| i.parse::<f32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn parse_step_table_for_matrix(step: &Step) -> Matrix44f32 {
    let table = parse_step_to_table(step);

    Matrix44f32::from(table)
}

#[given(expr = r"the following 4x4 matrix M:")]
fn the_following_4x4_matrix_m(world: &mut TheWorld, step: &Step) {
    world.m = parse_step_table_for_matrix(step);
}

#[then(expr = r"M[{int},{int}] = {float}")]
fn m_x_f(world: &mut TheWorld, r: usize, c: usize, result: f32) {
    assert_eq!(world.m.matrix[r][c], result);
}

#[given(expr = r"the following 2x2 matrix M:")]
fn the_following_2x2_matrix_m(world: &mut TheWorld, step: &Step) {
    world.m = parse_step_table_for_matrix(step);
}

#[given(expr = r"the following 3x3 matrix M:")]
fn the_following_3x3_matrix_m(world: &mut TheWorld, step: &Step) {
    world.m = parse_step_table_for_matrix(step);
}

#[given(expr = r"the following matrix {word}:")]
fn the_following_matrix_a(world: &mut TheWorld, name: String, step: &Step) {
    *world.get_matrix(&name) = parse_step_table_for_matrix(step);
}

#[then(expr = r"A = B")]
fn a_equal_b(world: &mut TheWorld) {
    let a = *world.get_matrix("A");
    let b = *world.get_matrix("B");

    assert_eq!(a, b);
}

#[then(expr = r"A != B")]
fn a_not_equal_b(world: &mut TheWorld) {
    let a = *world.get_matrix("A");
    let b = *world.get_matrix("B");

    assert_ne!(a, b);
}

#[then(expr = r"A * B is the following 4x4 matrix:")]
fn a_mul_b_is_the_following_x_matrix(world: &mut TheWorld, step: &Step) {
    let expected = parse_step_table_for_matrix(step);

    let a = *world.get_matrix("A");
    let b = *world.get_matrix("B");

    let result = a * b;
    assert_eq!(result, expected);
}

#[given(expr = r"{word} ← {tuple}")]
fn b_tuple(world: &mut TheWorld, name: String, tuple: CaptureTuple) {
    *world.get_tuple(&name) = *tuple;
}

#[then(expr = r"A * b = {tuple}")]
fn a_b_x_d_tuple(world: &mut TheWorld, tuple: CaptureTuple) {
    let expected = *tuple;

    let a = *world.get_matrix("A");
    let b = *world.get_tuple("b");

    let result = a * b;
    assert_eq!(result, expected);
}

#[then(expr = r"A * identity_matrix = A")]
fn a_identity_matrix_x_d_a(world: &mut TheWorld) {
    let a = *world.get_matrix("A");
    let identity = Matrix44f32::identity();

    let result = a * identity;
    assert_eq!(result, a);
}

#[then(expr = r"identity_matrix * a = a")]
fn identity_matrix_a_x_d_a(world: &mut TheWorld) {
    let identity = Matrix44f32::identity();
    let a = *world.get_tuple("a");

    let result = identity * a;
    assert_eq!(result, a);
}

#[then(expr = r"transpose\(A) is the following matrix:")]
fn transpose_a_is_the_following_matrix(world: &mut TheWorld, step: &Step) {
    let expected = parse_step_table_for_matrix(step);

    let a = *world.get_matrix("A");

    let result = Matrix44f32::transpose(a);
    assert_eq!(result, expected);
}

#[given(expr = r"A ← transpose\(identity_matrix)")]
fn a_transpose_identity_matrix(world: &mut TheWorld) {
    *world.get_matrix("A") = Matrix44f32::transpose(Matrix44f32::identity());
}

#[then(expr = r"A = identity_matrix")]
fn a_x_d_identity_matrix(world: &mut TheWorld) {
    let a = *world.get_matrix("A");
    let identity = Matrix44f32::identity();

    assert_eq!(a, identity);
}
