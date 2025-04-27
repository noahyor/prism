use std::sync::Arc;

use crate::{aabb::AABB, interval::Interval, material::Material, ray::Ray, vector::{Point3, Vector3}};

pub struct Hit {
    pub p: Point3,
    pub normal: Vector3,
    pub material: Arc<Box<dyn Material>>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<Hit>;

    fn bounding(&self) -> &AABB;

    fn objects(&self) -> usize {1}
}

pub struct HittableList {
    vec: Vec<Box<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { vec: Vec::new(), bbox: AABB::EMPTY }
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.bbox = AABB::enclose(&self.bbox, obj.bounding());
        self.vec.push(obj);
    }

    // pub fn clear(&mut self) {
    //     self.vec.clear();
    // }

    pub fn num_objects(&self) -> usize {
        self.vec.len()
    }

    pub fn objects(self) -> Vec<Box<dyn Hittable>> {
        self.vec
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<Hit> {
        self.vec.iter().fold(None, |accum, v| {
            let hit = match v.hit(ray, ray_t) {
                None => return accum,
                Some(hit) => hit,
            };
            match accum {
                None => Some(hit),
                Some(prev_hit) => if hit.t <= prev_hit.t {
                    Some(hit)
                } else {
                    Some(prev_hit)
                }
            }
        })
    }

    fn bounding(&self) -> &AABB {
        &self.bbox
    }

    fn objects(&self) -> usize {
        self.vec.len()
    }
}