use cucumber::{gherkin::Step, given, then, World};
use rusty_ray_tracer::core3d::{array_base::ArrayBase, matrix44f32::Matrix44f32, tuple::Tuple};

#[derive(World, Default, Debug)]
pub struct TheWorld {
    pub m: Matrix44f32,
}

#[given(expr = "the following 4x4 matrix M:")]
fn the_following_x_matrix_m(world: &mut TheWorld, step: &Step) {
    let table = step
        .table
        .as_ref()
        .unwrap()
        .rows
        .iter()
        .map(|r| {
            r.iter()
                .map(|i| i.parse::<f32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    table
        .iter()
        .enumerate()
        .for_each(|(i, r)| world.m.rows[i] = Tuple::new(r[0], r[1], r[2], r[3]));
}

#[then(expr = "M[{int},{int}] = {float}")]
fn m_x_f(world: &mut TheWorld, r: usize, c: usize, result: f32) {
    assert_eq!(world.m.rows[r].get_array_ref()[c], result);
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(TheWorld::run("tests/features/matrices.feature"));
}
