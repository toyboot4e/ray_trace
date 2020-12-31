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

use ray_trace::{Color8u, Ray, Sphere};

fn print_color(c: Color8u) {
    println!("{} {} {}", c.r, c.g, c.b);
}

/// Returns a `f32` that can be used to retrieve a hit point from [`Ray::expr`]
fn hit_sphere(sphere: &Sphere, ray: &Ray) -> Option<f32> {
    let face = ray.origin - sphere.center;

    let a = ray.dir.dot(ray.dir);
    let b = 2.0 * face.dot(ray.dir);
    let c = face.dot(face) - sphere.radius * sphere.radius;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        // two complex solutions: not hit point
        None
    } else {
        // choose the closer point of the two solutions of the quadratic equation
        Some((-b - discriminant.sqrt()) / (2.0 * a))
    }
}

fn color(ray: &Ray) -> Vec3 {
    let dir = ray.dir.normalize();

    let sphere = Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };

    if let Some(t) = hit_sphere(&sphere, ray) {
        let hit_point = ray.expr(t);
        let n = (hit_point - sphere.center).normalize();
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

    for j in (0..h).rev() {
        for i in 0..w {
            let ray = {
                // uv position on the gradiation board
                let uv = Vec2::new(i as f32 / w as f32, j as f32 / h as f32);
                // xyz position in the world coordinates
                let pos = left_down + (unit[0] * uv[0] + unit[1] * uv[1]);
                Ray { origin, dir: pos }
            };

            let rgb = color(&ray);
            print_color(Color8u::from_normalized(rgb));
        }
    }
}
