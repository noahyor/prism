use std::{f64::consts::PI, simd::{LaneCount, Simd, SupportedLaneCount}};

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn simd_of<const N: usize>(n: f64) -> Simd<f64, N>
    where LaneCount<N>: SupportedLaneCount {
    let array = [n; N];
    Simd::from_array(array)
}