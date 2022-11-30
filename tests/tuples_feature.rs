use derive_more::Deref;
use derive_more::FromStr;
use float_cmp::assert_approx_eq;
use rusty_ray_tracer::core3d::color::*;
use rusty_ray_tracer::core3d::color_rgb::ColorRGB;
use rusty_ray_tracer::core3d::coordinates4::Coordinates4;
use rusty_ray_tracer::core3d::point::*;
use rusty_ray_tracer::core3d::tuple::*;
use rusty_ray_tracer::core3d::vector::*;

use std::collections::HashMap;
use std::convert::Infallible;
use std::num::ParseFloatError;
use std::str::FromStr;

use async_trait::async_trait;
use cucumber::{given, then, when, Parameter, World, WorldInit};

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
    c: Color,
    c1: Color,
    c2: Color,
}
impl TuplesWorld {
    fn get_tuple(&mut self, name: &str) -> &mut Tuple {
        match name {
            "a1" => &mut self.a1,
            "a2" => &mut self.a2,
            _ => &mut self.a,
        }
    }
    fn get_point(&mut self, name: &str) -> &mut Point {
        match name {
            "p1" => &mut self.p1,
            "p2" => &mut self.p2,
            _ => &mut self.p,
        }
    }
    fn get_vector(&mut self, name: &str) -> &mut Vector {
        self.vectors
            .entry(name.to_string())
            .or_insert_with(Vector::default)
    }
    fn get_color(&mut self, name: &str) -> &mut Color {
        match name {
            "c1" => &mut self.c1,
            "c2" => &mut self.c2,
            _ => &mut self.c,
        }
    }
    fn get_any_as_tuple(&self, name: &str) -> Tuple {
        match name {
            "a" => self.a,
            "a1" => self.a1,
            "a2" => self.a2,
            "p" => self.p.into(),
            "p1" => self.p1.into(),
            "p2" => self.p2.into(),
            _ => (*self.vectors.get(&name.to_string()).unwrap()).into(),
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

#[derive(Parameter, Deref, FromStr)]
#[param(name = "var", regex = r"[\w][^\s]*")]
struct CaptureVar(String);

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

        Ok(CaptureTuple(Tuple::new(
            coords[0], coords[1], coords[2], coords[3],
        )))
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

        Ok(CapturePoint(Point::new(coords[0], coords[1], coords[2])))
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

        Ok(CaptureVector(Vector::new(coords[0], coords[1], coords[2])))
    }
}

#[derive(Parameter, Deref)]
#[param(
    name = "color",
    regex = r"color\(\s*[\d\.-]+\s*,\s*[\d\.-]+\s*,\s*[\d\.-]+\s*\)"
)]
struct CaptureColor(Color);
impl FromStr for CaptureColor {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<_> = s
            .to_lowercase()
            .strip_prefix("color")
            .expect("Color should start with cector")
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .map(|ss| {
                ss.trim()
                    .parse::<f32>()
                    .expect("Parsing component f32 failed")
            })
            .collect();

        Ok(CaptureColor(Color::new(coords[0], coords[1], coords[2])))
    }
}

// #[given(regex = r"^([^\s]) ← tuple\(([\d\.-]+), ([\d\.-]+), ([\d\.-]+), ([\d\.-]+)\)$")]
// fn a_tuple(world: &mut TuplesWorld, x: f32, y: f32, z: f32, w: f32) {
//     world.tuple = [x, y, z, w];
// }
#[given(expr = r"{word} ← {tuple}")]
fn a_tuple(world: &mut TuplesWorld, name: String, tuple: CaptureTuple) {
    let world_tuple = world.get_tuple(&name);
    *world_tuple = *tuple;
}

#[given(expr = r"{word} ← {point}")]
fn a_point(world: &mut TuplesWorld, name: String, point: CapturePoint) {
    let world_point = world.get_point(&name);
    *world_point = *point;
}

#[given(expr = r"{word} ← {vector}")]
fn a_vector(world: &mut TuplesWorld, name: String, vector: CaptureVector) {
    let world_vector = world.get_vector(&name);
    *world_vector = *vector;
}

