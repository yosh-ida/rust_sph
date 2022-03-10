use super::vector::Vector2;

pub struct Particle {
    pub position: Vector2,
    pub velocity: Vector2,
    pub velocity2: Vector2,
    pub force: Vector2,
    pub accel: Vector2,
    pub pressure: f64,
    pub density: f64,
    pub mass: f64,
    pub temperature: f64,
    pub viscosity: f64,
    pub movable: bool,
    pub belong: usize,
}

impl Particle {
    pub fn new(pos: Vector2, vel: Vector2, m: f64, t: f64, movable: bool) -> Particle {
        Particle {
            position: pos,
            velocity: vel,
            velocity2: vel,
            force: Vector2::new(0.0, 0.0),
            accel: Vector2::new(0.0, 0.0),
            pressure: 0.0,
            density: 0.0,
            mass: m,
            temperature: t,
            viscosity: 1.0,
            movable: movable,
            belong: 0,
        }
    }
}
