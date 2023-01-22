#![feature(test)]

extern crate test;
use rusty_ray_tracer::core3d::tuple::Tuple;
use test::black_box;

pub mod helpers;

const A: Tuple = Tuple::new(1.23, 4.56, 7.89, 1.01);
const B: Tuple = Tuple::new(1.11, 2.22, 3.33, 4.44);
const C: Tuple = Tuple::new(5.55, 6.66, 7.77, 8.88);

fn new() -> Tuple {
    Tuple::new(
        black_box(1.0),
        black_box(2.0),
        black_box(3.0),
        black_box(4.0),
    )
}

fn clone() -> Tuple {
    A.clone()
}

fn debug_fmt() -> String {
    format!("{A:?}")
}

fn from_array() -> Tuple {
    Tuple::from(black_box([1.0, 2.0, 3.0, 4.0]))
}

fn display() -> String {
    format!("{A}")
}

fn add_closure() -> Tuple {
    A + B
}

fn add_identity() -> Tuple {
    let b = black_box(Tuple::default());
    A + b
}

fn add_associative() -> Tuple {
    A + (B + C)
}

fn sub_not_closure() -> Tuple {
    A - B
}

fn sub_not_identity() -> Tuple {
    let b = black_box(Tuple::default());
    A - b
}

fn sub_not_associative() -> Tuple {
    A - (B - C)
}

fn neg() -> Tuple {
    -A
}

fn double_neg() -> Tuple {
    -(-A)
}

fn mul_closure() -> Tuple {
    A * black_box(100.1)
}

fn div_closure() -> Tuple {
    A / black_box(100.1)
}

iai_main!(
    new,
    clone,
    debug_fmt,
    from_array,
    display,
    add_closure,
    add_identity,
    add_associative,
    sub_not_closure,
    sub_not_identity,
    sub_not_associative,
    neg,
    double_neg,
    mul_closure,
    div_closure,
);
crit_group!(
    benches,
    new,
    clone,
    debug_fmt,
    from_array,
    display,
    add_closure,
    add_identity,
    add_associative,
    sub_not_closure,
    sub_not_identity,
    sub_not_associative,
    neg,
    double_neg,
    mul_closure,
    div_closure,
);
crit_main!(benches);
fn main() {
    iai_runner();

    crit_runner();
}
