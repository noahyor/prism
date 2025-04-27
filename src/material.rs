use std::sync::{Arc, Mutex};

use crate::{color::Color, hit::{Hit, Hittable}, ray::Ray, texture::{SolidTexture, Texture}, vector::Vector3, writer::Debugger};

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, hit: &Hit, debugger: Arc<Mutex<Debugger>>) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    texture: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(texture: Box<dyn Texture>) -> Self {
        Self {
            texture,
        }
    }

    pub fn from_const_col(albedo: Color) -> Self {
        Self {
            texture: SolidTexture::new(albedo).to_box()
        }
    }
    
    pub fn to_dyn(self) -> Arc<Box<dyn Material>> {
        Arc::new(Box::new(self))
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hit: &Hit, debugger: Arc<Mutex<Debugger>>) -> Option<(Color, Ray)> {
        let mut dir = hit.normal + Vector3::random_unit();
        if dir.is_near_zero() {dir = hit.normal};
        let scatter = Ray {origin: hit.p, dir, time: r_in.time};
        let attenuation = self.texture.value(hit.u, hit.v, &hit.p, debugger);
        Some((attenuation, scatter))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn from(r: f64, g: f64, b: f64, fuzz: f64) -> Self {
        Metal { albedo: Color { r, g, b }, fuzz }
    }
    
    pub fn to_dyn(self) -> Arc<Box<dyn Material>> {
        Arc::new(Box::new(self))
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &Hit, _debugger: Arc<Mutex<Debugger>>) -> Option<(Color, Ray)> {
        let dir = r_in.dir.reflect(&hit.normal);
        let dir = dir.unit() + (Vector3::random_unit() * self.fuzz);
        let scatter = Ray {origin: hit.p, dir, time: r_in.time};
        let attenuation = self.albedo;
        if scatter.dir.dot(&hit.normal) > 0.0 {
            Some((attenuation, scatter))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub index: f64,
}

impl Dielectric {
    pub fn from(index: f64) -> Self {
        Dielectric { index }
    }
    
    pub fn to_dyn(self) -> Arc<Box<dyn Material>> {
        Arc::new(Box::new(self))
    }

    fn reflectance(&self, cosine: f64, index: f64) -> f64 {
        let r0 = (1.0 - index) / (1.0 + index);
        let r0 = r0 * r0;
        r0 + (1.0-r0)*(1.0-cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &Hit, debugger: Arc<Mutex<Debugger>>) -> Option<(Color, Ray)> {
        let ri = if hit.front_face {1.0/self.index} else {self.index};
        let unit_dir = r_in.dir.unit();
        let cos_theta = -unit_dir.dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
        let refract = ri * sin_theta <= 1.0;
        let dir = if
            !refract || self.reflectance(cos_theta, ri) > rand::random()
        {
            unit_dir.reflect(&hit.normal)
        } else {
            unit_dir.refract(&hit.normal, ri)
        };
        let scattered = Ray { origin: hit.p, dir, time: r_in.time };
        Some((Color::WHITE, scattered))
    }
}

pub struct Portal {
    linked: Arc<Portal>,
    parent: std::sync::Weak<Box<dyn Hittable>>,
}

impl Portal {
    
}