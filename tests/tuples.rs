use derive_more::Deref;
use rusty_ray_tracer::core3d::coordinates4::Coordinates4;
use rusty_ray_tracer::core3d::point::*;
use rusty_ray_tracer::core3d::tuples::*;
use rusty_ray_tracer::core3d::vector::*;

use std::convert::Infallible;
use std::num::ParseFloatError;
use std::str::FromStr;

use async_trait::async_trait;
use cucumber::{given, then, Parameter, World, WorldInit};

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

#[derive(Parameter, Deref)]
#[param(
    name = "tuple",
    regex = r"tuple\(\s*[\d\.-]+\s*,\s*[\d\.-]+\s*,\s*[\d\.-]+\s*,\s*[\d\.-]+\s*\)"
)]
struct CaptureTuple(Tuple);
impl FromStr for CaptureTuple {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<_> = s
            .to_lowercase()
            .strip_prefix("tuple")
            .expect("Tuple should start with tuple")
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .map(|ss| {
                ss.trim()
                    .parse::<f32>()
                    .expect("Parsing component f32 failed")
            })
            .collect();

        Ok(CaptureTuple {
            0: Tuple::new(coords[0], coords[1], coords[2], coords[3]),
        })
    }
}

#[derive(Parameter, Deref)]
#[param(
    name = "point",
    regex = r"point\(\s*[\d\.-]+\s*,\s*[\d\.-]+\s*,\s*[\d\.-]+\s*\)"
)]
struct CapturePoint(Point);
impl FromStr for CapturePoint {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<_> = s
            .to_lowercase()
            .strip_prefix("point")
            .expect("Point should start with point")
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .map(|ss| {
                ss.trim()
                    .parse::<f32>()
                    .expect("Parsing component f32 failed")
            })
            .collect();

        Ok(CapturePoint {
            0: Point::new(coords[0], coords[1], coords[2]),
        })
    }
}

#[derive(Parameter, Deref)]
#[param(
    name = "vector",
    regex = r"vector\(\s*[\d\.-]+\s*,\s*[\d\.-]+\s*,\s*[\d\.-]+\s*\)"
)]
struct CaptureVector(Vector);
impl FromStr for CaptureVector {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<_> = s
            .to_lowercase()
            .strip_prefix("vector")
            .expect("Vector should start with vector")
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .map(|ss| {
                ss.trim()
                    .parse::<f32>()
                    .expect("Parsing component f32 failed")
            })
            .collect();

        Ok(CaptureVector {
            0: Vector::new(coords[0], coords[1], coords[2]),
        })
    }
}

// #[given(regex = r"^([^\s]) ← tuple\(([\d\.-]+), ([\d\.-]+), ([\d\.-]+), ([\d\.-]+)\)$")]
// async fn a_tuple(world: &mut TuplesWorld, x: f32, y: f32, z: f32, w: f32) {
//     world.tuple = [x, y, z, w];
// }
#[given(expr = r"{word} ← {tuple}")]
async fn a_tuple(world: &mut TuplesWorld, _name: String, tuple: CaptureTuple) {
    world.tuple = *tuple;
}

#[given(expr = r"{word} ← {point}")]
async fn a_point(world: &mut TuplesWorld, _name: String, point: CapturePoint) {
    world.tuple = (*point).into();
}

#[given(expr = r"{word} ← {vector}")]
async fn a_vector(world: &mut TuplesWorld, _name: String, vector: CaptureVector) {
    world.tuple = (*vector).into();
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

#[then(expr = r"{word} = {tuple}")]
async fn equal_to_tuple(world: &mut TuplesWorld, _name: String, tuple: CaptureTuple) {
    assert_eq!(world.tuple, *tuple);
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
