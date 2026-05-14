/// A 3-dimensional vector with f64 components.
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    /// Adds two Vec3 vectors together.
    pub fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    /// Subtracts one Vec3 vector from another.
    pub fn subtract(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    /// Multiplies a Vec3 vector by a scalar value.
    pub fn multiply_by_scalar(self, scalar: f64) -> Vec3 {
        Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }

    /// Computes the magnitude (length) of the Vec3 vector.
    pub fn compute_magnitude(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalizes the Vec3 vector to have a magnitude of 1.
    pub fn normalize(self) -> Vec3 {
        let magnitude = self.compute_magnitude();
        if magnitude == 0.0 {
            Vec3::new(0.0, 0.0, 0.0)
        } else {
            self.multiply_by_scalar(1.0 / magnitude)
        }
    }
}