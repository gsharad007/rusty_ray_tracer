use std::collections::HashMap;

use cucumber::{gherkin::Step, given, then, World};
use rusty_ray_tracer::core3d::matrix::Matrix44f32;

#[derive(World, Default, Debug)]
pub struct TheWorld {
    m: Matrix44f32,
    matrices: HashMap<String, Matrix44f32>,
}
impl TheWorld {
    fn get_matrix(&mut self, name: &str) -> &mut Matrix44f32 {
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

#[given(expr = "the following 4x4 matrix M:")]
fn the_following_4x4_matrix_m(world: &mut TheWorld, step: &Step) {
    world.m = parse_step_table_for_matrix(step);
}

#[then(expr = "M[{int},{int}] = {float}")]
fn m_x_f(world: &mut TheWorld, r: usize, c: usize, result: f32) {
    assert_eq!(world.m.matrix[r][c], result);
}

#[given(expr = "the following 2x2 matrix M:")]
fn the_following_2x2_matrix_m(world: &mut TheWorld, step: &Step) {
    world.m = parse_step_table_for_matrix(step);
}

#[given(expr = "the following 3x3 matrix M:")]
fn the_following_3x3_matrix_m(world: &mut TheWorld, step: &Step) {
    world.m = parse_step_table_for_matrix(step);
}

#[given(expr = "the following matrix {word}:")]
fn the_following_matrix_a(world: &mut TheWorld, name: String, step: &Step) {
    *world.get_matrix(&name) = parse_step_table_for_matrix(step);
}

#[then(expr = "A = B")]
fn a_equal_b(world: &mut TheWorld) {
    let a = *world.get_matrix("A");
    let b = *world.get_matrix("B");

    assert_eq!(a, b);
}

#[then(expr = "A != B")]
fn a_not_equal_b(world: &mut TheWorld) {
    let a = *world.get_matrix("A");
    let b = *world.get_matrix("B");

    assert_ne!(a, b);
}

#[then(expr = "A * B is the following 4x4 matrix:")]
fn a_mul_b_is_the_following_x_matrix(world: &mut TheWorld, step: &Step) {
    let expected = parse_step_table_for_matrix(step);

    let a = *world.get_matrix("A");
    let b = *world.get_matrix("B");

    let result = a * b;
    assert_eq!(result, expected);
}
