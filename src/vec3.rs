use std::ops::{Add, Div, Mul, Sub};

/* For geometric clarity as a point / position vector */
pub use Vec3 as Point3;

#[derive(Debug)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn from_array(array: &[f32; 3]) -> Self {
        Self {
            x: array[0].clone(),
            y: array[1].clone(),
            z: array[2].clone(),
        }
    }

    pub fn x(self: &Self) -> f32 {
        return self.x;
    }

    pub fn y(self: &Self) -> f32 {
        return self.y;
    }

    pub fn z(self: &Self) -> f32 {
        return self.z;
    }
}

impl<'a, 'b> Add<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn add(self: Self, other: &'b Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a, 'b> Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn sub(self: Self, other: &'b Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<'a, 'b> Mul<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn mul(self: Self, other: &'b Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl <'a> Mul<&'a mut Vec3> for f32 {
    type Output = &'a mut Vec3;

    fn mul(self: Self, other: &'a mut Vec3) -> &'a mut Vec3 {
        other.x *= self;
        other.y *= self;
        other.z *= self;
        return other;
    }
}

impl<'a, 'b> Div<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn div(self: Self, other: &'b Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Vec3 {
    pub fn magnitude(self: &Self) -> f32 {
        return (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
    }
}

