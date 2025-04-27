use std::simd::{LaneCount, Mask, Simd, SupportedLaneCount};

use crate::simd::{Maskish, Simdish};

#[derive(Clone, Copy, Debug)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

pub struct SimdInterval<const N: usize>
    where LaneCount<N>: SupportedLaneCount {
    pub mins: Simd<f64, N>,
    pub maxes: Simd<f64, N>,
}

impl Interval {
    pub fn enclose(a: &Interval, b: &Interval) -> Self {
        Interval {
            min: if a.min <= b.min {a.min} else {b.min},
            max: if a.max >= b.max {a.max} else {b.max},
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    // pub fn contains(&self, x: f64) -> bool {
    //     self.min <= x && x <= self.max
    // }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {return self.min;}
        if x > self.max {return self.max;}
        x
    }

    pub fn expand(&self, delta: f64) -> Self {               
        Interval { min: self.min - delta, max: self.max + delta }
    }

    pub fn is_norm(&self) -> bool {
        self.min <= self.max
    }

    pub const EMPTY: Interval = Interval {min: f64::INFINITY, max: f64::NEG_INFINITY};
    pub const UNIVERSE: Interval = Interval {min: f64::NEG_INFINITY, max: f64::INFINITY};
}

#[derive(Clone, Copy)]
pub struct SimdIntervalMask<const N: usize>
    where LaneCount<N>: SupportedLaneCount {
    pub mins: Mask<i64, N>,
    pub maxes: Mask<i64, N>,
}

impl<const N: usize> Simdish for SimdInterval<N>
    where LaneCount<N>: SupportedLaneCount {
    type Unpacked = Interval;
    type Mask = SimdIntervalMask<N>;
    
    fn replace(self, replace: Self, mask: <Self as Simdish>::Mask) -> Self {
        todo!()
    }
}

impl<const N: usize> Maskish for SimdIntervalMask<N>
    where LaneCount<N>: SupportedLaneCount {
    
    fn replace(self, replace: Self, mask: Self) -> Self {
        todo!()
    }
}