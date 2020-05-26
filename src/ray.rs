use crate::vector::Vector;
use crate::color::{Color};

pub type Point3D = Vector<f64>;

#[derive(Default)]
struct Ray {
    origin: Point3D,
    direction: Vector<f64>,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vector<f64>) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Point3D {
        &self.origin
    }

    pub fn direction(&self) -> &Vector<f64> {
        &self.direction
    }

    pub fn at(self, t: f64) -> Point3D {
        &self.origin + &(&self.direction * t)
    }

    pub fn color(&self) -> Color {
        let t = 0.5 * (self.direction().unit_direction().y() + 1f64);
        &(&Vector::new(1.0, 1.0, 1.0) * (1.0 - t)) + &(&Vector::new(0.5, 0.7, 1.0) * t)
    }
}