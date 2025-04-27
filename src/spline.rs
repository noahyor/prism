use crate::{aabb::AABB, interval::Interval, math::lerp, vector::Point3};

pub trait Curve: Sync + Send {
    fn sample(&self, u: f64) -> Point3;
}

pub trait BoundedCurve: Curve {    
    fn bound_interval(&self, interval: Interval) -> AABB;

    fn bound(&self) -> AABB {
        self.bound_interval(Interval::UNIVERSE)
    }
    
    fn bound_radius(&self, radius: f64) -> AABB {
        self.bound().expand(radius)
    }

    fn bound_segment_radius(&self, interval: Interval, radius: f64) -> AABB {
        self.bound_interval(interval).expand(radius)
    }
}

pub struct LinearSpline {
    controls: Vec<Point3>,
}

impl LinearSpline {
    pub fn new(controls: Vec<Point3>) -> Self {
        LinearSpline { controls }
    }
    
    pub fn bound_segment(&self, segment: usize) -> AABB {
        AABB::from(self.controls[segment], self.controls[segment + 1])
    }
}

impl Curve for LinearSpline {
    fn sample(&self, u: f64) -> Point3 {
        let segment = u.floor() as usize;
        let t = u.fract();
        lerp(&self.controls[segment], &self.controls[segment + 1], t)
    }
}

impl BoundedCurve for LinearSpline {
    fn bound_interval(&self, bound: Interval) -> AABB {
        let len = self.controls.len();
        let len_interval = Interval {min: 0.0, max: (len - 2) as f64};
        let segment_min = len_interval.clamp(bound.min) as usize;
        let segment_max = len_interval.clamp(bound.max) as usize;
        AABB::enclose(&self.bound_segment(segment_min), &self.bound_segment(segment_max))
    }
}

pub struct ConstantSpline {
    point: Point3,
}

impl ConstantSpline {
    pub fn new(point: Point3) -> Self {
        Self {
            point
        }
    }
}

impl Curve for ConstantSpline {
    fn sample(&self, _u: f64) -> Point3 {
        self.point
    }
}

impl BoundedCurve for ConstantSpline {
    fn bound_interval(&self, _interval: Interval) -> AABB {
        AABB::from(self.point, self.point)
    }
}