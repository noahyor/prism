use crate::simd::{MaskedSimd, SimdVec, Simdish};

pub struct SimdSieve<S: Simdish> {
    vec: SimdVec<MaskedSimd<S>>,
}

impl<S: Simdish> SimdSieve<S> {
    
}