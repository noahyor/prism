use rand::random;
use threadpool::ThreadPool;

use crate::{quality::QualityOptions, ray::Ray, util::degrees_to_radians, vector::{Point3, Vector3}};

#[derive(Clone)]
pub struct Camera {
    // "Public"
    pub img_width: usize,
    pub pos: Point3,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    // lookat: Point3,
    // vup: Vector3,
    defocus_angle: f64,
    // focus_dist: f64,

    // "Private"
    pub img_height: usize,
    pub pixel_samples_scale: f64,
    pub pixel00: Point3,
    pixel_du: Vector3,
    pixel_dv: Vector3,
    // u: Vector3,
    // v: Vector3,
    // w: Vector3,
    defocus_u: Vector3,
    defocus_v: Vector3,

    // Other
    pub threadpool: ThreadPool
}

impl Camera {
    pub fn new(
        quality: QualityOptions,
        img_aspect: f64,
        pos: Point3,
        vert_fov: f64,
        lookat: Point3,
        vup: Vector3,
        defocus_angle: f64,
        focus_dist: f64,
        num_threads: usize,
    ) -> Self {
        let img_height: usize = (quality.img_width as f64 / img_aspect) as usize;

        assert!(img_height > 1);

        let pixel_samples_scale = 1.0 / quality.samples_per_pixel as f64;

        let theta = degrees_to_radians(vert_fov);
        let h = (theta/2.0).tan();
        let view_height = 2.0 * h * focus_dist;
        let view_width = view_height * (quality.img_width as f64 / img_height as f64);

        let w = (pos - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);
        
        let view_u = u * view_width;
        let view_v = -v * view_height;

        let pixel_du = view_u / quality.img_width as f64;
        let pixel_dv = view_v / img_height as f64;

        let view_upper_left =
            pos - (w * focus_dist) - view_u/2.0 - view_v/2.0;
        let pixel00 = view_upper_left + (pixel_du + pixel_dv) * 0.5;

        let defocus_radius =
            focus_dist * (degrees_to_radians(defocus_angle / 2.0)).tan();
        let defocus_u = u * defocus_radius;
        let defocus_v = v * defocus_radius;

        let threadpool = ThreadPool::new(num_threads);

        Camera {
            // "Public"
            img_width: quality.img_width,
            pos,
            samples_per_pixel: quality.samples_per_pixel,
            max_depth: quality.max_depth,
            // lookat,
            // vup,
            defocus_angle,
            // focus_dist,
            
            // "Private"
            img_height,
            pixel_samples_scale,
            pixel00,
            pixel_du,
            pixel_dv,
            // u, v, w,
            defocus_u,
            defocus_v,

            // Other
            threadpool,
        }
    }

    pub fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00
            +  self.pixel_du * (i as f64 + offset.x())
            +  self.pixel_dv * (j as f64 + offset.y());

        let origin = if self.defocus_angle <= 0.0
            {self.pos} else {self.defocus_sample()};
        let dir = pixel_sample - origin;
        let time = random();

        Ray { origin, dir, time }
    }

    fn sample_square(&self) -> Vector3 {
        Vector3 (
            rand::random::<f64>() - 0.5,
            rand::random::<f64>() - 0.5,
            0.0
        )
    }

    fn defocus_sample(&self) -> Vector3 {
        let p = Vector3::random_unit_disc();
        self.pos + (self.defocus_u * p.x()) + (self.defocus_v * p.y())
    }

    pub fn total_pixels(&self) -> usize {
        self.img_height * self.img_width
    }
}

pub struct CameraBuilder {
    quality: QualityOptions,
    img_aspect: f64,
    pos: Point3,
    vert_fov: f64,
    lookat: Point3,
    vup: Vector3,
    defocus_angle: f64,
    focus_dist: f64,
    num_threads: usize,
}

impl CameraBuilder {
    pub fn new() -> Self {
        Self {
            quality: QualityOptions::DEFAULT,
            img_aspect: 16.0/9.0,
            pos: Vector3(0.0, 0.0, 0.0),
            vert_fov: 20.0,
            lookat: Vector3(1.0, 0.0, 0.0),
            vup: Vector3(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            num_threads: 8,
        }
    }

    pub fn build(self) -> Camera {
        Camera::new(
            self.quality,
            self.img_aspect,
            self.pos,
            self.vert_fov,
            self.lookat,
            self.vup,
            self.defocus_angle,
            self.focus_dist,
            self.num_threads
        )
    }

    pub fn quality(mut self, quality: QualityOptions) -> CameraBuilder {
        self.quality = quality;
        self
    }

    pub fn img_aspect(mut self, aspect: f64) -> CameraBuilder {
        self.img_aspect = aspect;
        self
    }

    
}

