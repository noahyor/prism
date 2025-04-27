use std::{collections::VecDeque, fs::File, io::BufWriter, sync::{Arc, Mutex}};

use indicatif::{ProgressBar, ProgressStyle};

use crate::{bvh::BVHNode, color::{Color, Pixel}, hit::Hittable, interval::Interval, ray::{Ray, SimdRay}, scene::Scene, simd::SimdVec, vector::Vector3, writer::{Debugger, ImgWriter}, FILE_OUT};

pub trait Renderer {
    fn render(&self, scene: Scene);
}

pub struct DefaultRenderer;

impl DefaultRenderer {
    pub fn new() -> Box<dyn Renderer> {
        Box::new(Self{})
    }
}

impl Renderer for DefaultRenderer {
    fn render(&self, scene: Scene) {
        let debugger = Arc::new(Mutex::new(Debugger::new("debug.txt")));

        let progress = ProgressBar::new_spinner().with_message("Generating Rays...");
        let mut rays = VecDeque::new();

        // Generate rays to evaluate
        for j in 0..(scene.camera.img_height) {
            for i in 0..(scene.camera.img_width) {
                for _ in 0..(scene.camera.samples_per_pixel) {
                    rays.push_back(scene.camera.get_ray(i, j));
                }
            }
        }
        let mut pack_rays = SimdVec::new();
        for chunk in 0..(rays.len() / 64) {
            if rays.len() < 64 {
                pack_rays.push_residue(rays.into());
                break;
            }
            let mut array = [Ray {origin: Vector3::new(), dir: Vector3::new(), time: 0.0}; 64];
            for index in 0..64 {
                array[index] = rays.pop_front().unwrap();
            }
            let simd = SimdRay::from_array(array);
            pack_rays.push(simd);
        }
        
        // for j in 0..(scene.camera.img_height) {
        //     for i in 0..(scene.camera.img_width) {
        //         assert_eq!(scene.camera.threadpool.panic_count(), 0);
        //         let this = scene.camera.clone();
        //         let progress = pixel_progress.clone();
        //         let writer = writer.clone();
        //         let root = scene.root.clone();
        //         let debugger = debugger.clone();
        //         scene.camera.threadpool.execute(move || {
        //             let mut color = Color::BLACK;
        //             for _ in 0..(this.samples_per_pixel) {
        //                 let ray = this.get_ray(i, j);
        //                 color += ray.color(this.max_depth, &*root, debugger.clone());
        //             }
        //             let color = color * this.pixel_samples_scale;
        //             let pixel = Pixel {color, i, j};
        //             writer.lock().unwrap().write(pixel);

        //             progress.lock().unwrap().inc(1);
        //         });
        //     }
        // }
    }
}

pub struct ScreenUV;

impl ScreenUV {
    pub fn new() -> Box<dyn Renderer> {
        Box::new(Self{})
    }
}

impl Renderer for ScreenUV {
    fn render(&self, scene: Scene) {
        let debugger = Arc::new(Mutex::new(Debugger::new("debug.txt")));
        let file = File::create(FILE_OUT).unwrap();
        let writer = Arc::new(Mutex::new(ImgWriter::new(
            BufWriter::new(file),
            scene.camera.img_width,
            scene.camera.img_height,
        )));
        for j in 0..(scene.camera.img_height) {
            for i in 0..(scene.camera.img_width) {
                writer.lock().unwrap().write(Pixel { color: Color { r: i as f64 / scene.camera.img_width as f64, g: j as f64 / scene.camera.img_height as f64, b: 0.0 }, j, i });
            }
        }
        writer.lock().unwrap().flush();
    }
}

pub struct UV;

impl UV {
    pub fn new() -> Box<dyn Renderer> {
        Box::new(Self{})
    }

    fn color(ray: &Ray, root: Arc<BVHNode>, debugger: Arc<Mutex<Debugger>>) -> Color {
        match root.hit(ray, &Interval {min: 0.001, max: f64::INFINITY}) {
            Some(hit) => {
                Color {r: hit.u, g: hit.v, b: 0.0}
            }
            None => {
                Color {r: 0.0, g: 0.0, b: 1.0}
            },
        }
    }
}

impl Renderer for UV {
    fn render(&self, scene: Scene) {
        let debugger = Arc::new(Mutex::new(Debugger::new("debug.txt")));
        let file = File::create(FILE_OUT).unwrap();
        let writer = Arc::new(Mutex::new(ImgWriter::new(
            BufWriter::new(file),
            scene.camera.img_width,
            scene.camera.img_height,
        )));
        let pixel_progress = Arc::new(Mutex::new(
            ProgressBar::new((scene.camera.img_height*scene.camera.img_width) as u64)
                .with_style(ProgressStyle::with_template("{human_pos}/{len} {wide_bar:.green} {elapsed}").unwrap())
        ));
        
        for j in 0..(scene.camera.img_height) {
            for i in 0..(scene.camera.img_width) {
                assert_eq!(scene.camera.threadpool.panic_count(), 0);
                let this = scene.camera.clone();
                let progress = pixel_progress.clone();
                let writer = writer.clone();
                let root = scene.root.clone();
                let debugger = debugger.clone();
                scene.camera.threadpool.execute(move || {
                    let mut color = Color::BLACK;
                    for _ in 0..(this.samples_per_pixel) {
                        let ray = this.get_ray(i, j);
                        color += Self::color(&ray, root.clone(), debugger.clone());
                    }
                    let color = color * this.pixel_samples_scale;
                    let pixel = Pixel {color, i, j};
                    writer.lock().unwrap().write(pixel);

                    progress.lock().unwrap().inc(1);
                });
            }
            scene.camera.threadpool.join();
            
        }
        pixel_progress.lock().unwrap().finish();
        writer.lock().unwrap().flush();
    }
}