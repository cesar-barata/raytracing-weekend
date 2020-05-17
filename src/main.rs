mod point;
mod ray;

fn main() {
    let image_width = 256;
    let image_height = 256;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            eprint!("\rScanlines remaining: {} ", j);
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25f64;

            let ir = (255.999 * r) as i64;
            let ig = (255.999 * g) as i64;
            let ib = (255.999 * b) as i64;

            print!("{} {} {}\n", ir, ig, ib);
        }
    }
    eprint!("\nDone.\n");
}
