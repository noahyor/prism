use std::{fs::File, io::{BufWriter, Write}, sync::{Arc, Mutex}};

use crate::color::Pixel;

pub struct ImgWriter {
    writer: BufWriter<File>,
    done: Vec<Pixel>,
    img_width: usize,
}

impl ImgWriter {
    pub fn new(mut writer: BufWriter<File>, img_width: usize, img_height: usize) -> Self {
        writer.write_all(
            format!("P3\n{} {}\n255\n", img_width, img_height).as_bytes()
        ).unwrap();
        ImgWriter { writer, done: Vec::new(), img_width }
    }

    pub fn write(&mut self, pixel: Pixel) {
        self.done.push(pixel);
    }

    pub fn flush(&mut self) {
        self.finish();
        self.writer.flush().unwrap()
    }

    fn finish(&mut self) {
        self.done.sort_by(|a, b| a.index(self.img_width).cmp(&b.index(self.img_width)));
        for pixel in &self.done {
            pixel.color.write(&mut self.writer);
        }
    }
}

pub struct DebugWriter {
    handle: BufWriter<File>,
}

impl DebugWriter {
    pub fn new(filename: &str) -> Self {
        Self {
            handle: BufWriter::new(File::create(filename).unwrap()),
        }
    }

    pub fn write<T: ToString>(&mut self, value: T) {
        self.handle.write_all(value.to_string().as_bytes()).unwrap()
    }

    pub fn writeln<T: ToString>(&mut self, value: T) {
        let mut string = value.to_string();
        string.push('\n');
        self.handle.write_all(string.as_bytes()).unwrap()
    }
}

#[derive(Clone)]
pub struct Debugger {
    writer: Arc<Mutex<DebugWriter>>
}

impl Debugger {
    pub fn new(filename: &str) -> Self {
        Self {
            writer: Arc::new(Mutex::new(DebugWriter::new(filename)))
        }
    }

    pub fn write<T: ToString>(&mut self, value: T) {
        self.writer.lock().unwrap().write(value);
    }

    pub fn writeln<T: ToString>(&mut self, value: T) {
        self.writer.lock().unwrap().writeln(value);
    }
}