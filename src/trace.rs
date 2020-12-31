use {glam::Vec3, rand::Rng};

use crate::{Hit, HitData, HitRecord, Material, Ray, ScatterRecord};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_range: [f32; 2]) -> Option<HitData> {
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

        Some(HitData {
            t,
            pos: hit_point,
            n,
        })
    }
}

// --------------------------------------------------------------------------------
// impl Material

/// Lambertian
pub struct DiffuseMaterial {
    pub albedo: Vec3,
}

impl Material for DiffuseMaterial {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        return Some(ScatterRecord {
            new_ray: Ray {
                origin: hit.data.pos,
                dir: hit.data.n + random_point_in_unit_sphere(),
            },
            attenuation: self.albedo,
        });

        fn random_point_in_unit_sphere() -> Vec3 {
            let mut rnd = rand::thread_rng();

            loop {
                let [a, b, c] = [
                    rnd.gen_range(0.0..1.0),
                    rnd.gen_range(0.0..1.0),
                    rnd.gen_range(0.0..1.0),
                ];

                let p = 2.0 * Vec3::new(a, b, c) - Vec3::new(1.0, 1.0, 1.0);

                if p.length_squared() < 1.0 {
                    return p;
                }
            }
        }
    }
}

pub struct MetalMaterial {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Material for MetalMaterial {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let ray = Ray {
            origin: hit.data.pos,
            dir: reflect(ray.dir.normalize(), hit.data.n),
        };

        return Some(ScatterRecord {
            new_ray: ray,
            attenuation: self.albedo,
        });

        /// Inverts the `vec`'s perpendicular component to a surface
        fn reflect(vec: Vec3, normal: Vec3) -> Vec3 {
            vec - 2.0 * vec.dot(normal) * normal
        }
    }
}
