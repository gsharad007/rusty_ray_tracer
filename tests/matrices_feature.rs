use std::collections::HashMap;

use cucumber::{gherkin::Step, given, then, World};
use rusty_ray_tracer::core3d::{array_base::ArrayBase, matrix44f32::Matrix44f32, tuple::Tuple};

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

#[given(expr = "the following 4x4 matrix M:")]
fn the_following_4x4_matrix_m(world: &mut TheWorld, step: &Step) {
    let table = parse_step_to_table(step);

    table
        .iter()
        .enumerate()
        .for_each(|(i, r)| world.m.rows[i] = Tuple::new(r[0], r[1], r[2], r[3]));
}

#[then(expr = "M[{int},{int}] = {float}")]
fn m_x_f(world: &mut TheWorld, r: usize, c: usize, result: f32) {
    assert_eq!(world.m.rows[r].get_array_ref()[c], result);
}

#[given(expr = "the following 2x2 matrix M:")]
fn the_following_2x2_matrix_m(world: &mut TheWorld, step: &Step) {
    let table = parse_step_to_table(step);

    table
        .iter()
        .enumerate()
        .for_each(|(i, r)| world.m.rows[i] = Tuple::new(r[0], r[1], 0.0, 0.0));
}

#[given(expr = "the following 3x3 matrix M:")]
fn the_following_3x3_matrix_m(world: &mut TheWorld, step: &Step) {
    let table = parse_step_to_table(step);

    table
        .iter()
        .enumerate()
        .for_each(|(i, r)| world.m.rows[i] = Tuple::new(r[0], r[1], r[2], 0.0));
}

#[given(expr = "the following matrix {word}:")]
fn the_following_matrix_a(world: &mut TheWorld, name: String, step: &Step) {
    let table = parse_step_to_table(step);
    let a = world.get_matrix(&name);

    table
        .iter()
        .enumerate()
        .for_each(|(i, r)| a.rows[i] = Tuple::new(r[0], r[1], r[2], r[3]));
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
