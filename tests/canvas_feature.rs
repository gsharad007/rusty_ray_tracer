use rusty_ray_tracer::graphics2d::canvas::Canvas;

use std::convert::Infallible;

use async_trait::async_trait;
use cucumber::{given, World, WorldInit};

#[derive(WorldInit, Default, Debug)]
pub struct CanvasWorld {
    c: Canvas,
}

// `World` needs to be implemented, so Cucumber knows how to construct it
// for each scenario.
#[async_trait(?Send)]
impl World for CanvasWorld {
    // We do require some error type.
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self::default())
    }
}

#[given(expr = r"c ← canvas\(10, 20\)")]
fn a_tuple(world: &mut CanvasWorld) {
    let world_canvas = &mut world.c;
    *world_canvas = Canvas::new(10, 20);
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(CanvasWorld::run("tests/features/canvas.feature"));
}
