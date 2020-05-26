use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};
use num::{Float};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Vector<Scalar: Float> {
    x: Scalar,
    y: Scalar,
    z: Scalar,
}

impl<Scalar: Float> Vector<Scalar> {
    pub fn new(x: Scalar, y: Scalar, z: Scalar) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> Scalar {
        self.x
    }

    pub fn y(&self) -> Scalar {
        self.y
    }

    pub fn z(&self) -> Scalar {
        self.z
    }

    pub fn length_squared(&self) -> Scalar {
        self * self
    }

    pub fn length(&self) -> Scalar {
        self.length_squared().sqrt()
    }

    pub fn cross_product(&self, rhs: &Self) -> Self {
        let x = self.y() * rhs.z() - self.z() * rhs.y();
        let y = self.z() * rhs.x() - self.x() * rhs.z();
        let z = self.x() * rhs.y() - self.y() * rhs.x();
        Self::new(x, y, z)
    }

    pub fn unit_direction(&self) -> Self {
        self / self.length()
    }
}

impl<Scalar: Float> AddAssign for Vector<Scalar> {
    fn add_assign(&mut self, rhs: Self) {
        *self = &*self + &rhs
    }
}

impl<Scalar: Float> SubAssign for Vector<Scalar> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = &*self - &rhs
    }
}

impl<Scalar: Float> MulAssign<Scalar> for Vector<Scalar> {
    fn mul_assign(&mut self, rhs: Scalar) {
        *self = &*self * rhs
    }
}

impl<Scalar: Float> DivAssign<Scalar> for Vector<Scalar> {
    fn div_assign(&mut self, rhs: Scalar) {
        *self = &*self / rhs
    }
}

// Inner product
impl<Scalar: Float> Mul for &Vector<Scalar> {
    type Output = Scalar;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }
}

impl<Scalar: Float> Mul for Vector<Scalar> {
    type Output = Scalar;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

// Scalar multiplication
impl<Scalar: Float> Mul<Scalar> for &Vector<Scalar> {
    type Output = Vector<Scalar>;

    fn mul(self, scalar: Scalar) -> Self::Output {
        Vector::new(
            self.x() * scalar,
            self.y() * scalar,
            self.z() * scalar,
        )
    }
}

impl<Scalar: Float> Mul<Scalar> for Vector<Scalar> {
    type Output = Vector<Scalar>;

    fn mul(self, scalar: Scalar) -> Self::Output {
        &self * scalar
    }
}

// Scalar division
impl<Scalar: Float> Div<Scalar> for &Vector<Scalar> {
    type Output = Vector<Scalar>;

    fn div(self, scalar: Scalar) -> Self::Output {
        self * scalar.recip()
    }
}

impl<Scalar: Float> Div<Scalar> for Vector<Scalar> {
    type Output = Vector<Scalar>;

    fn div(self, scalar: Scalar) -> Self::Output {
        &self / scalar.recip()
    }
}

impl<Scalar: Float> Add<&Vector<Scalar>> for Vector<Scalar> {
    type Output = Vector<Scalar>;

    fn add(self, other: &Self) -> Self::Output {
        &self + other
    }
}

impl<Scalar: Float> Add for &Vector<Scalar> {
    type Output = Vector<Scalar>;

    fn add(self, other: Self) -> Self::Output {
        Vector::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl<Scalar: Float> Add<Vector<Scalar>> for &Vector<Scalar> {
    type Output = Vector<Scalar>;

    fn add(self, other: Vector<Scalar>) -> Self::Output {
        self + &other
    }
}

impl<Scalar: Float> Add for Vector<Scalar> {
    type Output = Vector<Scalar>;

    fn add(self, other: Self) -> Self::Output {
        &self + other
    }
}

impl<Scalar: Float> Neg for &Vector<Scalar> {
    type Output = Vector<Scalar>;

    fn neg(self) -> Self::Output {
        Vector::new(
            -self.x(),
            -self.y(),
            -self.z(),
        )
    }
}

impl<Scalar: Float> Neg for Vector<Scalar> {
    type Output = Vector<Scalar>;

    fn neg(self) -> Self::Output {
        -&self
    }
}

impl<Scalar: Float> Sub<&Vector<Scalar>> for Vector<Scalar> {
    type Output = Vector<Scalar>;

    fn sub(self, other: &Vector<Scalar>) -> Self::Output {
        &self - other
    }
}

impl<Scalar: Float> Sub for &Vector<Scalar> {
    type Output = Vector<Scalar>;

    fn sub(self, other: &Vector<Scalar>) -> Self::Output {
        self + &(-other)
    }
}

impl<Scalar: Float> Sub<Vector<Scalar>> for &Vector<Scalar> {
    type Output = Vector<Scalar>;

    fn sub(self, other: Vector<Scalar>) -> Self::Output {
        self - &other
    }
}

impl<Scalar: Float> Sub for Vector<Scalar> {
    type Output = Vector<Scalar>;

    fn sub(self, other: Self) -> Self::Output {
        &self - &other
    }
}
