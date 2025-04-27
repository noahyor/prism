use std::{fs::File, io::{BufWriter, Write}, ops::{Add, AddAssign, Div, Mul, Sub}};

use rand::random;

use crate::{interval::Interval, vector::Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

pub struct Pixel {
    pub color: Color,
    pub j: usize,
    pub i: usize,
}

impl Pixel {
    pub fn index(&self, img_width: usize) -> usize {
        self.i + self.j * img_width
    }
}

impl Color {
    pub fn write(self, writer: &mut BufWriter<File>) {
        let intensity = Interval {min: 0.0, max: 0.999};
        let r_int: u8 = (256.0 * 
            intensity.clamp(linear_to_gamma(self.r))
        ) as u8;
        let g_int: u8 = (256.0 * 
            intensity.clamp(linear_to_gamma(self.g))
        ) as u8;
        let b_int: u8 = (256.0 * 
            intensity.clamp(linear_to_gamma(self.b))
        ) as u8;
        
        writer.write_all(
            format!("{} {} {}\n", r_int, g_int, b_int).as_bytes()
        ).unwrap();
    }

    pub fn from_all(n: f64) -> Self {
        Color { r: n, g: n, b: n }
    }

    pub fn random() -> Self {
        Color {
            r: random(),
            g: random(),
            b: random(),
        }
    }

    pub fn random_with(min: f64, max: f64) -> Self {
        Self::random()*(max - min) + Self::from_all(min)
    }

    pub const WHITE: Color = Color {r: 1.0, g: 1.0, b: 1.0};
    pub const BLACK: Color = Color {r: 0.0, g: 0.0, b: 0.0};
    pub const CYAN : Color = Color {r: 0.0, g: 1.0, b: 1.0};
}

fn linear_to_gamma(component: f64) -> f64 {
    if component > 0.0 {component.sqrt()} else {0.0}
}

impl From<Vector3> for Color {
    fn from(value: Vector3) -> Self {
        Color { r: value.x(), g: value.y(), b: value.z() }
    }
}

impl Into<Vector3> for Color {
    fn into(self) -> Vector3 {
        Vector3(self.r, self.g, self.b)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<f64> for &Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Div<f64> for &Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}