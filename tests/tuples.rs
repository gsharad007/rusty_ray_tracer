use derive_more::Deref;
use rusty_ray_tracer::core3d::coordinates4::Coordinates4;
use rusty_ray_tracer::core3d::point::*;
use rusty_ray_tracer::core3d::tuple::*;
use rusty_ray_tracer::core3d::vector::*;

use std::collections::HashMap;
use std::convert::Infallible;
use std::num::ParseFloatError;
use std::str::FromStr;

use async_trait::async_trait;
use cucumber::{given, then, Parameter, World, WorldInit};

// `World` is your shared, likely mutable state.
#[derive(Debug, WorldInit, Default)]
pub struct TuplesWorld {
    a: Tuple,
    a1: Tuple,
    a2: Tuple,
    p: Point,
    p1: Point,
    p2: Point,
    vectors: HashMap<String, Vector>,
}
impl TuplesWorld {
    fn get_tuple(&mut self, name: String) -> &mut Tuple {
        match name.as_str() {
            "a1" => &mut self.a1,
            "a2" => &mut self.a2,
            _ => &mut self.a,
        }
    }
    fn get_point(&mut self, name: String) -> &mut Point {
        match name.as_str() {
            "p1" => &mut self.p1,
            "p2" => &mut self.p2,
            _ => &mut self.p,
        }
    }
    fn get_vector(&mut self, name: String) -> &mut Vector {
        self.vectors.entry(name).or_insert(Vector::default())
    }
    fn get_any_as_tuple(&self, name: String) -> Tuple {
        match name.as_str() {
            "a" => self.a,
            "a1" => self.a1,
            "a2" => self.a2,
            "p" => self.p.into(),
            "p1" => self.p1.into(),
            "p2" => self.p2.into(),
            _ => (*self.vectors.get(&name).unwrap()).into(),
        }
    }
}

// `World` needs to be implemented, so Cucumber knows how to construct it
// for each scenario.
#[async_trait(?Send)]
impl World for TuplesWorld {
    // We do require some error type.
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self::default())
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
// fn a_tuple(world: &mut TuplesWorld, x: f32, y: f32, z: f32, w: f32) {
//     world.tuple = [x, y, z, w];
// }
#[given(expr = r"{word} ← {tuple}")]
fn a_tuple(world: &mut TuplesWorld, name: String, tuple: CaptureTuple) {
    let world_tuple = world.get_tuple(name);
    *world_tuple = *tuple;
}

#[given(expr = r"{word} ← {point}")]
fn a_point(world: &mut TuplesWorld, name: String, point: CapturePoint) {
    let world_point = world.get_point(name);
    *world_point = *point;
}

#[given(expr = r"{word} ← {vector}")]
fn a_vector(world: &mut TuplesWorld, name: String, vector: CaptureVector) {
    let world_vector = world.get_vector(name);
    *world_vector = *vector;
}

#[then(regex = r"^([^\s])\.([xyzw]) = ([\d\.-]+)$")]
fn dim_equal(world: &mut TuplesWorld, name: String, dim: String, value: f32) {
    let world_tuple = world.get_any_as_tuple(name);
    match dim.as_str() {
        "x" => assert_eq!(value, world_tuple.x()),
        "y" => assert_eq!(value, world_tuple.y()),
        "z" => assert_eq!(value, world_tuple.z()),
        "w" => assert_eq!(value, world_tuple.w()),
        _ => unreachable!(),
    };
}

#[then(expr = r"{word} = {tuple}")]
fn equal_to_tuple(world: &mut TuplesWorld, name: String, tuple: CaptureTuple) {
    let world_tuple = world.get_any_as_tuple(name);
    assert_eq!(world_tuple, *tuple);
}

#[then(expr = r"{word} is a point")]
fn is_a_point(world: &mut TuplesWorld, name: String) {
    let world_tuple = world.get_any_as_tuple(name);
    assert!(world_tuple.is_point() == true);
}

#[then(expr = r"{word} is not a point")]
fn is_not_a_point(world: &mut TuplesWorld, name: String) {
    let world_tuple = world.get_any_as_tuple(name);
    assert!(world_tuple.is_point() == false);
}

#[then(expr = r"{word} is a vector")]
fn is_a_vector(world: &mut TuplesWorld, name: String) {
    let world_tuple = world.get_any_as_tuple(name);
    assert!(world_tuple.is_vector() == true);
}

#[then(expr = r"{word} is not a vector")]
fn is_not_a_vector(world: &mut TuplesWorld, name: String) {
    let world_tuple = world.get_any_as_tuple(name);
    assert!(world_tuple.is_vector() == false);
}

#[then(expr = r"a1 + a2 = {tuple}")]
fn a1_add_a2_eq_tuple(world: &mut TuplesWorld, tuple: CaptureTuple) {
    let result = world.a1 + world.a2;
    assert_eq!(result, *tuple);
}

#[then(expr = r"p1 - p2 = {vector}")]
fn p1_sub_p2_eq_vector(world: &mut TuplesWorld, vector: CaptureVector) {
    let result = world.p1 - world.p2;
    assert_eq!(result, *vector);
}

#[then(expr = r"p - v = {point}")]
fn p_sub_v_eq_vector(world: &mut TuplesWorld, point: CapturePoint) {
    let result = world.p - *world.get_vector("v".to_string());
    assert_eq!(result, *point);
}

#[then(expr = r"v1 - v2 = {vector}")]
fn v1_sub_v2_eq_vector(world: &mut TuplesWorld, vector: CaptureVector) {
    let result = *world.get_vector("v1".to_string()) - *world.get_vector("v2".to_string());
    assert_eq!(result, *vector);
}

#[then(expr = r"zero - v = {vector}")]
fn zero_sub_v_eq_vector(world: &mut TuplesWorld, vector: CaptureVector) {
    let result = *world.get_vector("zero".to_string()) - *world.get_vector("v".to_string());
    assert_eq!(result, *vector);
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(TuplesWorld::run("tests/features/tuples.feature"));
}
