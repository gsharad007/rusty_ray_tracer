#![feature(test)]

extern crate test;
use test::black_box;

pub mod helpers;

fn fibonacci_recursive(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

fn fibonacci_iterative(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    if n != 0 {
        for _ in 0..n {
            let c = a + b;
            a = b;
            b = c;
        }
    }
    b
}

fn fib_rec() -> u64 {
    fibonacci_recursive(black_box(20))
}

fn fib_iter() -> u64 {
    fibonacci_iterative(black_box(20))
}

crit_group!(benches, fib_rec, fib_iter);
crit_main!(benches);
iai_main!(fib_rec, fib_iter);
fn main() {
    iai_runner();

    crit_runner();
}
