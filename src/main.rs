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

*/

use glam::{Vec2, Vec3};

use ray_trace::{Color8u, HitRecord, Ray, Sphere, Surface, World};

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

    // gradation board: size [4.0, 2.0] with center at [0.0, 0.0, -1.0]
    let left_down = Vec3::new(-2.0, -1.0, -1.0);
    let unit = [Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0)];

    let origin = Vec3::new(0.0, 0.0, 0.0);

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

    for j in (0..h).rev() {
        for i in 0..w {
            let ray = {
                // uv position on the gradiation board
                let uv = Vec2::new(i as f32 / w as f32, j as f32 / h as f32);
                // xyz position in the world coordinates
                let pos = left_down + (unit[0] * uv[0] + unit[1] * uv[1]);
                Ray { origin, dir: pos }
            };

            let rgb = color(&ray, &world);

            print_color(Color8u::from_normalized(rgb));
        }
    }
}
