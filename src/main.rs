mod vector;
mod color;
mod ray;

use crate::vector::Vector;
use crate::color::{ color_fmt };

fn main() {
    let aspect_ratio = 16.0 / 9.0f64;
    let image_width = 384;
    let image_height = (image_width as f64 / aspect_ratio) as u16;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let viewport_height = 2.0f64;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vector::new(0f64, 0f64, 0f64);
    let horizontal = Vector::new(viewport_width, 0f64, 0f64);
    let vertical = Vector::new(0f64, viewport_height, 0f64);
    let lower_left_corner = origin - horizontal/2f64 - vertical/2f64;

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            eprint!("\rScanlines remaining: {} ", j);
            let color = Vector::new(
                i as f64 / (image_width as f64 - 1f64),
                j as f64 / (image_height as f64 - 1f64),
                0.25,
            );

            print!("{}\n", color_fmt(&color));
        }
    }
    eprint!("\nDone.\n");
}
