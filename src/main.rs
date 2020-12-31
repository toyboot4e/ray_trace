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

use {glam::Vec3, rand::Rng};

use ray_trace::{
    trace::{DiffuseMaterial, MetalMaterial, Sphere},
    Camera, Color8u, Renderable, World,
};

fn print_color(c: Color8u) {
    println!("{} {} {}", c.r, c.g, c.b);
}

fn main() {
    let (w, h) = (200, 100);
    println!("P3\n{} {}\n255", w, h);

    // TODO: add convenient method for World
    let mut world = {
        let mut xs = Vec::<Renderable>::with_capacity(2);

        xs.push(Renderable {
            ix: 0,
            surface: Box::new(Sphere {
                center: Vec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
            }),
            material: Box::new(DiffuseMaterial {
                albedo: Vec3::new(0.8, 0.3, 0.3),
            }),
        });

        // big sphere!
        xs.push(Renderable {
            ix: 1,
            surface: Box::new(Sphere {
                center: Vec3::new(0.0, -100.5, -1.0),
                radius: 100.0,
            }),
            material: Box::new(DiffuseMaterial {
                albedo: Vec3::new(0.8, 0.8, 0.0),
            }),
        });

        // metalic sphere!
        xs.push(Renderable {
            ix: 2,
            surface: Box::new(Sphere {
                center: Vec3::new(1.0, 0.0, -1.0),
                radius: 0.5,
            }),
            material: Box::new(MetalMaterial {
                albedo: Vec3::new(0.8, 0.6, 0.2),
                fuzz: 0.5,
            }),
        });

        xs.push(Renderable {
            ix: 3,
            surface: Box::new(Sphere {
                center: Vec3::new(-1.0, 0.0, -1.0),
                radius: 0.5,
            }),
            material: Box::new(MetalMaterial {
                albedo: Vec3::new(0.8, 0.8, 0.8),
                fuzz: 0.5,
            }),
        });

        World { objs: xs }
    };

    // number of samplers per pixel
    let n_samples = 100;

    let cam = Camera::new();

    let mut rng = rand::thread_rng();

    for j in (0..h).rev() {
        for i in 0..w {
            let mut rgb = Vec3::new(0.0, 0.0, 0.0);

            // sample multiple rays for each pixel
            for _ in 0..n_samples {
                let ray = cam.ray([
                    (i as f32 + rng.gen_range(0.0..1.0)) as f32 / w as f32,
                    (j as f32 + rng.gen_range(0.0..1.0)) as f32 / h as f32,
                ]);

                rgb += ray_trace::color(&ray, &mut world)
            }

            rgb /= n_samples as f32;
            rgb = Vec3::new(rgb[0].sqrt(), rgb[1].sqrt(), rgb[2].sqrt());

            self::print_color(Color8u::from_normalized(rgb));
        }
    }
}
