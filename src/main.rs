/*!

A scene with a gradation board in the background

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

use ray_trace::{Color8u, Ray};

fn print_color(c: Color8u) {
    println!("{} {} {}", c.r, c.g, c.b);
}

fn color(ray: &Ray) -> Vec3 {
    let dir = ray.dir.normalize();

    // sample color from the gladation board
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
