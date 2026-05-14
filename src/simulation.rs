use crate::vec3::Vec3;
use crate::body::CelestialBody;
use crate::physics::{compute_next_position, compute_next_acceleration, compute_next_velocity};

/// Simulates the motion of celestial bodies over a given time step.
pub fn simulate_step(bodies: &mut [CelestialBody], dt: f64) {
    // First, compute the next positions all bodies
    let next_positions: Vec<Vec3> = bodies.iter()
        .map(|body| compute_next_position(body, dt))
        .collect();

    // Apply the new positions to the bodies temporarily to compute the next accelerations
    for (i, body) in bodies.iter_mut().enumerate() {
        body.position = next_positions[i];
    }

    // Create a temporary copy to compute accelerations without modifying original bodies
    let temp_bodies = bodies.to_vec();

    // Then, compute the next accelerations based on the new positions
    let next_accelerations: Vec<Vec3> = bodies.iter()
        .map(|body| compute_next_acceleration(body, &temp_bodies))
        .collect();

    // Finally, compute the next velocities and update the bodies
    for (i, body) in bodies.iter_mut().enumerate() {
        let next_velocity = compute_next_velocity(body, next_accelerations[i], dt);
        body.velocity = next_velocity;
        body.acceleration = next_accelerations[i];
    }
}