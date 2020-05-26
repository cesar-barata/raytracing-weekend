use crate::vector::Vector;

pub type Color = Vector<f64>;

pub fn color_fmt(color: &Color) -> String {
    let approx_factor = 255.999;
    let ir = (approx_factor * color.x()) as i64;
    let ig = (approx_factor * color.y()) as i64;
    let ib = (approx_factor * color.z()) as i64;
    format!("{} {} {}", ir, ig, ib)
}
