use std::sync::{Arc, Mutex};

use crate::{color::Color, image::{self, ImgData}, vector::Point3, writer::Debugger};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3, debugger: Arc<Mutex<Debugger>>) -> Color;
}

pub struct SolidTexture {
    albedo: Color,
}

impl SolidTexture {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo
        }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self { albedo: Color { r, g, b } }
    }

    pub fn to_box(self) -> Box<dyn Texture> {
        Box::new(self)
    }
}

impl Texture for SolidTexture {
    fn value(&self, _u: f64, _v: f64, _p: &Point3, _debugger: Arc<Mutex<Debugger>>) -> Color {
        self.albedo
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Box<dyn Texture>, odd: Box<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn from_const_col(scale: f64, even: Color, odd: Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: SolidTexture::new(even).to_box(),
            odd: SolidTexture::new(odd).to_box(),
        }
    }

    pub fn to_box(self) -> Box<dyn Texture> {
        Box::new(self)
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3, debugger: Arc<Mutex<Debugger>>) -> Color {
        let x = (self.inv_scale * p.x()).floor() as isize;
        let y = (self.inv_scale * p.y()).floor() as isize;
        let z = (self.inv_scale * p.z()).floor() as isize;

        let is_even = (x + y + z) % 2 == 0;

        if is_even {self.even.value(u, v, p, debugger)} else {self.odd.value(u, v, p, debugger)}
    }
}

pub struct ImageTexture {
    image: ImgData,
}

impl ImageTexture {
    pub fn new(file: String) -> Self {
        Self {
            image: image::get_img(file),
        }
    }

    pub fn to_box(self) -> Box<dyn Texture> {
        Box::new(self)
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3, debugger: Arc<Mutex<Debugger>>) -> Color {
        if u > 1.0 || u < 0.0 || v > 1.0 || v < 0.0 {
            return Color::CYAN;
        };

        let v = 1.0 - v;

        let i = (u * self.image.width() as f64) as usize;
        let j = (v * self.image.width() as f64) as usize;
        self.image.pixel(i, j, debugger)
    }
}