#[given(expr = r"{word} ← {color}")]
fn a_color(world: &mut TuplesWorld, name: String, color: CaptureColor) {
    let world_color = world.get_color(&name);
    *world_color = *color;
}

#[then(regex = r"^([^\s])\.([xyzw]) = ([\d\.-]+)$")]
fn dim_equal(world: &mut TuplesWorld, name: String, dim: String, value: f32) {
    let world_tuple = world.get_any_as_tuple(&name);
    match dim.as_str() {
        "x" => assert_eq!(value, world_tuple.x()),
        "y" => assert_eq!(value, world_tuple.y()),
        "z" => assert_eq!(value, world_tuple.z()),
        "w" => assert_eq!(value, world_tuple.w()),
        _ => unreachable!(),
    };
}

#[then(regex = r"^([^\s])\.(red|green|blue|alpha) = ([\d\.-]+)$")]
fn dim_color_equal(world: &mut TuplesWorld, name: String, dim: String, value: f32) {
    let world_color = world.get_color(&name);
    match dim.as_str() {
        "red" => assert_eq!(value, world_color.r()),
        "green" => assert_eq!(value, world_color.g()),
        "blue" => assert_eq!(value, world_color.b()),
        _ => unreachable!(),
    };
}

#[then(expr = r"{var} = {tuple}")]
fn equal_to_tuple(world: &mut TuplesWorld, var: CaptureVar, tuple: CaptureTuple) {
    let world_tuple = world.get_any_as_tuple(var.as_str());
    assert_eq!(world_tuple, *tuple);
}

#[then(expr = r"-{var} = {tuple}")]
fn negative_equal_to_tuple(world: &mut TuplesWorld, var: CaptureVar, tuple: CaptureTuple) {
    let world_tuple = -world.get_any_as_tuple(var.as_str());
    assert_eq!(world_tuple, *tuple);
}

#[then(expr = r"{word} is a point")]
fn is_a_point(world: &mut TuplesWorld, name: String) {
    let world_tuple = world.get_any_as_tuple(&name);
    assert!(world_tuple.is_point());
}

#[then(expr = r"{word} is not a point")]
fn is_not_a_point(world: &mut TuplesWorld, name: String) {
    let world_tuple = world.get_any_as_tuple(&name);
    assert!(!world_tuple.is_point());
}

#[then(expr = r"{word} is a vector")]
fn is_a_vector(world: &mut TuplesWorld, name: String) {
    let world_tuple = world.get_any_as_tuple(&name);
    assert!(world_tuple.is_vector());
}

#[then(expr = r"{word} is not a vector")]
fn is_not_a_vector(world: &mut TuplesWorld, name: String) {
    let world_tuple = world.get_any_as_tuple(&name);
    assert!(!world_tuple.is_vector());
}

#[then(expr = r"a1 + a2 = {tuple}")]
fn a1_add_a2_eq_tuple(world: &mut TuplesWorld, tuple: CaptureTuple) {
    let result = world.a1 + world.a2;
    assert_eq!(result, *tuple);
}

#[then(expr = r"p + v = {point}")]
fn p_add_v_eq_vector(world: &mut TuplesWorld, point: CapturePoint) {
    let result = world.p + *world.get_vector("v");
    assert_eq!(result, *point);
}

#[then(expr = r"v1 + v2 = {vector}")]
fn v1_add_v2_eq_vector(world: &mut TuplesWorld, vector: CaptureVector) {
    let result = *world.get_vector("v1") + *world.get_vector("v2");
    assert_eq!(result, *vector);
}

#[then(expr = r"zero + v = {vector}")]
fn zero_add_v_eq_vector(world: &mut TuplesWorld, vector: CaptureVector) {
    let result = *world.get_vector("zero") + *world.get_vector("v");
    assert_eq!(result, *vector);
}

#[then(expr = r"c1 + c2 = {color}")]
fn v1_add_v2_eq_color(world: &mut TuplesWorld, color: CaptureColor) {
    let result = *world.get_color("c1") + *world.get_color("c2");
    assert_eq!(result, *color);
}

