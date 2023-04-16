use std::collections::HashMap;

use cucumber::{gherkin::Step, given, then, World};
use float_cmp::ApproxEq;
use float_cmp::assert_approx_eq;
use rusty_ray_tracer::core3d::matrix::Cofactor;
use rusty_ray_tracer::core3d::matrix::Determinant;
use rusty_ray_tracer::core3d::matrix::Identity;
use rusty_ray_tracer::core3d::matrix::Invert;
use rusty_ray_tracer::core3d::matrix::Matrix22f32;
use rusty_ray_tracer::core3d::matrix::Matrix33f32;
use rusty_ray_tracer::core3d::matrix::Minor;
use rusty_ray_tracer::core3d::matrix::Submatrix;
use rusty_ray_tracer::core3d::matrix::Transpose;
use rusty_ray_tracer::core3d::{matrix::Matrix44f32, tuple::Tuple};

mod captures;
use crate::captures::CaptureTuple;

#[derive(Debug, Default, PartialEq, Copy, Clone)]
enum AnyMatrix {
    Mat44(Matrix44f32),
    Mat33(Matrix33f32),
    Mat22(Matrix22f32),
    #[default]
    None,
}

impl ApproxEq for AnyMatrix
{
    type Margin = <f32 as ApproxEq>::Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        match (self, other) {
            (AnyMatrix::Mat44(m1), AnyMatrix::Mat44(m2)) => m1.approx_eq(m2, margin),
            (AnyMatrix::Mat33(m1), AnyMatrix::Mat33(m2)) => m1.approx_eq(m2, margin),
            (AnyMatrix::Mat22(m1), AnyMatrix::Mat22(m2)) => m1.approx_eq(m2, margin),
            _ => false,
        }
    }
}

