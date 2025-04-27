use std::f64::consts::{PI, TAU};
use std::sync::Arc;

use crate::aabb::AABB;
use crate::anim::Animation;
use crate::hit::{Hit, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::{Point3, Vector3};

pub struct Sphere {
    anim: Animation,
    radius: f64,
    material: Arc<Box<dyn Material>>,
    bbox: AABB,
}

impl Sphere {
    pub fn from_const_pos(
        x: f64, y: f64, z: f64, radius: f64,
        material: Arc<Box<dyn Material>>
    ) -> Self {
        Self::new(
            Animation::constant(Vector3 (x, y, z), radius), radius, material
        )
    }
    
    pub fn new(
        anim: Animation,
        radius: f64,
        material: Arc<Box<dyn Material>>,
    ) -> Self {
        let bbox = anim.bound_all();
        Sphere { anim, radius, material, bbox }
    }

    pub fn new_const_pos(
        pos: Point3,
        radius: f64,
        material: Arc<Box<dyn Material>>,
    ) -> Self {
        let anim = Animation::constant(pos, radius);
        let bbox = anim.bound_all();
        Sphere { anim, radius, material, bbox }
    }

    pub fn as_box(self) -> Box<dyn Hittable> {
        Box::new(self)
    }

    fn get_sphere_uv(p: Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        let u = phi / TAU;
        let v = theta / PI;
        
        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<Hit> {
        let cur_pos = self.anim.sample(ray.time);
        let oc = cur_pos - ray.origin;
        let a = ray.dir.length_squared();
        let h = ray.dir.dot(&oc);
        let c = oc.length_squared() - self.radius*self.radius;
        
        let discr = h*h - a*c;
        if discr < 0.0 { return None; };
        
        let dsqrt = discr.sqrt();

        let mut root = (h - dsqrt) / a;
        if !ray_t.surrounds(root) {
            root = (h + dsqrt) / a;
            if !ray_t.surrounds(root) { return None; };
        };


        let point = ray.at(root);
        let outward_normal = (point - cur_pos) / self.radius;
        let front_face = ray.dir.dot(&outward_normal) < 0.0;
        let normal =
            if front_face {outward_normal} else {-outward_normal};
        
        let (u, v) = Self::get_sphere_uv(outward_normal);

        return Some(Hit {
            p: point,
            normal,
            t: root,
            u,
            v,
            front_face,
            material: self.material.clone()
        });
    }

    fn bounding(&self) -> &AABB {
        &self.bbox
    }
}