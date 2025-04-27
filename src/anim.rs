use crate::{aabb::AABB, spline::{BoundedCurve, ConstantSpline, LinearSpline}, vector::Point3};

pub struct Animation {
    curve: Box<dyn BoundedCurve>,
    size: f64,
}

impl Animation {
    pub fn new(
        curve: Box<dyn BoundedCurve>,
        size: f64,
    ) -> Self {
        Animation { curve, size }
    }

    pub fn linear(controls: Vec<Point3>, size: f64) -> Self {
        Animation::new(Box::new(LinearSpline::new(controls)), size)
    }

    pub fn constant(point: Point3, size: f64) -> Self {
        Animation::new(Box::new(ConstantSpline::new(point)), size)
    }
    
    pub fn bound_all(&self) -> AABB {
        self.curve.bound_radius(self.size)
    }

    pub fn sample(&self, t: f64) -> Point3 {
        self.curve.sample(t)
    }
}