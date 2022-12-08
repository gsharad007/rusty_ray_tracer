use std::{str::FromStr, num::ParseFloatError};

use cucumber::Parameter;
use derive_more::{Deref, FromStr};
use rusty_ray_tracer::core3d::{tuple::Tuple, point::Point, vector::Vector, color::Color};


#[derive(Parameter, Deref, FromStr)]
#[param(name = "var", regex = r"[\w][^\s]*")]
pub struct CaptureVar(String);

#[derive(Parameter, Deref)]
#[param(
    name = "tuple",
    regex = r"tuple\(\s*[\d\.-]+\s*,\s*[\d\.-]+\s*,\s*[\d\.-]+\s*,\s*[\d\.-]+\s*\)"
)]
pub struct CaptureTuple(Tuple);
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

        Ok(Self(Tuple::new(coords[0], coords[1], coords[2], coords[3])))
    }
}

#[derive(Parameter, Deref)]
#[param(
    name = "point",
    regex = r"point\(\s*[\d\.-]+\s*,\s*[\d\.-]+\s*,\s*[\d\.-]+\s*\)"
)]
pub struct CapturePoint(Point);
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

        Ok(Self(Point::new(coords[0], coords[1], coords[2])))
    }
}

#[derive(Parameter, Deref)]
#[param(
    name = "vector",
    regex = r"vector\(\s*[\d\.-]+\s*,\s*[\d\.-]+\s*,\s*[\d\.-]+\s*\)"
)]
pub struct CaptureVector(Vector);
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

        Ok(Self(Vector::new(coords[0], coords[1], coords[2])))
    }
}

#[derive(Parameter, Deref)]
#[param(
    name = "color",
    regex = r"color\(\s*[\d\.-]+\s*,\s*[\d\.-]+\s*,\s*[\d\.-]+\s*\)"
)]
pub struct CaptureColor(Color);
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

        Ok(Self(Color::new(coords[0], coords[1], coords[2])))
    }
}
