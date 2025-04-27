use std::sync::Arc;

use crate::{aabb::AABB, hit::{Hit, Hittable}, interval::Interval, ray::Ray, writer::Debugger};

pub struct BVHNode {
    left: Arc<Box<dyn Hittable>>,
    right: Option<Arc<Box<dyn Hittable>>>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(mut objects: Vec<Box<dyn Hittable>>, debug_file: &str) -> Self {
        Self::new_internal(objects, 0, Debugger::new(debug_file))
    }
    
    fn new_internal(mut objects: Vec<Box<dyn Hittable>>, level: usize, debugger: Debugger) -> Self {
        let mut bbox = AABB::EMPTY;
        for object in &objects {
            bbox = AABB::enclose(&bbox, object.bounding())
        }
        
        let axis = bbox.longest_axis();
        
        let compare = match axis {
            0 => |a: &Box<dyn Hittable>, b: &Box<dyn Hittable>| {
                let a_interval = a.bounding().x;
                let b_interval = b.bounding().x;
                a_interval.min.partial_cmp(&b_interval.min).unwrap()
            },
            1 => |a: &Box<dyn Hittable>, b: &Box<dyn Hittable>| {
                let a_interval = a.bounding().y;
                let b_interval = b.bounding().y;
                a_interval.min.partial_cmp(&b_interval.min).unwrap()
            },
            2 => |a: &Box<dyn Hittable>, b: &Box<dyn Hittable>| {
                let a_interval = a.bounding().z;
                let b_interval = b.bounding().z;
                a_interval.min.partial_cmp(&b_interval.min).unwrap()
            },
            _ => unreachable!()
        };
        
        let len = objects.len();
        
        if len == 1 {
            let obj = objects.remove(0);
            let bbox = obj.bounding().clone();
            BVHNode {
                left: Arc::new(obj),
                right: None,
                bbox,
            }
        } else if len == 2 {
            let left = objects.remove(0);
            let right = objects.remove(0);
            let left_bound = left.bounding().clone();
            let right_bound = right.bounding().clone();
            BVHNode {
                left: Arc::new(left),
                right: Some(Arc::new(right)),
                bbox: AABB::enclose(&left_bound, &right_bound)
            }
        } else {
            objects[..].sort_by(compare);
            let mid = (len)/2;
            let right_objs = objects.split_off(mid);
            let left: Arc<Box<dyn Hittable>> = Arc::new(
                Box::new(BVHNode::new_internal(objects, level + 1, debugger.clone()))
            );
            let right: Arc<Box<dyn Hittable>> = Arc::new(
                Box::new(BVHNode::new_internal(right_objs, level + 1, debugger.clone()))
            );
            BVHNode {
                left,
                right: Some(right),
                bbox,
            }
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<Hit> {
        self.bbox.test(ray, ray_t)?;
        if let None = self.right {return self.left.hit(ray, ray_t)};

        let right = self.right.clone().expect("Impossible!");

        let left_hit = self.left.hit(ray, ray_t);
        match left_hit {
            None => return right.hit(ray, ray_t),
            Some(left_rec) => {
                let right_hit =
                    right.hit(ray, &Interval { min: ray_t.min, max: left_rec.t });
                if let None = right_hit {return Some(left_rec)};
                right_hit
        }}
    }

    fn bounding(&self) -> &AABB {
        &self.bbox
    }

    fn objects(&self) -> usize {
        self.left.objects() + match &self.right {
            None => 0,
            Some(obj) => obj.objects()
        }
    }
}