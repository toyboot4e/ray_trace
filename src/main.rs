use glam::Vec3;

use ray_trace::Color8u;

fn print_color(c: Color8u) {
    println!("{} {} {}", c.r, c.g, c.b);
}

fn main() {
    let (w, h) = (200, 100);
    println!("P3\n{} {}\n255", w, h);

    for j in (0..h).rev() {
        for i in 0..w {
            let rgb_f32 = Vec3::from([i as f32 / w as f32, j as f32 / h as f32, 0.2]);
            let rgb_u8 = Color8u::from_normalized(rgb_f32);
            print_color(rgb_u8);
        }
    }
}