#[then(expr = r"p1 - p2 = {vector}")]
fn p1_sub_p2_eq_vector(world: &mut TuplesWorld, vector: CaptureVector) {
    let result = world.p1 - world.p2;
    assert_eq!(result, *vector);
}

#[then(expr = r"p - v = {point}")]
fn p_sub_v_eq_vector(world: &mut TuplesWorld, point: CapturePoint) {
    let result = world.p - *world.get_vector("v");
    assert_eq!(result, *point);
}

#[then(expr = r"v1 - v2 = {vector}")]
fn v1_sub_v2_eq_vector(world: &mut TuplesWorld, vector: CaptureVector) {
    let result = *world.get_vector("v1") - *world.get_vector("v2");
    assert_eq!(result, *vector);
}

#[then(expr = r"zero - v = {vector}")]
fn zero_sub_v_eq_vector(world: &mut TuplesWorld, vector: CaptureVector) {
    let result = *world.get_vector("zero") - *world.get_vector("v");
    assert_eq!(result, *vector);
}

#[then(expr = r"c1 - c2 = {color}")]
fn c1_sub_c2_eq_color(world: &mut TuplesWorld, color: CaptureColor) {
    let result = *world.get_color("c1") - *world.get_color("c2");
    assert_eq!(result, *color);
}

#[then(expr = r"a * {float} = {tuple}")]
fn a_mul_float_equal_tuple(world: &mut TuplesWorld, scaler: f32, tuple: CaptureTuple) {
    let result = *world.get_tuple("a") * scaler;
    assert_eq!(result, *tuple);
}

#[then(expr = r"c * {float} = {color}")]
fn a_mul_float_equal_color(world: &mut TuplesWorld, scaler: f32, color: CaptureColor) {
    let result = *world.get_color("c") * scaler;
    assert_eq!(result, *color);
}

#[then(expr = r"a \/ {float} = {tuple}")]
fn a_div_float_equal_tuple(world: &mut TuplesWorld, scaler: f32, tuple: CaptureTuple) {
    let result = *world.get_tuple("a") / scaler;
    assert_eq!(result, *tuple);
}

#[then(expr = r"magnitude\({word}\) = {float}")]
fn magnitude_v_equal_scaler(world: &mut TuplesWorld, name: String, scaler: f32) {
    let result = world.get_vector(&name).magnitude();
    assert_approx_eq!(f32, result, scaler);
}

#[then(expr = r"magnitude\(v\) = √{float}")]
fn magnitude_v_equal_sqrt_scaler(world: &mut TuplesWorld, scaler: f32) {
    let result = world.get_vector("v").magnitude();
    assert_eq!(result, scaler.sqrt());
}

#[then(expr = r"normalize\(v\) = {vector}")]
fn normalize_v_equal_vector(world: &mut TuplesWorld, vector: CaptureVector) {
    let result = world.get_vector("v").normalize();
    assert_eq!(result, *vector);
}

#[when(expr = r"norm ← normalize\(v\)")]
fn norm_vector_equal_normalize(world: &mut TuplesWorld) {
    *world.get_vector("norm") = world.get_vector("v").normalize();
}

#[then(expr = r"dot\(a, b\) = {float}")]
fn dot_vector_vector_equal_scaler(world: &mut TuplesWorld, scaler: f32) {
    let a = *world.get_vector("a");
    let b = *world.get_vector("b");
    assert_eq!(Vector::dot(a, b), scaler);
}

#[then(expr = r"cross\({word}, {word}\) = {vector}")]
fn cross_vector_vector_equal_vector(
    world: &mut TuplesWorld,
    name1: String,
    name2: String,
    vector: CaptureVector,
) {
    let a = *world.get_vector(&name1);
    let b = *world.get_vector(&name2);
    assert_eq!(Vector::cross(a, b), *vector);
}

#[then(expr = r"c1 * c2 = {color}")]
fn c1_mul_c2_eq_color(world: &mut TuplesWorld, color: CaptureColor) {
    let result = *world.get_color("c1") * *world.get_color("c2");
    assert_eq!(result, *color);
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(TuplesWorld::run("tests/features/tuples.feature"));
}
