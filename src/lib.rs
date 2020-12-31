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

#[derive(Debug, Clone, Default, PartialEq)]
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

/// What a [`Ray`] can hit
pub trait Surface {
    // `t_range`: exclusibe range `(t_min, t_max)`
    fn hit(&self, ray: &Ray, t_range: [f32; 2]) -> Option<HitRecord>;
}

pub struct World {
    pub objs: Vec<Box<dyn Surface>>,
    pub rng: rand::rngs::ThreadRng,
}

impl Surface for World {
    fn hit(&self, ray: &Ray, t_range: [f32; 2]) -> Option<HitRecord> {
        let mut t_max = t_range[1];

        // hit record at closest point
        let mut rec: Option<HitRecord> = None;

        for obj in &self.objs {
            if let Some(r) = obj.hit(ray, [t_range[0], t_max]) {
                t_max = r.t;
                rec = Some(r);
            }
        }

        rec
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HitRecord {
    /// Value to retrieve the hit point from [`Ray`]
    pub t: f32,
    /// Hit point
    pub pos: Vec3,
    /// Normal
    pub n: Vec3,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Surface for Sphere {
    fn hit(&self, ray: &Ray, t_range: [f32; 2]) -> Option<HitRecord> {
        let face = ray.origin - self.center;

        let a = ray.dir.dot(ray.dir);
        let b = 2.0 * face.dot(ray.dir);
        let c = face.dot(face) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            // two complex solutions: not hit point
            return None;
        }

        // choose the closer point of the two solutions of the quadratic equation
        let t = (-b - discriminant.sqrt()) / (2.0 * a);

        // not in range; filtered
        if t < t_range[0] || t > t_range[1] {
            return None;
        }

        // finally return a record
        let hit_point = ray.expr(t);
        let n = (hit_point - self.center) / self.radius;

        Some(HitRecord {
            t,
            pos: hit_point,
            n,
        })
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Camera {
    origin: Vec3,
    left_down: Vec3,
    h: Vec3,
    v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            origin: Vec3::new(0.0, 0.0, 0.0),
            left_down: Vec3::new(-2.0, -1.0, -1.0),
            h: Vec3::new(4.0, 0.0, 0.0),
            v: Vec3::new(0.0, 2.0, 0.0),
        }
    }

    pub fn ray(&self, uv: [f32; 2]) -> Ray {
        Ray {
            origin: self.origin,
            dir: self.left_down + uv[0] * self.h + uv[1] * self.v,
        }
    }
}
