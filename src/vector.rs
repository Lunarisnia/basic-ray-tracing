use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vector3(pub f32, pub f32, pub f32);

impl Vector3 {
    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn length(&self) -> f32 {
        self.squared().sqrt()
    }

    pub fn out(&self) {
        println!("{} {} {}", self.x(), self.y(), self.z());
    }

    pub fn squared(&self) -> f32 {
        (self.x() * self.x()) + (self.y() * self.y()) + (self.z() * self.z())
    }

    pub fn dot(&self, v: &Vector3) -> f32 {
        (self.x() * v.x()) + (self.y() * v.y()) + (self.z() * v.z())
    }

    pub fn cross(&self, v: &Vector3) -> Vector3 {
        Vector3(
            self.y() * v.z() - self.z() * v.y(),
            self.z() * v.x() - self.x() * v.z(),
            self.x() * v.y() - self.y() * v.x(),
        )
    }

    pub fn unit_vector(v: Vector3) -> Vector3 {
        let v_clone = Vector3(v.x(), v.y(), v.z());
        v_clone / v.length()
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3(-self.0, -self.1, -self.2)
    }
}

impl ops::Add<Vector3> for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.0 = self.x() + rhs.x();
        self.1 = self.y() + rhs.y();
        self.2 = self.z() + rhs.z();
    }
}

impl ops::Sub<Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl ops::Sub<&Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Self::Output {
        Vector3(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl ops::SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        self.0 = self.x() - rhs.x();
        self.1 = self.y() - rhs.y();
        self.2 = self.z() - rhs.z();
    }
}

impl ops::Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3(rhs.x() * self, rhs.y() * self, rhs.z() * self)
    }
}


impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl ops::Mul<f32> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 = self.x() * rhs;
        self.1 = self.y() * rhs;
        self.2 = self.z() * rhs;
    }
}

impl ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl ops::Mul<&Vector3> for f32 {
    type Output = Vector3;


    fn mul(self, rhs: &Vector3) -> Self::Output {
        Vector3(rhs.x() * self, rhs.y() * self, rhs.z() * self)
    }
}

impl ops::MulAssign<Vector3> for Vector3 {
    fn mul_assign(&mut self, rhs: Vector3) {
        self.0 = self.x() * rhs.x();
        self.1 = self.y() * rhs.y();
        self.2 = self.z() * rhs.z();
    }
}

impl ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0 / rhs;
    }
}

impl ops::Div<f32> for &Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0/rhs)
    }
}

