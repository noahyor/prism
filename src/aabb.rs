use std::simd::{cmp::SimdPartialOrd, LaneCount, Mask, SupportedLaneCount};

use crate::{interval::{Interval, SimdInterval, SimdIntervalMask}, ray::{Ray, SimdRay}, simd::{MaskedSimd, SimdVec}, sieve::SimdSieve, util::simd_of, vector::Point3};

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    // pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
    //     AABB { x, y, z }
    // }

    pub fn from(a: Point3, b: Point3) -> Self {
        let x = if a.x() <= b.x() {
            Interval {min: a.x(), max: b.x()}
        } else {
            Interval {min: b.x(), max: a.x()}
        };

        let y = if a.y() <= b.y() {
            Interval {min: a.y(), max: b.y()}
        } else {
            Interval {min: b.y(), max: a.y()}
        };

        let z = if a.z() <= b.z() {
            Interval {min: a.z(), max: b.z()}
        } else {
            Interval {min: b.z(), max: a.z()}
        };

        AABB { x, y, z }
    }

    pub fn enclose(a: &AABB, b: &AABB) -> Self {
        AABB {
            x: Interval::enclose(&a.x, &b.x),
            y: Interval::enclose(&a.y, &b.y),
            z: Interval::enclose(&a.z, &b.z),
        }
    }

    pub fn expand(&self, delta: f64) -> Self {
        AABB {
            x: self.x.expand(delta),
            y: self.y.expand(delta),
            z: self.z.expand(delta),
        }
    }

    pub fn test(&self, ray: &Ray, ray_t: &Interval) -> Option<Interval> {
        let mut max = ray_t.max;
        let mut min = ray_t.min;
        
        let adinv = 1.0 / ray.dir.x();
        let t0 = (self.x.min - ray.origin.x()) * adinv;
        let t1 = (self.x.max - ray.origin.x()) * adinv;

        if t0 < t1 {
            if t0 > min {min = t0};
            if t1 < max {max = t1};
        } else {
            if t1 > min {min = t1};
            if t0 < max {max = t0};
        }

        if max <= min {return None;}

        let adinv = 1.0 / ray.dir.y();
        let t0 = (self.y.min - ray.origin.y()) * adinv;
        let t1 = (self.y.max - ray.origin.y()) * adinv;

        if t0 < t1 {
            if t0 > min {min = t0};
            if t1 < max {max = t1};
        } else {
            if t1 > min {min = t1};
            if t0 < max {max = t0};
        }

        if max <= min {return None;}

        let adinv = 1.0 / ray.dir.z();
        let t0 = (self.z.min - ray.origin.z()) * adinv;
        let t1 = (self.z.max - ray.origin.z()) * adinv;

        if t0 < t1 {
            if t0 > min {min = t0};
            if t1 < max {max = t1};
        } else {
            if t1 > min {min = t1};
            if t0 < max {max = t0};
        }

        if max <= min {return None;}

        Some(Interval { min, max })
    }

    pub fn simd_test<const N: usize>(&self, rays: SimdRay<N>, ray_t: SimdInterval<N>) -> MaskedSimd<SimdInterval<N>>
        where LaneCount<N>: SupportedLaneCount {
        let mut mask = Mask::from_array([true; N]);

        let mut max = ray_t.maxes;
        let mut min = ray_t.mins;
        
        let adinv = 1.0 / rays.dirs;

        let t0 = (simd_of(self.x.min) - rays.origins.x()) * adinv.x();
        let t1 = (simd_of(self.x.max) - rays.origins.x()) * adinv.x();

        // if t0 < t1 {
        //     if t0 > min {min = t0};
        //     if t1 < max {max = t1};
        // } else {
        //     if t1 > min {min = t1};
        //     if t0 < max {max = t0};
        // }

        let min_lt = t0.simd_gt(min).select(t0, min);
        let max_lt = t1.simd_lt(max).select(t1, max);

        let min_ge = t1.simd_gt(min).select(t1, min);
        let max_ge = t0.simd_lt(max).select(t0, max);

        let select = t0.simd_lt(t1);

        min = select.select(min_lt, min_ge);
        max = select.select(max_lt, max_ge);

        mask = mask & min.simd_gt(max);


        let t0 = (simd_of(self.y.min) - rays.origins.y()) * adinv.y();
        let t1 = (simd_of(self.y.max) - rays.origins.y()) * adinv.y();

        let min_lt = t0.simd_gt(min).select(t0, min);
        let max_lt = t1.simd_lt(max).select(t1, max);

        let min_ge = t1.simd_gt(min).select(t1, min);
        let max_ge = t0.simd_lt(max).select(t0, max);

        let select = t0.simd_lt(t1);

        min = select.select(min_lt, min_ge);
        max = select.select(max_lt, max_ge);

        mask = mask & min.simd_gt(max);


        let t0 = (simd_of(self.z.min) - rays.origins.z()) * adinv.z();
        let t1 = (simd_of(self.z.max) - rays.origins.z()) * adinv.z();

        let min_lt = t0.simd_gt(min).select(t0, min);
        let max_lt = t1.simd_lt(max).select(t1, max);

        let min_ge = t1.simd_gt(min).select(t1, min);
        let max_ge = t0.simd_lt(max).select(t0, max);

        let select = t0.simd_lt(t1);

        min = select.select(min_lt, min_ge);
        max = select.select(max_lt, max_ge);

        mask = mask & min.simd_gt(max);

        MaskedSimd {simd: SimdInterval { mins: min, maxes: max }, mask: SimdIntervalMask {mins: mask, maxes: mask}}
    }

    pub fn simd_vec_test<const N: usize>(
        &self,
        rays_vec: SimdVec<SimdRay<N>>,
        ray_t: SimdVec<SimdInterval<N>>
    ) -> SimdSieve<SimdRay<N>>
        where LaneCount<N>: SupportedLaneCount,
    {
        let (rays, residue) = rays_vec.extract();
        let (ray_ts, ts_residue) = ray_t.extract();
        for (ray, ts) in rays.into_iter().zip(ray_ts) {
            let mask = self.simd_test(ray, ts);
        }
        todo!()
    }

    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.z.size() {
            if self.x.size() > self.z.size() {0} else {2}
        } else {
            if self.y.size() > self.z.size() {1} else {2}
        }
    }

    pub fn is_norm(&self) -> bool {
        self.x.is_norm() && self.y.is_norm() && self.z.is_norm()
    }

    pub const EMPTY: AABB = AABB {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };

    pub const UNIVERSE: AABB = AABB {
        x: Interval::UNIVERSE,
        y: Interval::UNIVERSE,
        z: Interval::UNIVERSE,
    };
}