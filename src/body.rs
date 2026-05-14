use crate::vec3::Vec3;

/// A celestial body with properties such as mass, radius, position, velocity, and acceleration.
#[derive(Debug, Clone)]
pub struct CelestialBody {
    pub name: String,
    pub mass: f64,
    pub radius: f64,
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
}

impl CelestialBody {
    pub fn new(name: String, mass: f64, radius: f64, position: Vec3, velocity: Vec3) -> Self {
        CelestialBody {
            name,
            mass,
            radius,
            position,
            velocity,
            acceleration: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}