use std::ops::{Add, Mul};

pub fn lerp<T>(a: &T, b: &T, t: f64) -> <<T as Mul<f64>>::Output as Add>::Output     where
    T: Mul<f64> + Clone,
    <T as Mul<f64>>::Output: Add,
{
    let a = a.clone();
    let b = b.clone();
    a * (1.0-t) + b * t
}