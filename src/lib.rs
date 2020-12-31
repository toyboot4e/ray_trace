use glam::Vec3;

#[derive(Debug, Clone, Default, Copy, PartialEq)]
pub struct Color8u {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color8u {
    pub fn from_normalized(xs: impl Into<[f32; 3]>) -> Self {
        let xs = xs.into();

        Self {
            r: (255.9 * xs[0]) as u8,
            g: (255.9 * xs[1]) as u8,
            b: (255.9 * xs[2]) as u8,
        }
    }
}

pub struct Ray {
    pub origin: Vec3,
    /// Not normalized
    pub dir: Vec3,
}

impl Ray {
    /// Ray's expression that returns a point
    pub fn expr(&self, t: f32) -> Vec3 {
        self.origin + self.dir * t
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}
