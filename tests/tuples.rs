use rusty_ray_tracer::core3d::tuples::Coord3D;
use rusty_ray_tracer::core3d::tuples::Tuple;

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

// // Steps are defined with `given`, `when` and `then` attributes.
// #[given(regex = r"^([^\s]) ← tuple\(([\d\.-]+), ([\d\.-]+), ([\d\.-]+), ([\d\.-]+)\)$")]
// async fn a_tuple(world: &mut TuplesWorld, x: f32, y: f32, z: f32, w: f32) {
//     world.tuple = [x, y, z, w];
// }
// Steps are defined with `given`, `when` and `then` attributes.
#[given(expr = "{word} ← tuple\\({float}, {float}, {float}, {float})")]
async fn a_tuple(world: &mut TuplesWorld, _name: String, x: f32, y: f32, z: f32, w: f32) {
    world.tuple = [x, y, z, w];
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

#[then(expr = "a is a point")]
async fn is_a_point(world: &mut TuplesWorld) {
    assert!(world.tuple.is_point());
}

#[then(expr = "a is not a point")]
async fn is_not_a_point(world: &mut TuplesWorld) {
    assert!(!world.tuple.is_point());
}

#[then(regex = r"^a is a vector$")]
async fn is_a_vector(world: &mut TuplesWorld) {
    assert!(world.tuple.is_vector());
}

#[then(regex = r"^a is not a vector$")]
async fn is_not_a_vector(world: &mut TuplesWorld) {
    assert!(!world.tuple.is_vector());
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(TuplesWorld::run("tests/features/tuples.feature"));
}
