use std::sync::{Arc, Mutex};

use crate::{color::Color, texture::Texture, vector::Point3, writer::Debugger};

pub struct DebugTexture {
    debug_type: DebugType,
}

pub enum DebugType {
    UV,
}

impl DebugTexture {
    pub fn new(debug_type: DebugType) -> Self {
        Self { debug_type }
    }

    pub fn to_dyn(self) -> Box<dyn Texture> {
        Box::new(self)
    }
}

impl Texture for DebugTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3, _debugger: Arc<Mutex<Debugger>>) -> Color {
        Color { r: u, g: v, b: 0.0 }
    }
}