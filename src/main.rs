/*!

A scene with a sphere and a gradation board in the background

# Coordinate system

Right-handed coordinate system with y axis going up:

```no_run
   y
   ^
   |
---+---> x
   |
   |     z axis: from the screen to you
```

* `uv`: normalized screen coordinates
*/

use {
    glam::{Vec2, Vec3},
    rand::Rng,
};

use ray_trace::{Camera, Color8u, HitRecord, Ray, Sphere, Surface, World};

fn print_color(c: Color8u) {
    println!("{} {} {}", c.r, c.g, c.b);
}

fn color(ray: &Ray, world: &World) -> Vec3 {
    let dir = ray.dir.normalize();

    if let Some(rec) = world.hit(ray, [0.0, f32::MAX]) {
        let n = rec.n;
        return 0.5 * Vec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }

    // sample color from the background (gradation board)
    let t = 0.5 * (dir.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let (w, h) = (200, 100);
    println!("P3\n{} {}\n255", w, h);

    // number of samplers per pixel
    let n_samples = 100;

    let cam = Camera::new();

    let world = {
        let mut xs = Vec::<Box<dyn Surface>>::with_capacity(2);

        xs.push(Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        }));

        // big sphere!
        xs.push(Box::new(Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        }));

        World { objs: xs }
    };

    let mut rnd = rand::thread_rng();

    for j in (0..h).rev() {
        for i in 0..w {
            let mut rgb = Vec3::new(0.0, 0.0, 0.0);

            // sample multiple rays for each pixel
            for _ in 0..n_samples {
                let ray = cam.ray([
                    (i as f32 + rnd.gen_range(0.0..1.0)) as f32 / w as f32,
                    (j as f32 + rnd.gen_range(0.0..1.0)) as f32 / h as f32,
                ]);

                rgb += self::color(&ray, &world)
            }

            rgb /= n_samples as f32;

            self::print_color(Color8u::from_normalized(rgb));
        }
    }
}
