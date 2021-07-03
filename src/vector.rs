use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

//#region Vector4
#[derive(Copy, Clone, Debug)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn zero() -> Vector4 {
        Vector4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        Vector4 {
            x,
            y,
            z,
            w,
        }
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&mut self) {
        let inv_length = 1.0 / self.length();
        self.x *= inv_length;
        self.y *= inv_length;
        self.z *= inv_length;
        self.w *= inv_length;
    }

    pub fn distance_squared(&self, other: &Vector4) -> f32 {
        let d_x = self.x - other.x;
        let d_y = self.y - other.y;
        let d_z = self.z - other.z;
        let d_w = self.w - other.w;

        d_x * d_x + d_y * d_y + d_z * d_z + d_w * d_w
    }

    pub fn distance(&self, other: &Vector4) -> f32 {
        self.distance_squared(other).sqrt()
    }

    pub fn dot(&self, other: &Vector4) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}

impl PartialEq<Vector4> for Vector4 {
    fn eq(&self, other: &Vector4) -> bool {
        self.x <= other.x + f32::EPSILON && self.x >= other.x + f32::EPSILON &&
            self.y <= other.y + f32::EPSILON && self.y >= other.y + f32::EPSILON &&
            self.z <= other.z + f32::EPSILON && self.z >= other.z + f32::EPSILON &&
            self.w <= other.w + f32::EPSILON && self.w >= other.w + f32::EPSILON
    }
}

impl From<[f32; 4]> for Vector4 {
    fn from(e: [f32; 4]) -> Self {
        Vector4 {
            x: e[0],
            y: e[1],
            z: e[2],
            w: e[3],
        }
    }
}

impl Into<[f32; 4]> for Vector4 {
    fn into(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl Neg for Vector4 {
    type Output = Vector4;

    fn neg(self) -> Self::Output {
        Vector4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Add<Vector4> for Vector4 {
    type Output = Vector4;

    fn add(self, rhs: Vector4) -> Self::Output {
        Vector4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl AddAssign<Vector4> for Vector4 {
    fn add_assign(&mut self, rhs: Vector4) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl Sub<Vector4> for Vector4 {
    type Output = Vector4;

    fn sub(self, rhs: Vector4) -> Self::Output {
        Vector4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl SubAssign<Vector4> for Vector4 {
    fn sub_assign(&mut self, rhs: Vector4) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

impl Mul<Vector4> for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Self::Output {
        Vector4 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl MulAssign<Vector4> for Vector4 {
    fn mul_assign(&mut self, rhs: Vector4) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}

impl Mul<f32> for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl MulAssign<f32> for Vector4 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl Div<Vector4> for Vector4 {
    type Output = Vector4;

    fn div(self, rhs: Vector4) -> Self::Output {
        Vector4 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}

impl DivAssign<Vector4> for Vector4 {
    fn div_assign(&mut self, rhs: Vector4) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}

impl Div<f32> for Vector4 {
    type Output = Vector4;

    fn div(self, rhs: f32) -> Self::Output {
        Vector4 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl DivAssign<f32> for Vector4 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}
//#endregion

//#region Vector3
#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn zero() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 {
            x,
            y,
            z,
        }
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&mut self) {
        let inv_length = 1.0 / self.length();
        self.x *= inv_length;
        self.y *= inv_length;
        self.z *= inv_length;
    }

    pub fn distance_squared(&self, other: &Vector3) -> f32 {
        let d_x = self.x - other.x;
        let d_y = self.y - other.y;
        let d_z = self.z - other.z;

        d_x * d_x + d_y * d_y + d_z * d_z
    }

    pub fn distance(&self, other: &Vector3) -> f32 {
        self.distance_squared(other).sqrt()
    }

    pub fn dot(&self, other: &Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z + (-self.z * other.y),
            y: self.z * other.x + (-self.x * other.z),
            z: self.x * other.y + (-self.y * other.x),
        }
    }
}

impl PartialEq<Vector3> for Vector3 {
    fn eq(&self, other: &Vector3) -> bool {
        self.x <= other.x + f32::EPSILON && self.x >= other.x + f32::EPSILON &&
            self.y <= other.y + f32::EPSILON && self.y >= other.y + f32::EPSILON &&
            self.z <= other.z + f32::EPSILON && self.z >= other.z + f32::EPSILON
    }
}

impl From<[f32; 3]> for Vector3 {
    fn from(e: [f32; 3]) -> Self {
        Vector3 {
            x: e[0],
            y: e[1],
            z: e[2],
        }
    }
}

impl Into<[f32; 3]> for Vector3 {
    fn into(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign<Vector3> for Vector3 {
    fn mul_assign(&mut self, rhs: Vector3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<Vector3> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl DivAssign<Vector3> for Vector3 {
    fn div_assign(&mut self, rhs: Vector3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
//#endregion

//#region Vector2
#[derive(Copy, Clone, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn zero() -> Vector2 {
        Vector2 {
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 {
            x,
            y,
        }
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&mut self) {
        let inv_length = 1.0 / self.length();
        self.x *= inv_length;
        self.y *= inv_length;
    }

    pub fn distance_squared(&self, other: &Vector2) -> f32 {
        let d_x = self.x - other.x;
        let d_y = self.y - other.y;

        d_x * d_x + d_y * d_y
    }

    pub fn distance(&self, other: &Vector2) -> f32 {
        self.distance_squared(other).sqrt()
    }

    pub fn dot(&self, other: &Vector2) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

impl PartialEq<Vector2> for Vector2 {
    fn eq(&self, other: &Vector2) -> bool {
        self.x <= other.x + f32::EPSILON && self.x >= other.x + f32::EPSILON &&
            self.y <= other.y + f32::EPSILON && self.y >= other.y + f32::EPSILON
    }
}

impl From<[f32; 3]> for Vector2 {
    fn from(e: [f32; 3]) -> Self {
        Vector2 {
            x: e[0],
            y: e[1],
        }
    }
}

impl Into<[f32; 2]> for Vector2 {
    fn into(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Self::Output {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Vector2> for Vector2 {
    fn sub_assign(&mut self, rhs: Vector2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<Vector2> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl MulAssign<Vector2> for Vector2 {
    fn mul_assign(&mut self, rhs: Vector2) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<Vector2> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl DivAssign<Vector2> for Vector2 {
    fn div_assign(&mut self, rhs: Vector2) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: f32) -> Self::Output {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
//#endregion