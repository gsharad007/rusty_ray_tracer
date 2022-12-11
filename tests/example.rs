use cucumber::{given, then, when, World};

// These `Cat` definitions would normally be inside your project's code,
// not test code, but we create them here for the show case.
#[derive(Debug, Default)]
struct Cat {
    pub fullness: u32,
    pub exploded: bool,
}

impl Cat {
    fn feed(&mut self, count: u32) {
        self.fullness += count;
        self.exploded |= self.fullness > 2;
    }
    const fn is_full(&self) -> bool {
        self.fullness > 0
    }
    const fn has_exploded(&self) -> bool {
        self.exploded
    }
}

// `World` is your shared, likely mutable state.
#[derive(Debug, Default, World)]
pub struct AnimalWorld {
    cat: Cat,
}

// Steps are defined with `given`, `when` and `then` attributes.
#[given(regex = r"^a (hungry|satiated|full) cat$")]
async fn hungry_cat(world: &mut AnimalWorld, state: String) {
    match state.as_str() {
        "hungry" => world.cat.fullness = 0,
        "satiated" => world.cat.fullness = 1,
        "full" => world.cat.fullness = 2,
        _ => unreachable!(),
    }
}
// #[given(expr = "a {word} cat")]
// fn hungry_cat(world: &mut AnimalWorld, state: String) {
//     match state.as_str() {
//         "hungry" =>  world.cat.hungry = true,
//         "satiated" =>  world.cat.hungry = false,
//         s => panic!("expected 'hungry' or 'satiated', found: {}", s),
//     }
// }

#[when(regex = r"^I feed the cat (\d+) times$")]
async fn feed_cat(world: &mut AnimalWorld, count: u32) {
    world.cat.feed(count);
}

#[then(regex = r"^the cat (is not hungry|has exploded)?$")]
async fn cat_is_fed(world: &mut AnimalWorld, result: String) {
    match result.as_str() {
        "is not hungry" => assert!(world.cat.is_full()),
        "has exploded" => assert!(world.cat.has_exploded()),
        _ => unreachable!(),
    };
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(AnimalWorld::run("tests/features/example"));
}
