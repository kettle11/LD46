use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub const PI: f32 = std::f32::consts::PI;
pub const RADIANS_TO_DEGREES: f32 = 180.0 / PI;
pub const DEGREES_TO_RADIANS: f32 = PI / 180.0;

pub fn radians(degrees: f32) -> f32 {
    DEGREES_TO_RADIANS * degrees
}

// Vector4
#[derive(Debug, Copy, Clone)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    #[inline]
    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    #[inline]
    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn mul(self, rhs: f32) -> Vector2 {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn div(self, rhs: f32) -> Vector2 {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vector2i {
    pub x: i32,
    pub y: i32,
}
#[derive(Debug, Copy, Clone)]
pub struct Plane {
    pub point: Vector3,
    pub normal: Vector3,
}

impl Plane {
    pub fn new(point: Vector3, normal: Vector3) -> Self {
        Self { point, normal }
    }
}

// https://en.wikipedia.org/wiki/Line%E2%80%93plane_intersection
pub fn plane_and_ray(plane: &Plane, ray: &Ray) -> Option<(f32, Vector3)> {
    let bottom = Vector3::dot(ray.direction, plane.normal);

    if bottom == 0.0 {
        None // No intersection
    } else {
        let top = Vector3::dot(plane.point - ray.origin, plane.normal);

        if top == 0.0 {
            None // Technically it intersects the entire plane, because the line is on the plane.
                 // However for now we're just saying it doesn't intersect.
        } else {
            let distance = top / bottom;
            Some((distance, ray.direction * distance + ray.origin))
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub direction: Direction3,
    pub origin: Point3,
    pub inverse: Vector3, // Multiplicative inverse. 1 / direction.
}

impl Ray {
    pub fn new(origin: Point3, direction: Direction3) -> Self {
        Self {
            origin,
            direction,
            inverse: Vector3::new(1. / direction.x, 1. / direction.y, 1. / direction.z),
        }
    }
}

pub type Point3 = Vector3;
pub type Direction3 = Vector3;

// ----------------- Vector3 -----------------------
#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Neg for Vector3 {
    type Output = Vector3;

    #[inline]
    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    #[inline]
    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector3 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: f32) -> Vector3 {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f32> for Vector3 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        };
    }
}

impl Div<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn div(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn div(self, rhs: f32) -> Vector3 {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f32> for Vector3 {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        };
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    #[inline]
    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vector3 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn new_uniform(value: f32) -> Vector3 {
        Vector3 {
            x: value,
            y: value,
            z: value,
        }
    }

    #[inline]
    pub fn dot(a: Vector3, b: Vector3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    #[inline]
    pub fn zxy(self) -> Vector3 {
        Vector3 {
            x: self.z,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    pub fn cross(a: Vector3, b: Vector3) -> Vector3 {
        (a.zxy() * b - a * b.zxy()).zxy()
    }

    #[inline]
    pub fn length(self) -> f32 {
        Vector3::dot(self, self).sqrt()
    }
    #[inline]
    pub fn normalize(&mut self) {
        *self /= self.length()
    }

    #[inline]
    pub fn normal(self) -> Vector3 {
        self / self.length()
    }

    pub const UP: Vector3 = Vector3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    pub const DOWN: Vector3 = Vector3 {
        x: 0.0,
        y: -1.0,
        z: 0.0,
    };
    pub const RIGHT: Vector3 = Vector3 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const LEFT: Vector3 = Vector3 {
        x: -1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const FORWARD: Vector3 = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };
    pub const BACK: Vector3 = Vector3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };

    pub const ONE: Vector3 = Vector3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    pub const ZERO: Vector3 = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
}

pub type Matrix3x3 = [f32; 9];

// ----------------- Mat4 -----------------------
use std::ops::{Deref, DerefMut};

// This should not be copy!
#[derive(Debug, Clone, Copy)]
pub struct Matrix4x4(pub [f32; 16]);

impl Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    #[inline]
    fn mul(self, b: Self) -> Self {
        Matrix4x4([
            b[0] * self[0] + b[1] * self[4] + b[2] * self[8] + b[3] * self[12],
            b[0] * self[1] + b[1] * self[5] + b[2] * self[9] + b[3] * self[13],
            b[0] * self[2] + b[1] * self[6] + b[2] * self[10] + b[3] * self[14],
            b[0] * self[3] + b[1] * self[7] + b[2] * self[11] + b[3] * self[15],
            b[4] * self[0] + b[5] * self[4] + b[6] * self[8] + b[7] * self[12],
            b[4] * self[1] + b[5] * self[5] + b[6] * self[9] + b[7] * self[13],
            b[4] * self[2] + b[5] * self[6] + b[6] * self[10] + b[7] * self[14],
            b[4] * self[3] + b[5] * self[7] + b[6] * self[11] + b[7] * self[15],
            b[8] * self[0] + b[9] * self[4] + b[10] * self[8] + b[11] * self[12],
            b[8] * self[1] + b[9] * self[5] + b[10] * self[9] + b[11] * self[13],
            b[8] * self[2] + b[9] * self[6] + b[10] * self[10] + b[11] * self[14],
            b[8] * self[3] + b[9] * self[7] + b[10] * self[11] + b[11] * self[15],
            b[12] * self[0] + b[13] * self[4] + b[14] * self[8] + b[15] * self[12],
            b[12] * self[1] + b[13] * self[5] + b[14] * self[9] + b[15] * self[13],
            b[12] * self[2] + b[13] * self[6] + b[14] * self[10] + b[15] * self[14],
            b[12] * self[3] + b[13] * self[7] + b[14] * self[11] + b[15] * self[15],
        ])
    }
}

impl Deref for Matrix4x4 {
    type Target = [f32; 16];

    fn deref(&self) -> &[f32; 16] {
        &self.0
    }
}

impl DerefMut for Matrix4x4 {
    fn deref_mut(&mut self) -> &mut [f32; 16] {
        &mut self.0
    }
}

impl Matrix4x4 {
    pub fn get_translation(&self) -> Vector3 {
        Vector3::new(self[12], self[13], self[14])
    }

    pub fn set_translation(&mut self, translation: Vector3) {
        self[12] = translation.x;
        self[13] = translation.y;
        self[14] = translation.z;
    }

    pub fn scale(&self) -> Vector3 {
        Vector3::new(
            Vector3::new(self[0], self[1], self[2]).length(),
            Vector3::new(self[4], self[5], self[6]).length(),
            Vector3::new(self[8], self[9], self[10]).length(),
        )
    }

    pub const IDENTITY: Matrix4x4 = Matrix4x4([
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]);

    // This may not work for all cases
    // This is not well verified
    pub fn decompose_to_translation_rotation_scale(&self) -> (Vector3, Vector3, Quaternion) {
        let translation = self.get_translation();
        let scale = self.scale();
        let rotation = Matrix4x4([
            self[0] / scale.x,
            self[1] / scale.x,
            self[2] / scale.x,
            0.0,
            self[4] / scale.y,
            self[5] / scale.y,
            self[6] / scale.y,
            0.0,
            self[8] / scale.z,
            self[9] / scale.z,
            self[10] / scale.z,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ]);

        let rotation = Quaternion::from_matrix4x4(&rotation);
        (translation, scale, rotation)
    }
    pub fn inverse(&self) -> Matrix4x4 {
        // Adapted from Raylib
        let a00 = self[0];
        let a01 = self[1];
        let a02 = self[2];
        let a03 = self[3];
        let a10 = self[4];
        let a11 = self[5];
        let a12 = self[6];
        let a13 = self[7];
        let a20 = self[8];
        let a21 = self[9];
        let a22 = self[10];
        let a23 = self[11];
        let a30 = self[12];
        let a31 = self[13];
        let a32 = self[14];
        let a33 = self[15];

        let b00 = a00 * a11 - a01 * a10;
        let b01 = a00 * a12 - a02 * a10;
        let b02 = a00 * a13 - a03 * a10;
        let b03 = a01 * a12 - a02 * a11;
        let b04 = a01 * a13 - a03 * a11;
        let b05 = a02 * a13 - a03 * a12;
        let b06 = a20 * a31 - a21 * a30;
        let b07 = a20 * a32 - a22 * a30;
        let b08 = a20 * a33 - a23 * a30;
        let b09 = a21 * a32 - a22 * a31;
        let b10 = a21 * a33 - a23 * a31;
        let b11 = a22 * a33 - a23 * a32;

        // Calculate the invert determinant (inlined to avoid double-caching)
        let inv_det = 1.0 / (b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06);
        Matrix4x4([
            (a11 * b11 - a12 * b10 + a13 * b09) * inv_det,
            (-a01 * b11 + a02 * b10 - a03 * b09) * inv_det,
            (a31 * b05 - a32 * b04 + a33 * b03) * inv_det,
            (-a21 * b05 + a22 * b04 - a23 * b03) * inv_det,
            (-a10 * b11 + a12 * b08 - a13 * b07) * inv_det,
            (a00 * b11 - a02 * b08 + a03 * b07) * inv_det,
            (-a30 * b05 + a32 * b02 - a33 * b01) * inv_det,
            (a20 * b05 - a22 * b02 + a23 * b01) * inv_det,
            (a10 * b10 - a11 * b08 + a13 * b06) * inv_det,
            (-a00 * b10 + a01 * b08 - a03 * b06) * inv_det,
            (a30 * b04 - a31 * b02 + a33 * b00) * inv_det,
            (-a20 * b04 + a21 * b02 - a23 * b00) * inv_det,
            (-a10 * b09 + a11 * b07 - a12 * b06) * inv_det,
            (a00 * b09 - a01 * b07 + a02 * b06) * inv_det,
            (-a30 * b03 + a31 * b01 - a32 * b00) * inv_det,
            (a20 * b03 - a21 * b01 + a22 * b00) * inv_det,
        ])
    }
}
pub const MAT4_IDENTITY: Matrix4x4 = Matrix4x4([
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
]);

// Adapted from Raylib
// Does not work for projection matrices
pub fn mat4_transform_point(m: &Matrix4x4, v: Point3) -> Vector3 {
    Vector3 {
        x: m[0] * v.x + m[4] * v.y + m[8] * v.z + m[12],
        y: m[1] * v.x + m[5] * v.y + m[9] * v.z + m[13],
        z: m[2] * v.x + m[6] * v.y + m[10] * v.z + m[14],
    }
}

pub fn mat4_transform_direction(m: &Matrix4x4, v: Direction3) -> Vector3 {
    Vector3 {
        x: m[0] * v.x + m[4] * v.y + m[8] * v.z,
        y: m[1] * v.x + m[5] * v.y + m[9] * v.z,
        z: m[2] * v.x + m[6] * v.y + m[10] * v.z,
    }
}

pub fn mat4_transform_ray(m: &Matrix4x4, ray: Ray) -> Ray {
    let origin = mat4_transform_point(m, ray.origin);
    let direction = mat4_transform_direction(m, ray.direction);

    Ray::new(origin, direction)
}

pub fn mat4_perspective_infinite2(
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
    z_near: f32,
) -> Matrix4x4 {
    let rl = right - left;
    let tb = top - bottom;

    let m: Matrix4x4 = Matrix4x4([
        // Column major
        (2.0 * z_near) / rl, // focal length?
        0.0,
        0.0,
        0.0,
        0.0,
        (2.0 * z_near) / tb,
        0.0,
        0.0,
        (right + left) / rl,
        (top + bottom) / tb,
        -1.0,
        -1.0,
        0.0,
        0.0,
        -2.0 * z_near,
        0.0,
    ]);
    m
}

pub fn mat4_perspective_infinite(fovy: f32, ar: f32, z_near: f32, _z_far: f32) -> Matrix4x4 {
    let top = z_near * (fovy / 2.0).tan();
    let bottom = -top;
    let right = top * ar;
    let left = -right;

    mat4_perspective_infinite2(top, bottom, left, right, z_near)
}

pub fn mat4_perspective(fovy: f32, ar: f32, z_near: f32, z_far: f32) -> Matrix4x4 {
    let top = z_near * (fovy / 2.0).tan();
    let bottom = -top;
    let right = top * ar;
    let left = -right;

    let rl = right - left;
    let tb = top - bottom;
    let far_minus_near = z_far - z_near;

    let m: Matrix4x4 = Matrix4x4([
        // Column major
        (2.0 * z_near) / rl,
        0.0,
        0.0,
        0.0,
        0.0,
        (2.0 * z_near) / tb,
        0.0,
        0.0,
        (right + left) / rl,
        (top + bottom) / tb,
        -(z_near + z_far) / far_minus_near,
        -1.0,
        0.0,
        0.0,
        -(2.0 * z_far * z_near) / far_minus_near,
        0.0,
    ]);
    m
}

// Adapted from Ultraviolet:
// https://github.com/termhn/ultraviolet/blob/master/src/projection/rh_yup.rs#L16
pub fn mat4_orthographic(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
) -> Matrix4x4 {
    let rml = right - left;
    let rpl = right + left;
    let tmb = top - bottom;
    let tpb = top + bottom;
    let fmn = far - near;
    let fpn = far + near;
    Matrix4x4([
        2.0 / rml,
        0.0,
        0.0,
        0.0,
        0.0,
        2.0 / tmb,
        0.0,
        0.0,
        0.0,
        0.0,
        -2.0 / fmn,
        0.0,
        -(rpl / rml),
        -(tpb / tmb),
        -(fpn / fmn),
        1.0,
    ])
}

// Adapted from Ultraviolet's: https://github.com/termhn/ultraviolet/blob/05c65be5ac67ae536f8731c515fb83a78d797828/src/mat.rs#L739
pub fn mat4_look_at(eye: Point3, target: Point3, up: Direction3) -> Matrix4x4 {
    let f = (target - eye).normal(); // This is flipped because with OpenGL the camera faces towards the -z
    let r = Vector3::cross(f, up).normal();
    let u = Vector3::cross(r, f).normal();

    let m: Matrix4x4 = Matrix4x4([
        r.x,
        u.x,
        -f.x,
        0.0,
        r.y,
        u.y,
        -f.y,
        0.0,
        r.z,
        u.z,
        -f.z,
        0.0,
        -Vector3::dot(r, eye),
        -Vector3::dot(u, eye),
        -Vector3::dot(f, eye),
        1.0,
    ]);
    m.inverse()
}

pub fn mat4_from_angle_axis(m: &mut Matrix4x4, angle: f32, axis: Vector3) {
    let sinres = angle.sin();
    let cosres = angle.cos();
    let t = 1.0 - cosres;

    let x = axis.x;
    let y = axis.y;
    let z = axis.z;

    m[0] = x * x * t + cosres;
    m[1] = y * x * t + z * sinres;
    m[2] = z * x * t - y * sinres;
    m[3] = 0.0;

    m[4] = x * y * t - z * sinres;
    m[5] = y * y * t + cosres;
    m[6] = z * y * t + x * sinres;
    m[7] = 0.0;

    m[8] = x * z * t + y * sinres;
    m[9] = y * z * t - x * sinres;
    m[10] = z * z * t + cosres;
    m[11] = 0.0;

    m[12] = 0.0;
    m[13] = 0.0;
    m[14] = 0.0;
    m[15] = 1.0;
}

pub fn mat4_scale(x: f32, y: f32, z: f32) -> Matrix4x4 {
    Matrix4x4([
        x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
    ])
}

pub fn mat4_translate(x: f32, y: f32, z: f32) -> Matrix4x4 {
    Matrix4x4([
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, x, y, z, 1.0,
    ])
}

// Adapted from Raylib
pub fn mat4_multiply(a: &Matrix4x4, b: &Matrix4x4) -> Matrix4x4 {
    Matrix4x4([
        a[0] * b[0] + a[1] * b[4] + a[2] * b[8] + a[3] * b[12],
        a[0] * b[1] + a[1] * b[5] + a[2] * b[9] + a[3] * b[13],
        a[0] * b[2] + a[1] * b[6] + a[2] * b[10] + a[3] * b[14],
        a[0] * b[3] + a[1] * b[7] + a[2] * b[11] + a[3] * b[15],
        a[4] * b[0] + a[5] * b[4] + a[6] * b[8] + a[7] * b[12],
        a[4] * b[1] + a[5] * b[5] + a[6] * b[9] + a[7] * b[13],
        a[4] * b[2] + a[5] * b[6] + a[6] * b[10] + a[7] * b[14],
        a[4] * b[3] + a[5] * b[7] + a[6] * b[11] + a[7] * b[15],
        a[8] * b[0] + a[9] * b[4] + a[10] * b[8] + a[11] * b[12],
        a[8] * b[1] + a[9] * b[5] + a[10] * b[9] + a[11] * b[13],
        a[8] * b[2] + a[9] * b[6] + a[10] * b[10] + a[11] * b[14],
        a[8] * b[3] + a[9] * b[7] + a[10] * b[11] + a[11] * b[15],
        a[12] * b[0] + a[13] * b[4] + a[14] * b[8] + a[15] * b[12],
        a[12] * b[1] + a[13] * b[5] + a[14] * b[9] + a[15] * b[13],
        a[12] * b[2] + a[13] * b[6] + a[14] * b[10] + a[15] * b[14],
        a[12] * b[3] + a[13] * b[7] + a[14] * b[11] + a[15] * b[15],
    ])
}

// Adapted from GLM
// Should not be inlined
#[allow(non_snake_case)]
pub fn mat4_inverse_transpose(r: &mut Matrix4x4, m: &Matrix4x4) {
    let SubFactor00 = m[10] * m[15] - m[14] * m[11];
    let SubFactor01 = m[9] * m[15] - m[13] * m[11];
    let SubFactor02 = m[9] * m[14] - m[13] * m[10];
    let SubFactor03 = m[8] * m[15] - m[12] * m[11];
    let SubFactor04 = m[8] * m[14] - m[12] * m[10];
    let SubFactor05 = m[8] * m[13] - m[12] * m[9];
    let SubFactor06 = m[6] * m[15] - m[14] * m[7];
    let SubFactor07 = m[5] * m[15] - m[13] * m[7];
    let SubFactor08 = m[5] * m[14] - m[13] * m[6];
    let SubFactor09 = m[4] * m[15] - m[12] * m[7];
    let SubFactor10 = m[4] * m[14] - m[12] * m[6];
    let SubFactor11 = m[4] * m[13] - m[12] * m[5];
    let SubFactor12 = m[6] * m[11] - m[10] * m[7];
    let SubFactor13 = m[5] * m[11] - m[9] * m[7];
    let SubFactor14 = m[5] * m[10] - m[9] * m[6];
    let SubFactor15 = m[4] * m[11] - m[8] * m[7];
    let SubFactor16 = m[4] * m[10] - m[8] * m[6];
    let SubFactor17 = m[4] * m[9] - m[8] * m[5];

    let Inverse = [
        m[5] * SubFactor00 - m[6] * SubFactor01 + m[7] * SubFactor02,
        -(m[4] * SubFactor00 - m[6] * SubFactor03 + m[7] * SubFactor04),
        m[4] * SubFactor01 - m[5] * SubFactor03 + m[7] * SubFactor05,
        -(m[4] * SubFactor02 - m[5] * SubFactor04 + m[6] * SubFactor05),
        -(m[1] * SubFactor00 - m[2] * SubFactor01 + m[3] * SubFactor02),
        m[0] * SubFactor00 - m[2] * SubFactor03 + m[3] * SubFactor04,
        -(m[0] * SubFactor01 - m[1] * SubFactor03 + m[3] * SubFactor05),
        m[0] * SubFactor02 - m[1] * SubFactor04 + m[2] * SubFactor05,
        m[1] * SubFactor06 - m[2] * SubFactor07 + m[3] * SubFactor08,
        -(m[0] * SubFactor06 - m[2] * SubFactor09 + m[3] * SubFactor10),
        m[0] * SubFactor07 - m[1] * SubFactor09 + m[3] * SubFactor11,
        -(m[0] * SubFactor08 - m[1] * SubFactor10 + m[2] * SubFactor11),
        -(m[1] * SubFactor12 - m[2] * SubFactor13 + m[3] * SubFactor14),
        m[0] * SubFactor12 - m[2] * SubFactor15 + m[3] * SubFactor16,
        -(m[0] * SubFactor13 - m[1] * SubFactor15 + m[3] * SubFactor17),
        m[0] * SubFactor14 - m[1] * SubFactor16 + m[2] * SubFactor17,
    ];

    let Determinant = m[0] * Inverse[0] + m[1] * Inverse[1] + m[2] * Inverse[2] + m[3] * Inverse[3];

    r[0] = Inverse[0] / Determinant;
    r[1] = Inverse[1] / Determinant;
    r[2] = Inverse[2] / Determinant;
    r[3] = Inverse[3] / Determinant;

    r[4] = Inverse[4] / Determinant;
    r[5] = Inverse[5] / Determinant;
    r[6] = Inverse[6] / Determinant;
    r[7] = Inverse[7] / Determinant;

    r[8] = Inverse[8] / Determinant;
    r[9] = Inverse[9] / Determinant;
    r[10] = Inverse[10] / Determinant;
    r[11] = Inverse[11] / Determinant;

    r[12] = Inverse[12] / Determinant;
    r[13] = Inverse[13] / Determinant;
    r[14] = Inverse[14] / Determinant;
    r[15] = Inverse[15] / Determinant;
}

pub fn mat4_from_rotation_translation(rotation: Quaternion, translation: Vector3) -> Matrix4x4 {
    let q = rotation.to_matrix4x4();
    let t = mat4_translate(translation.x, translation.y, translation.z);
    t * q
}

// Translation, rotation, scale
pub fn mat4_from_trs(translation: Vector3, rotation: Quaternion, scale: Vector3) -> Matrix4x4 {
    let t = mat4_translate(translation.x, translation.y, translation.z);
    let r = rotation.to_matrix4x4();
    let s = mat4_scale(scale.x, scale.y, scale.z);
    t * r * s
}

// Quaternion stuff
#[derive(Debug, Copy, Clone)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&mut self) {
        let length = self.length();
        self.x /= length;
        self.y /= length;
        self.z /= length;
        self.w /= length;
    }

    pub fn rotate_vector(&self, v: Vector3) -> Vector3 {
        let quat_vector = Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        };
        let uv = Vector3::cross(quat_vector, v);
        let uuv = Vector3::cross(quat_vector, uv);

        v + ((uv * self.w) + uuv) * 2.0
    }

    pub fn up(&self) -> Vector3 {
        self.rotate_vector(Vector3::UP)
    }

    pub fn forward(&self) -> Vector3 {
        self.rotate_vector(Vector3::FORWARD)
    }

    pub fn back(&self) -> Vector3 {
        self.rotate_vector(Vector3::BACK)
    }

    pub fn left(&self) -> Vector3 {
        self.rotate_vector(Vector3::LEFT)
    }

    pub fn right(&self) -> Vector3 {
        self.rotate_vector(Vector3::RIGHT)
    }

    pub fn from_angle_axis(angle: f32, axis: Vector3) -> Quaternion {
        let a = angle * 0.5;
        let sinres = a.sin();
        let cosres = a.cos();

        let mut q = Quaternion {
            x: axis.x * sinres,
            y: axis.y * sinres,
            z: axis.z * sinres,
            w: cosres,
        };
        q.normalize();
        q
    }

    // Yaw and pitched swapped from Wikipedia article, is wiki wrong or is there something I don't understand?
    // roll and pitch is swapped from article too.
    pub fn from_euler_angles_xyz(roll: f32, yaw: f32, pitch: f32) -> Quaternion {
        let cx = (roll * 0.5).cos();
        let sx = (roll * 0.5).sin();
        let cy = (yaw * 0.5).cos();
        let sy = (yaw * 0.5).sin();
        let cz = (pitch * 0.5).cos();
        let sz = (pitch * 0.5).sin();
        Quaternion {
            x: sx * cy * cz + cx * sy * sz,
            y: cx * sy * cz - sx * cy * sz,
            z: cx * cy * sz + sx * sy * cz,
            w: cx * cy * cz - sx * sy * sz,
        }
    }

    pub fn to_matrix4x4(&self) -> Matrix4x4 {
        let mut q = *self;
        q.normalize();
        let qxx = q.x * q.x;
        let qyy = q.y * q.y;
        let qzz = q.z * q.z;
        let qxz = q.x * q.z;
        let qxy = q.x * q.y;
        let qyz = q.y * q.z;
        let qwx = q.w * q.x;
        let qwy = q.w * q.y;
        let qwz = q.w * q.z;

        Matrix4x4([
            1.0 - (2.0 * (qyy + qzz)),
            2.0 * (qxy + qwz),
            2.0 * (qxz - qwy),
            0.0,
            2.0 * (qxy - qwz),
            1.0 - (2.0 * (qxx + qzz)),
            2.0 * (qyz + qwx),
            0.0,
            2.0 * (qxz + qwy),
            2.0 * (qyz - qwx),
            1.0 - (2.0 * (qxx + qyy)),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ])
    }
    // This is not fully verified.
    // Adapted from Raylib:
    // https://github.com/raysan5/raylib/blob/master/src/raymath.h#L1193
    pub fn from_matrix4x4(m: &Matrix4x4) -> Self {
        let t = m[0] + m[5] + m[10] + m[15]; // Trace

        let (x, y, z, w) = if t > 0.000_000_01 {
            let s = f32::sqrt(t) * 2.0;
            let inv_s = 1.0 / s;
            let x = (m[6] - m[9]) * inv_s;
            let y = (m[8] - m[2]) * inv_s;
            let z = (m[1] - m[4]) * inv_s;
            let w = 0.25 * s;
            (x, y, z, w)
        } else if m[0] > m[5] && m[0] > m[10] {
            let s = f32::sqrt(1.0 + m[0] - m[5] - m[10]) * 2.0;
            let inv_s = 1.0 / s;

            let x = 0.25 * s;
            let y = (m[1] + m[4]) * inv_s;
            let z = (m[8] + m[2]) * inv_s;
            let w = (m[6] - m[9]) * inv_s;
            (x, y, z, w)
        } else if m[5] > m[10] {
            let s = f32::sqrt(1.0 + m[5] - m[0] - m[10]) * 2.0;
            let inv_s = 1.0 / s;

            let x = (m[1] + m[4]) * inv_s;
            let y = 0.25 * s;
            let z = (m[6] + m[9]) * inv_s;
            let w = (m[8] - m[2]) * inv_s;
            (x, y, z, w)
        } else {
            let s = f32::sqrt(1.0 + m[10] - m[0] - m[5]) * 2.0;
            let inv_s = 1.0 / s;

            let x = (m[8] + m[2]) * inv_s;
            let y = (m[6] + m[9]) * inv_s;
            let z = 0.25 * s;
            let w = (m[1] - m[4]) * inv_s;
            (x, y, z, w)
        };

        Self { x, y, z, w }
    }

    pub const IDENTITY: Quaternion = Quaternion {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };
}

impl Neg for Quaternion {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul for Quaternion {
    type Output = Quaternion;

    // Adapted from Raylib
    #[inline]
    fn mul(self, rhs: Quaternion) -> Quaternion {
        let qax = self.x;
        let qay = self.y;
        let qaz = self.z;
        let qaw = self.w;
        let qbx = rhs.x;
        let qby = rhs.y;
        let qbz = rhs.z;
        let qbw = rhs.w;

        Quaternion {
            x: qax * qbw + qaw * qbx + qay * qbz - qaz * qby,
            y: qay * qbw + qaw * qby + qaz * qbx - qax * qbz,
            z: qaz * qbw + qaw * qbz + qax * qby - qay * qbx,
            w: qaw * qbw - qax * qbx - qay * qby - qaz * qbz,
        }
    }
}
