pub mod core3d;

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
    use crate::core3d::{coordinates4::Coordinates4, point::Point, vector::Vector};
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
        let mut travelling_projectile = projectile;
        let mut accumulated_ticks = 0.0;
        let tick_per_frame = 1.0 / 60.0;
        while travelling_projectile.position.y() > 0.0 {
            accumulated_ticks += tick_per_frame;
            travelling_projectile = tick(tick_per_frame, travelling_projectile, &environment);
            info!(
                "travelling_projectile.position {}",
                travelling_projectile.position
            );
        }
        assert_eq!(2.7666647, accumulated_ticks);
        assert_eq!(
            Point::new(2.7281666, -0.0066070966, 0.0),
            travelling_projectile.position
        );
        assert_eq!(
            Vector::new(0.97233534, -1.7113346, 0.0),
            travelling_projectile.velocity
        );
    }

    fn tick(tick: f32, projectile: Projectile, environment: &Environment) -> Projectile {
        let velocity = projectile.velocity + (environment.gravity + environment.wind) * tick;
        let position = projectile.position + (velocity * tick);
        Projectile { position, velocity }
    }
}
