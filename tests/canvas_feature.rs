use std::collections::HashMap;

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
    colors: HashMap<String, Color>,
    ppm: PPM,
}
impl CanvasWorld {
    fn get_color(&mut self, name: &str) -> &mut Color {
        self.colors
            .entry(name.to_string())
            .or_insert_with(Color::default)
    }
}

#[given(expr = r"c ← canvas\({int}, {int}\)")]
fn a_canvas(world: &mut CanvasWorld, width: u16, height: u16) {
    let world_canvas = &mut world.c;
    *world_canvas = Canvas::new(width, height);
}

#[given(expr = r"{word} ← {color}")]
fn a_color(world: &mut CanvasWorld, name: String, color: CaptureColor) {
    let world_color = world.get_color(&name);
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

#[when(expr = r"write_pixel\(c, {int}, {int}, {word}\)")]
fn write_pixel(world: &mut CanvasWorld, x: u16, y: u16, color_name: String) {
    let color = *world.get_color(&color_name);
    world.c.set_pixel_at(x, y, color);
}

#[when(expr = r"every pixel of c is set to {color}")]
fn write_all_pixels(world: &mut CanvasWorld, color: CaptureColor) {
    for y in 0..world.c.height {
        for x in 0..world.c.width {
            world.c.set_pixel_at(x, y, *color);
        }
    }
}

#[then(expr = r"pixel_at\(c, 2, 3\) = red")]
fn pixel_at_equals(world: &mut CanvasWorld) {
    let red = *world.get_color("red");
    assert_eq!(world.c.get_pixel_at(2, 3), red);
}

#[when(expr = r"ppm ← canvas_to_ppm\(c\)")]
fn canvas_to_ppm(world: &mut CanvasWorld) {
    world.ppm = PPM::from(&world.c);
}

#[then(expr = r"lines {int}-{int} of ppm are")]
fn ppm_select_lines_are(world: &mut CanvasWorld, start: usize, end: usize, step: &Step) {
    let ppm_text = world.ppm.to_string();
    let skip = start - 1;
    let take = end - skip;
    let result = ppm_text.lines().skip(skip).take(take);
    let expected = step.docstring.as_ref().unwrap().trim().lines();
    expected.zip(result).for_each(|t| assert_eq!(t.0, t.1));
}

#[then(expr = r"ppm ends with a newline character")]
fn ppm_ends_with_newline(world: &mut CanvasWorld) {
    let ppm_text = world.ppm.to_string();
    assert_eq!(true, ppm_text.ends_with('\n'));
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(CanvasWorld::run("tests/features/canvas.feature"));
}
