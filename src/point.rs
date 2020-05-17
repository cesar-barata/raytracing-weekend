pub type Scalar = f64;
pub type Point = Vector;
pub type Color = Vector;

#[derive(Debug, Default, Clone)]
pub struct Vector {
    data: [Scalar; 3]
}

impl Vector {
    fn new(x: Scalar, y: Scalar, z: Scalar) -> Self {
        Self { data: [x, y, z] }
    }

    fn x(&self) -> Scalar {
        self.data[0]
    }

    fn y(&self) -> Scalar {
        self.data[1]
    }

    fn z(&self) -> Scalar {
        self.data[2]
    }

    fn length_squared(&self) -> Scalar {
        self * self
    }

    fn length(&self) -> Scalar {
        self.length_squared().sqrt()
    }
}

impl std::ops::Mul<&Vector> for &Vector {
    type Output = Scalar;

    fn mul(self, rhs: &Vector) -> Self::Output {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }
}

impl std::ops::Mul<Scalar> for &Vector {
    type Output = Vector;

    fn mul(self, scalar: Scalar) -> Self::Output {
        Vector::new(
            self.x() * scalar,
            self.y() * scalar,
            self.z() * scalar,
        )
    }
}

impl std::ops::Mul<&Vector> for Scalar {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        self * rhs
    }
}

impl std::ops::Div<Scalar> for &Vector {
    type Output = Vector;

    fn div(self, scalar: Scalar) -> Self::Output {
        self * (1 as Scalar / scalar)
    }
}

impl std::ops::Add for &Vector {
    type Output = Vector;

    fn add(self, other: &Vector) -> Self::Output {
        Vector::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl std::ops::Neg for &Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(
            -self.x(),
            -self.y(),
            -self.z(),
        )
    }
}

impl std::ops::Sub for &Vector {
    type Output = Vector;

    fn sub(self, other: &Vector) -> Self::Output {
        self + &(-other)
    }
}
