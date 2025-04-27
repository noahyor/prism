use std::{fs::File, sync::{Arc, Mutex}};

use png::{Decoder, OutputInfo};

use crate::{color::Color, writer::Debugger};

pub fn get_img(file: String) -> ImgData {
    let decoder = Decoder::new(
        File::open(file).unwrap()
    );
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    ImgData { buf, info }
}

pub struct ImgData {
    buf: Vec<u8>,
    info: OutputInfo,
}

impl ImgData {
    pub fn pixel(&self, x: usize, y: usize, debugger: Arc<Mutex<Debugger>>) -> Color {
        let index = self.deinterlace(x, y);
        // let index = (x + y * self.height()) * 3;

        let (r, g, b) =
            (self.buf[index], self.buf[index + 1], self.buf[index + 2]);
        let (r, g, b) = (r as f64, g as f64, b as f64);
        let (r, g, b) = (r/256.0, g/256.0, b/256.0);

        Color { r, g, b }
    }

    pub fn width(&self) -> usize {
        self.info.width as usize
    }

    pub fn height(&self) -> usize {
        self.info.height as usize
    }

    fn deinterlace(&self, x: usize, y: usize) -> usize {
        ((match y % 2 {
            0 => x,
            1 => (x + self.height()) % (self.height() * 2),
            _ => unreachable!(),
        }) + (y * self.height())) * 3
    }
}