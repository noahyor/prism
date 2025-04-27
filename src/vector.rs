use std::{ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub}, simd::{LaneCount, Mask, Simd, SupportedLaneCount}};

use rand::random;

use crate::{color::Color, simd::Maskish};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3(pub f64, pub f64, pub f64);

pub struct SimdVector3<const N: usize> (pub Simd<f64, N>, pub Simd<f64, N>, pub Simd<f64, N>) where LaneCount<N>: SupportedLaneCount;

impl<const N: usize> SimdVector3<N>
    where LaneCount<N>: SupportedLaneCount {
    pub fn from_array(array: [Vector3; N]) -> Self {
        let mut x = [0.0; N];
        let mut y = [0.0; N];
        let mut z = [0.0; N];
        for idx in 0..N {
            x[idx] = array[idx].x();
            y[idx] = array[idx].y();
            z[idx] = array[idx].z();
        }
        SimdVector3(Simd::from_array(x), Simd::from_array(y), Simd::from_array(z))
    }
}

pub type Point3 = Vector3;

pub type SimdPoint3<const N: usize>     where LaneCount<N>: SupportedLaneCount = SimdVector3<N>;

impl Vector3 {
    pub fn new() -> Self {
        Vector3 (0.0, 0.0, 0.0)
    }

    pub fn from(x: f64, y: f64, z: f64) -> Self {
        Vector3 (x, y, z)
    }

    pub fn from_all(n: f64) -> Self {
        Vector3(n, n, n)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        (self.length_squared()).sqrt()
    }
    
    pub fn length_squared(&self) -> f64 {
        self.dot(&self)
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        (self.x() * rhs.x()) + (self.y() * rhs.y()) + (self.z() * rhs.z())
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vector3 (
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }

    pub fn unit(&self) -> Self {
        self / self.length()
    }

    pub fn random() -> Self {
        Vector3 (
            random(),
            random(),
            random(),
        )
    }

    pub fn random_with(min: f64, max: f64) -> Self {
        Self::random()*(max - min) + Self::from_all(min)
    }

    pub fn random_unit() -> Self {
        loop {
            let p = Self::random_with(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {return p / lensq.sqrt();}
        }
    }

    pub fn random_unit_disc() -> Self {
        loop {
            let p = Vector3::random() * -2.0 - Vector3::from_all(-1.0);
            if p.length_squared() < 1.0 {return p;}
        }
    }

    pub fn random_on(normal: &Vector3) -> Self {
        let on_sphere = Self::random_unit();
        if on_sphere.dot(normal) > 0.0 {
            on_sphere
        } else {
            -on_sphere
        }
    }

    pub fn is_near_zero(&self) -> bool {
        let s = 1e-8;
        self.x().abs() < s && self.y().abs() < s && self.z().abs() < s
    }

    pub fn reflect(&self, normal: &Vector3) -> Self {
        *self - normal*2.0*self.dot(normal)
    }

    pub fn refract(&self, normal: &Vector3, etas: f64) -> Self {
        let cos_theta = -self.dot(normal).min(1.0);
        let r_out_perp = (*self + normal * cos_theta) * etas;
        let r_out_parallel =
            normal * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
        r_out_perp + r_out_parallel
    }
}

impl<const N: usize> SimdVector3<N>
    where LaneCount<N>: SupportedLaneCount {
    pub fn x(&self) -> Simd<f64, N> {
        self.0
    }

    pub fn y(&self) -> Simd<f64, N> {
        self.1
    }

    pub fn z(&self) -> Simd<f64, N> {
        self.2
    }
}

#[derive(Clone, Copy)]
pub struct SimdVector3Mask<const N: usize> (pub Mask<i64, N>, pub Mask<i64, N>, pub Mask<i64, N>) where LaneCount<N>: SupportedLaneCount;

impl<const N: usize> Maskish for SimdVector3Mask<N>
    where LaneCount<N>: SupportedLaneCount {
    
    fn replace(self, replace: Self, mask: Self) -> Self {
        todo!()
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3 (
            -self.x(),
            -self.y(),
            -self.z(),
        )
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 (
            self.x() + rhs.x(),
            self.y() + rhs.y(),
            self.z() + rhs.z(),
        )
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 (
            self.x() - rhs.x(),
            self.y() - rhs.y(),
            self.z() - rhs.z(),
        )
    }
}

impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector3 (
            self.x() * rhs.x(),
            self.y() * rhs.y(),
            self.z() * rhs.z(),
        )
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3 (
            self.x() * rhs,
            self.y() * rhs,
            self.z() * rhs,
        )
    }
}

impl Mul<f64> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3 (
            self.x() * rhs,
            self.y() * rhs,
            self.z() * rhs,
        )
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Div<f64> for &Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.x();
        self.1 += rhs.y();
        self.2 += rhs.z();
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Add<Color> for Vector3 {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self.x() + rhs.r,
            g: self.y() + rhs.g,
            b: self.z() + rhs.b,
        }
    }
}

impl<const N: usize> Div<SimdVector3<N>> for f64
    where LaneCount<N>: SupportedLaneCount {
    type Output = SimdVector3<N>;

    fn div(self, rhs: SimdVector3<N>) -> Self::Output {
        let simd = Simd::from_array([self; N]);
        SimdVector3(
            simd / rhs.x(),
            simd / rhs.y(),
            simd / rhs.z(),
        )
    }
}