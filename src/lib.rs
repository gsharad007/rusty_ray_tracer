pub mod asset_types;
pub mod core3d;
pub mod graphics2d;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[cfg(test)]
mod projectile_tests {
    use crate::{
        asset_types::ppm::PPM,
        core3d::{color::Color, coordinates4::Coordinates4, point::Point, vector::Vector},
        graphics2d::canvas::Canvas,
    };
    // use log::info;
    // use test_log::test;
    use std::println as info;

    struct Projectile {
        position: Point,
        velocity: Vector,
    }

    struct Environment {
        gravity: Vector,
        wind: Vector,
    }

    const SCALE: u16 = 8; // 512
    const CANVAS_WIDTH: u16 = 2 * SCALE;
    const CANVAS_HEIGHT: u16 = SCALE;
    const POSITION_TO_CANVAS_SCALE: f32 = 0.66 * (SCALE as f32);
    fn map_projectile_position_to_canvas(point: Point) -> (u16, u16) {
        (
            (point.x() * POSITION_TO_CANVAS_SCALE) as u16,
            (CANVAS_HEIGHT - 1) - (point.y() * POSITION_TO_CANVAS_SCALE) as u16,
        )
    }

    #[test]
    // #[test_log::test] // Automatically wraps test to initialize logging
    fn projectile() {
        let projectile = Projectile {
            position: Point::new(0.0, 1.0, 0.0),
            velocity: Vector::new(1.0, 1.0, 0.0),
        };
        let environment = Environment {
            gravity: Vector::new(0.0, -0.980, 0.0),
            wind: Vector::new(-0.01, 0.0, 0.0),
        };
        assert_eq!(
            Vector::new(0.99, 0.02, 0.0),
            projectile.velocity + environment.gravity + environment.wind
        );
        const TICK_PER_FRAME: f32 = 1.0 / 120.0;
        const PROJECTILE_COLOR: Color = Color::new(1.0, 0.2, 0.2);
        let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);
        let mut travelling_projectile = projectile;
        let mut accumulated_ticks = 0.0;
        while travelling_projectile.position.y() > 0.0 {
            let coord = map_projectile_position_to_canvas(travelling_projectile.position);
            canvas.set_pixel_at(coord.0, coord.1, PROJECTILE_COLOR);

            accumulated_ticks += TICK_PER_FRAME;
            travelling_projectile = tick(TICK_PER_FRAME, travelling_projectile, &environment);
            info!(
                "travelling_projectile.position {}",
                travelling_projectile.position
            );
        }
        assert_eq!(2.775_008_7, accumulated_ticks);
        assert_eq!(
            Point::new(2.736_383_7, -0.009_639_465, 0.0),
            travelling_projectile.position
        );
        assert_eq!(
            Vector::new(0.972_252, -1.719_501_3, 0.0),
            travelling_projectile.velocity
        );
        let ppm = PPM::from(&canvas);
        assert_eq!(
            "\
P3
16 8
255
0 0 0 0 0 0 255 51 51 255 51 51 255 51 51 255 51 51 255 51 51 255 51
51 255 51 51 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
255 51 51 255 51 51 255 51 51 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 51 51
255 51 51 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
255 51 51 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 51 51
255 51 51 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 51 51
255 51 51 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
51 51 255 51 51 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 255 51 51 255 51 51 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 255 51 51 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 255 51 51 255 51 51 0 0 0
",
            ppm.to_string()
        );
        // let mut file =
        //     std::fs::File::create("travelling_projectile.ppm").expect("Failed to create file!");
        // std::io::Write::write_all(&mut file, ppm.to_string().as_bytes())
        //     .expect("Failed to write to file!");
    }

    fn tick(tick: f32, projectile: Projectile, environment: &Environment) -> Projectile {
        let velocity = projectile.velocity + (environment.gravity + environment.wind) * tick;
        let position = projectile.position + (velocity * tick);
        Projectile { position, velocity }
    }
}