#[derive(World, Default, Debug)]
pub struct TheWorld {
    matrices: HashMap<String, AnyMatrix>,
    tuples: HashMap<String, Tuple>,
}
impl TheWorld {
    fn get_matrix(&mut self, name: &str) -> &mut AnyMatrix {
        self.matrices
            .entry(name.to_string())
            .or_insert_with(AnyMatrix::default)
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

fn parse_step_table_for_matrix(step: &Step) -> AnyMatrix {
    let table = parse_step_to_table(step);

    if table.len() == 4 && table[0].len() == 4 {
        AnyMatrix::Mat44(Matrix44f32::from(table))
    } else if table.len() == 3 && table[0].len() == 3 {
        AnyMatrix::Mat33(Matrix33f32::from(table))
    } else if table.len() == 2 && table[0].len() == 2 {
        AnyMatrix::Mat22(Matrix22f32::from(table))
    } else {
        assert!(table.len() >= 2 && table.len() <= 4);
        AnyMatrix::default()
    }
}

#[given(expr = r"the following 4x4 matrix {word}:")]
fn the_following_4x4_matrix_m(world: &mut TheWorld, name: String, step: &Step) {
    *world.get_matrix(&name) = parse_step_table_for_matrix(step);
}

#[then(expr = r"{word}[{int},{int}] = {float}")]
fn m_x_f(world: &mut TheWorld, name: String, r: usize, c: usize, result: f32) {
    let a = *world.get_matrix(&name);

    let value = match a {
        AnyMatrix::Mat44(m) => m.matrix[r][c],
        AnyMatrix::Mat33(m) => m.matrix[r][c],
        AnyMatrix::Mat22(m) => m.matrix[r][c],
        AnyMatrix::None => panic!("Accesssing unknown"),
    };
    assert_eq!(value, result);
}

#[then(expr = r"{word}[{int},{int}] = {int}\/{int}")]
fn m_x_num_den(world: &mut TheWorld, name: String, r: usize, c: usize, num: i32, den: i32) {
    let a = *world.get_matrix(&name);
    let result = num as f32 / den as f32;

    let value = match a {
        AnyMatrix::Mat44(m) => m.matrix[r][c],
        AnyMatrix::Mat33(m) => m.matrix[r][c],
        AnyMatrix::Mat22(m) => m.matrix[r][c],
        AnyMatrix::None => panic!("Accesssing unknown"),
    };
    assert_eq!(value, result);
}

#[given(expr = r"the following 2x2 matrix {word}:")]
fn the_following_2x2_matrix_m(world: &mut TheWorld, name: String, step: &Step) {
    *world.get_matrix(&name) = parse_step_table_for_matrix(step);
}

#[given(expr = r"the following 3x3 matrix {word}:")]
fn the_following_3x3_matrix_m(world: &mut TheWorld, name: String, step: &Step) {
    *world.get_matrix(&name) = parse_step_table_for_matrix(step);
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

#[then(expr = r"B is the following 4x4 matrix:")]
fn b_is_the_following_x_matrix(world: &mut TheWorld, step: &Step) {
    let expected = parse_step_table_for_matrix(step);

    let b = *world.get_matrix("B");

    // assert_approx_eq!(AnyMatrix, b, expected, epsilon = 0.000001);
    assert_approx_eq!(AnyMatrix, b, expected, ulps = 750);
}

#[then(expr = r"A * B is the following 4x4 matrix:")]
fn a_mul_b_is_the_following_x_matrix(world: &mut TheWorld, step: &Step) {
    let AnyMatrix::Mat44(expected) = parse_step_table_for_matrix(step) else {unreachable!()};

    let AnyMatrix::Mat44(a) = *world.get_matrix("A") else {unreachable!()};
    let AnyMatrix::Mat44(b) = *world.get_matrix("B") else {unreachable!()};

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

    let AnyMatrix::Mat44(a) = *world.get_matrix("A") else {unreachable!()};
    let b = *world.get_tuple("b");

    let result = a * b;
    assert_eq!(result, expected);
}

#[then(expr = r"A * identity_matrix = A")]
fn a_identity_matrix_x_d_a(world: &mut TheWorld) {
    let AnyMatrix::Mat44(a) = *world.get_matrix("A") else {unreachable!()};
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
    let AnyMatrix::Mat44(expected) = parse_step_table_for_matrix(step) else {unreachable!()};

    let AnyMatrix::Mat44(a) = *world.get_matrix("A") else {unreachable!()};

    let result = Matrix44f32::transpose(a);
    assert_eq!(result, expected);
}

#[given(expr = r"A ← transpose\(identity_matrix)")]
fn a_transpose_identity_matrix(world: &mut TheWorld) {
    *world.get_matrix("A") = AnyMatrix::Mat44(Matrix44f32::transpose(Matrix44f32::identity()));
}

#[then(expr = r"A = identity_matrix")]
fn a_x_d_identity_matrix(world: &mut TheWorld) {
    let AnyMatrix::Mat44(a) = *world.get_matrix("A") else {unreachable!()};
    let identity = Matrix44f32::identity();

    assert_eq!(a, identity);
}

#[then(expr = r"determinant\({word}) = {int}")]
fn determinant_a_x_d(world: &mut TheWorld, name: String, expected: f32) {
    let a = *world.get_matrix(&name);

    let result = match a {
        AnyMatrix::Mat44(a) => a.determinant(),
        AnyMatrix::Mat33(a) => a.determinant(),
        AnyMatrix::Mat22(a) => a.determinant(),
        AnyMatrix::None => unreachable!(),
    };

    assert_eq!(result, expected);
}

#[then(expr = r"submatrix\({word}, {int}, {int}) is the following {int}x{int} matrix:")]
fn submatrix_a_is_the_following_x_matrix(
    world: &mut TheWorld,
    name: String,
    r: usize,
    c: usize,
    _width: usize,
    _height: usize,
    step: &Step,
) {
    let expected = parse_step_table_for_matrix(step);

    match expected {
        AnyMatrix::Mat44(_) => unreachable!(),
        AnyMatrix::Mat33(expected) => {
            let AnyMatrix::Mat44(a) = *world.get_matrix(&name) else {unreachable!()};
            let b = a.submatrix(r, c);

            assert_eq!(b, expected);
        }
        AnyMatrix::Mat22(expected) => {
            let AnyMatrix::Mat33(a) = *world.get_matrix(&name) else {unreachable!()};
            let b = a.submatrix(r, c);

            assert_eq!(b, expected);
        }
        AnyMatrix::None => unreachable!(),
    }
}

#[given(expr = r"{word} ← submatrix\({word}, {int}, {int})")]
fn b_submatrix_a(world: &mut TheWorld, name_b: String, name_a: String, r: usize, c: usize) {
    let AnyMatrix::Mat33(a) = *world.get_matrix(&name_a) else {unreachable!()};
    *world.get_matrix(&name_b) = AnyMatrix::Mat22(a.submatrix(r, c));
}

#[then(expr = r"minor\({word}, {int}, {int}) = {float}")]
fn minor_a_x_d(world: &mut TheWorld, name: String, r: usize, c: usize, expected: f32) {
    let a = *world.get_matrix(&name);

    let b = match a {
        AnyMatrix::Mat44(a) => a.minor(r, c),
        AnyMatrix::Mat33(a) => a.minor(r, c),
        AnyMatrix::Mat22(_) | AnyMatrix::None => unreachable!(),
    };

    assert_eq!(b, expected);
}

#[then(expr = r"cofactor\({word}, {int}, {int}) = {float}")]
fn cofactor_a_x_d(world: &mut TheWorld, name: String, r: usize, c: usize, expected: f32) {
    let a = *world.get_matrix(&name);

    let b = match a {
        AnyMatrix::Mat44(a) => a.cofactor(r, c),
        AnyMatrix::Mat33(a) => a.cofactor(r, c),
        AnyMatrix::Mat22(_) | AnyMatrix::None => unreachable!(),
    };

    assert_eq!(b, expected);
}

#[then(expr = r"A is invertible")]
fn a_is_invertible(world: &mut TheWorld) {
    let AnyMatrix::Mat44(a) = *world.get_matrix("A") else {unreachable!()};
    assert!(a.is_invertible());
}

#[then(expr = r"A is not invertible")]
fn a_is_not_invertible(world: &mut TheWorld) {
    let AnyMatrix::Mat44(a) = *world.get_matrix("A") else {unreachable!()};
    assert!(!a.is_invertible());
}

#[given(expr = r"B ← inverse\(A)")]
fn b_inverse_a(world: &mut TheWorld) {
    let AnyMatrix::Mat44(a) = *world.get_matrix("A") else {unreachable!()};
    *world.get_matrix("B") = AnyMatrix::Mat44(a.inverse().unwrap());
}
