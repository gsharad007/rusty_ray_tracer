use rusty_ray_tracer::core3d::tuples::*;

use std::convert::Infallible;

use async_trait::async_trait;
use cucumber::{given, then, World, WorldInit};

// `World` is your shared, likely mutable state.
#[derive(Debug, WorldInit)]
pub struct TuplesWorld {
    tuple: Tuple,
}

// `World` needs to be implemented, so Cucumber knows how to construct it
// for each scenario.
#[async_trait(?Send)]
impl World for TuplesWorld {
    // We do require some error type.
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            tuple: Default::default(),
        })
    }
}

// #[given(regex = r"^([^\s]) ← tuple\(([\d\.-]+), ([\d\.-]+), ([\d\.-]+), ([\d\.-]+)\)$")]
// async fn a_tuple(world: &mut TuplesWorld, x: f32, y: f32, z: f32, w: f32) {
//     world.tuple = [x, y, z, w];
// }
#[given(expr = r"{word} ← tuple\({float}, {float}, {float}, {float})")]
async fn a_tuple(world: &mut TuplesWorld, _name: String, x: f32, y: f32, z: f32, w: f32) {
    world.tuple = [x, y, z, w];
}

#[given(expr = r"{word} ← point\({float}, {float}, {float})")]
async fn a_point(world: &mut TuplesWorld, _name: String, x: f32, y: f32, z: f32) {
    world.tuple = new_point(x, y, z);
}

#[given(expr = r"{word} ← vector\({float}, {float}, {float})")]
async fn a_vector(world: &mut TuplesWorld, _name: String, x: f32, y: f32, z: f32) {
    world.tuple = new_vector(x, y, z);
}

#[then(regex = r"^([^\s])\.([xyzw]) = ([\d\.-]+)$")]
async fn dim_equal(world: &mut TuplesWorld, _name: String, dim: String, value: f32) {
    match dim.as_str() {
        "x" => assert!(world.tuple.x() == value),
        "y" => assert!(world.tuple.y() == value),
        "z" => assert!(world.tuple.z() == value),
        "w" => assert!(world.tuple.w() == value),
        _ => unreachable!(),
    };
}

#[then(expr = r"{word} = tuple\({float}, {float}, {float}, {float})")]
async fn equal_to_tuple(world: &mut TuplesWorld, _name: String, x: f32, y: f32, z: f32, w: f32) {
    assert_eq!(world.tuple, [x, y, z, w]);
}

#[then(expr = r"{word} is a point")]
async fn is_a_point(world: &mut TuplesWorld, _name: String) {
    assert!(world.tuple.is_point() == true);
}

#[then(expr = r"{word} is not a point")]
async fn is_not_a_point(world: &mut TuplesWorld, _name: String) {
    assert!(world.tuple.is_point() == false);
}

#[then(expr = r"{word} is a vector")]
async fn is_a_vector(world: &mut TuplesWorld, _name: String) {
    assert!(world.tuple.is_vector() == true);
}

#[then(expr = r"{word} is not a vector")]
async fn is_not_a_vector(world: &mut TuplesWorld, _name: String) {
    assert!(world.tuple.is_vector() == false);
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(TuplesWorld::run("tests/features/tuples.feature"));
}
