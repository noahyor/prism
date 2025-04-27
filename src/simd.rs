pub struct SimdVec<S: Simdish> {
    vec: Vec<S>,
    residue: Vec<<S as Simdish>::Unpacked>,
}

pub struct MaskedSimd<S: Simdish> {
    pub simd: S,
    pub mask: <S as Simdish>::Mask,
}

impl<S: Simdish> SimdVec<S> {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            residue: Vec::new(),
        }
    }

    pub fn push(&mut self, simd: S) {
        self.vec.push(simd);
    }

    pub fn push_residue(&mut self, mut residue: Vec<<S as Simdish>::Unpacked>) {
        self.residue.append(&mut residue);
    }

    pub fn extract(self) -> (Vec<S>, Vec<<S as Simdish>::Unpacked>) {
        (self.vec, self.residue)
    }
}

impl<S: Simdish> Simdish for MaskedSimd<S> {
    type Unpacked = (<S as Simdish>::Unpacked, bool);
    type Mask = <S as Simdish>::Mask;

    fn replace(self, replace: Self, mask: <Self as Simdish>::Mask) -> Self {
        let simd = self.simd.replace(replace.simd, mask);
        let mask = self.mask.replace(replace.mask, mask);
        Self { simd, mask }
    }
}

pub trait Simdish {
    type Unpacked;
    type Mask: Maskish;

    fn replace(self, replace: Self, mask: <Self as Simdish>::Mask) -> Self;
}

pub trait Maskish: Copy + Clone {
    fn replace(self, replace: Self, mask: Self) -> Self;
}