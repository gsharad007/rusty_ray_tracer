use rusty_ray_tracer::{
    asset_types::ppm::PPM,
    core3d::{color::Color, color_rgb::ColorRGB},
    graphics2d::canvas::Canvas,
};

use cucumber::{gherkin::Step, given, then, when, World};

mod captures;
use crate::captures::CaptureColor;

#[derive(World, Default, Debug)]
pub struct CanvasWorld {
    c: Canvas,
    red: Color,
    ppm: PPM,
}

#[given(expr = r"c ← canvas\({int}, {int}\)")]
fn a_canvas(world: &mut CanvasWorld, width: u16, height: u16) {
    let world_canvas = &mut world.c;
    *world_canvas = Canvas::new(width, height);
}

#[given(expr = r"red ← {color}")]
fn a_color(world: &mut CanvasWorld, color: CaptureColor) {
    let world_color = &mut world.red;
    *world_color = *color;
}

#[then(expr = r"c.width = {int}")]
fn dim_width_equals(world: &mut CanvasWorld, dimension: u16) {
    assert_eq!(dimension, world.c.width);
}

#[then(expr = r"c.height = {int}")]
fn dim_height_equals(world: &mut CanvasWorld, dimension: u16) {
    assert_eq!(dimension, world.c.height);
}

#[then(expr = r"every pixel of c is color\(0, 0, 0\)")]
fn every_pixel_equals(world: &mut CanvasWorld) {
    for y in 0..world.c.height {
        for x in 0..world.c.width {
            let pixel = world.c.get_pixel_at(x, y);
            assert_eq!(0.0, pixel.r());
            assert_eq!(0.0, pixel.g());
            assert_eq!(0.0, pixel.b());
        }
    }
}

#[when(expr = r"write_pixel\(c, 2, 3, red\)")]
fn write_pixel(world: &mut CanvasWorld) {
    world.c.set_pixel_at(2, 3, world.red);
}

#[then(expr = r"pixel_at\(c, 2, 3\) = red")]
fn pixel_at_equals(world: &mut CanvasWorld) {
    assert_eq!(world.c.get_pixel_at(2, 3), world.red);
}

#[when(expr = r"ppm ← canvas_to_ppm\(c\)")]
fn canvas_to_ppm(world: &mut CanvasWorld) {
    world.ppm = PPM::from(&world.c);
}

#[then(expr = r"lines 1-3 of ppm are")]
fn ppm_lines_are(world: &mut CanvasWorld, step: &Step) {
    let ppm_text = world.ppm.to_string();
    let mut result = ppm_text.split_whitespace();
    let mut expected = step.docstring.as_ref().unwrap().trim().split_whitespace();
    assert_eq!(expected.next(), result.next());
    assert_eq!(expected.next(), result.next());
    assert_eq!(expected.next(), result.next());
    assert_eq!(expected.next(), result.next());
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(CanvasWorld::run("tests/features/canvas.feature"));
}
