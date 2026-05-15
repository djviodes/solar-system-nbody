use crate::vec3::Vec3;
use crate::body::CelestialBody;

pub const G: f64 = 6.67430e-11;

/// Computes the next position of a body using Velocity Verlet integration.
pub fn compute_next_position(body: &CelestialBody, dt: f64) -> Vec3 {
    body.position
        .add(body.velocity.multiply_by_scalar(dt))
        .add(body.acceleration.multiply_by_scalar(0.5 * dt.powi(2)))
}

/// Computes the next acceleration of a body using Velocity Verlet integration.
pub fn compute_next_acceleration(
    body: &CelestialBody,
    other_bodies: &[CelestialBody]
) -> Vec3 {
    let mut total_acceleration = Vec3::new(0.0, 0.0, 0.0);
    for other in other_bodies {
        if other.name == body.name {
            continue;
        }
        let direction = other.position.subtract(body.position);
        let distance = direction.compute_magnitude();
        if distance > 0.0 {
            let acceleration_magnitude = (G * other.mass) / distance.powi(2);
            total_acceleration = total_acceleration
                .add(direction.normalize().multiply_by_scalar(acceleration_magnitude));
        }
    }
    total_acceleration
}

/// Computes the next velocity of a body using Velocity Verlet integration.
pub fn compute_next_velocity(body: &CelestialBody, next_acceleration: Vec3, dt: f64) -> Vec3 {
    body.velocity
        .add((body.acceleration.add(next_acceleration)).multiply_by_scalar(0.5 * dt))
}

/// Computes the total mechanical energy of all bodies in the system.
pub fn compute_total_system_energy(bodies: &[CelestialBody]) -> f64 {
    let mut total_energy = 0.0;

    // Compute kinetic energy
    for body in bodies {
        total_energy += 0.5 * body.mass * body.velocity.compute_magnitude().powi(2);
    };

    // Compute potential energy
    for i in 0..bodies.len() {
        for j in (i + 1)..bodies.len() {
            let distance = bodies[i].position.subtract(bodies[j].position).compute_magnitude();
            if distance > 0.0 {
                total_energy -= G * bodies[i].mass * bodies[j].mass / distance;
            }
        }
    };

    total_energy
}