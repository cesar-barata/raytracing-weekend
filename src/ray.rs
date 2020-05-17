use crate::point::*;

#[derive(Default)]
struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    fn new(origin: Point, direction: Point) -> Ray {
        Ray { origin, direction }
    }

    fn origin(&self) -> &Point {
        &self.origin
    }

    fn direction(&self) -> &Point {
        &self.direction
    }

    fn at(self, t: f64) -> Point {
        &self.origin + &(t * &self.direction)
    }
}