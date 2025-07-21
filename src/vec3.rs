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
    pub fn new(x: f32, y:f32, z:f32) -> Self {
        Self {
            x,
            y,
            z,
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

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self: Self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self: Self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self: Self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self: Self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self: Self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self: Self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self: Self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self: Self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self: Self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self: Self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Mul<f32> for &Vec3 {
    type Output = Vec3;
    fn mul(self: Self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self: Self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f32> for &Vec3 {
    type Output = Vec3;
    fn div(self: Self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

/* utility to avoid writing out scalar reciprocals */
impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self: Self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Vec3 {
    pub fn magnitude(&self) -> f32 {
        return (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
    }

    pub fn dot_product(self: &Self, rhs: &Vec3) -> f32 {
        return 
            self.x * rhs.x
            +   self.y * rhs.y
            +   self.z * rhs.z;
    }

    pub fn cross_product(self: &Self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.z - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn normalise(v: &Self) -> Self {
        let magnitude = v.magnitude();
        return v / magnitude;
    }
}

#[cfg(test)]
mod tests {
    use super::{Vec3};

    #[test]
    fn normalise() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let unit_vector = Vec3::normalise(&v);
        assert!((1.0 - unit_vector.magnitude()) - f32::EPSILON < 0.0);
    }

    #[test]
    fn dot_product() {
        let v1 = Vec3::new(1.0, 1.0, 1.0);
        let v2 = Vec3::new(1.0, 1.0, 1.0);
        let dot_product = v1.dot_product(&v2);
        assert_eq!(dot_product, 3.0);
    }

    #[test]
    fn commutativity_scalar_multiplication() {
        let v1 = Vec3::new(1.0, 1.0, 1.0);
        let v2 = Vec3::new(2.0, 2.0, 2.0);
        let actual1 = 2.0 * &v1;
        let actual2 = &v1 * 2.0;
        assert_eq!(
            v2.x - actual1.x + v2.y - actual1.y + v2.z - actual1.z,
            v2.x - actual2.x + v2.y - actual2.y + v2.z - actual2.z
        );
    }

    #[test]
    fn vector_addition() {
        let v1 = Vec3::new(1.0, 1.0, 1.0);
        let v2 = Vec3::new(2.0, 2.0, 2.0);
        let actual = &v1 + &v2;
        assert!(actual.x == 3.0 && actual.y == 3.0 && actual.z == 3.0);
    }

    #[test]
    fn vector_subtraction() {
        let v1 = Vec3::new(1.0, 1.0, 1.0);
        let v2 = Vec3::new(2.0, 2.0, 2.0);
        let actual = &v1 + &v2;
        assert!(actual.x == 3.0 && actual.y == 3.0 && actual.z == 3.0);
    }
}
