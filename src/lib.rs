pub mod trace;

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

pub struct World {
    pub objs: Vec<Renderable>,
}

impl World {
    /// Called by [`self::color`]
    fn hit(&self, ray: &Ray, t_range: [f32; 2]) -> Option<HitRecord> {
        let mut t_max = t_range[1];
        let mut hit: Option<HitRecord> = None;

        // colelct hit records and select the closest one
        for obj in &self.objs {
            if let Some(hit_rec) = obj.hit(ray, [t_range[0], t_max]) {
                t_max = hit_rec.data.t;
                hit = Some(hit_rec);
            }
        }

        hit
    }
}

pub struct Renderable {
    pub ix: usize,
    pub surface: Box<dyn Hit>,
    pub material: Box<dyn Material>,
}

impl Renderable {
    /// Called by [`World`]
    fn hit(&self, ray: &Ray, t_range: [f32; 2]) -> Option<HitRecord> {
        self.surface.hit(ray, t_range).map(|data| HitRecord {
            obj_ix: self.ix,
            data,
        })
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HitRecord {
    /// To which object the ray hit?
    pub obj_ix: usize,
    pub data: HitData,
}

pub fn color(ray: &Ray, world: &mut World) -> Vec3 {
    let dir = ray.dir.normalize();

    // if hit, sample color from the renderable
    if let Some(hit) = world.hit(ray, [0.001, f32::MAX]) {
        let obj = &world.objs[hit.obj_ix];

        if let Some(scatter) = obj.material.scatter(&hit) {
            // traverse the scattered ray
            // TODO: recursion depth check
            scatter.attenuation * color(&scatter.new_ray, world)
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    } else {
        // sample color from the background (gradation board)
        let t = 0.5 * (dir.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

// --------------------------------------------------------------------------------
// Ray tracing

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

pub trait Hit {
    // `t_range`: exclusibe range `(t_min, t_max)`
    fn hit(&self, ray: &Ray, t_range: [f32; 2]) -> Option<HitData>;
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HitData {
    /// Value to retrieve the hit point from [`Ray`]
    pub t: f32,
    /// Hit point
    pub pos: Vec3,
    /// Normal
    pub n: Vec3,
}

pub trait Material {
    /// Returns the scattered result (orelse completely absorbed)
    fn scatter(&self, rec: &HitRecord) -> Option<ScatterRecord>;
}

pub struct ScatterRecord {
    pub new_ray: Ray,
    pub attenuation: Vec3,
}

// --------------------------------------------------------------------------------
// Else

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
