use std::{ops::Div, simd::{LaneCount, Mask, Simd, SupportedLaneCount}, sync::{Arc, Mutex}};

use crate::{color::Color, hit::Hittable, interval::Interval, math::lerp, simd::{Maskish, Simdish}, vector::{Point3, SimdPoint3, SimdVector3, SimdVector3Mask, Vector3}, writer::Debugger};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub dir: Vector3,
    pub time: f64,
}

pub struct SimdRay<const N: usize>
    where LaneCount<N>: SupportedLaneCount {
    pub origins: SimdPoint3<N>,
    pub dirs: SimdVector3<N>,
    pub times: Simd<f64, N>
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.dir * t
    }

    pub fn color(&self, max_depth: usize, world: &dyn Hittable, debugger: Arc<Mutex<Debugger>>) -> Color {
        if max_depth <= 0 {return Color::BLACK;}
        
        match world.hit(self, &Interval {min: 0.001, max: f64::INFINITY}) {
            Some(hit) => {
                if let Some((attenuation, new_dir)) =
                    hit.material.scatter(self, &hit, debugger.clone())
                {
                    attenuation * new_dir.color(max_depth - 1, world, debugger.clone())
                } else {
                    Color::BLACK
                }

            }
            None => {
                let unit_dir = self.dir.unit();
                let a = 0.5 * (unit_dir.y() + 1.0);
                let white = Color { r: 1.0, g: 1.0, b: 1.0 };
                let blue = Color { r: 0.5, g: 0.7, b: 1.0 };
                lerp(&white, &blue, a)
            },
        }
    }
}

impl<const N: usize> SimdRay<N>
    where LaneCount<N>: SupportedLaneCount {
    pub fn from_array(array: [Ray; N]) -> Self {
        let mut origins = [Vector3::new(); N];
        let mut dirs = [Vector3::new(); N];
        let mut times = [0.0; N];

        for index in 0..N {
            origins[index] = array[index].origin;
            dirs[index] = array[index].dir;
            times[index] = array[index].time;
        }

        SimdRay { origins: SimdVector3::from_array(origins), dirs: SimdVector3::from_array(dirs), times: Simd::from_array(times) }
    }
}

#[derive(Clone, Copy)]
pub struct SimdRayMask<const N: usize>
    where LaneCount<N>: SupportedLaneCount {
    origins: SimdVector3Mask<N>,
    dirs: SimdVector3Mask<N>,
    times: Mask<i64, N>,
}

impl<const N: usize> Simdish for SimdRay<N>
    where LaneCount<N>: SupportedLaneCount {
    type Unpacked = Ray;
    type Mask = SimdRayMask<N>;

    fn replace(self, replace: Self, mask: <Self as Simdish>::Mask) -> Self {
        todo!()
    }
}

impl<const N: usize> Maskish for SimdRayMask<N>
    where LaneCount<N>: SupportedLaneCount {
    fn replace(self, replace: Self, mask: Self) -> Self {
        let origins = self.origins.replace(replace.origins, mask.origins);
        todo!()
    }
}