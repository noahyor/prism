use rand::{random, Rng};

use crate::vector::Point3;

pub struct Perlin {
    rand_float: [f64; 256],
    perm_x: [usize; 256],
    perm_y: [usize; 256],
    perm_z: [usize; 256],
}

impl Perlin {
    pub fn new() -> Self {
        let mut rand_float = [0.0; 256];

        for i in 0..256 {
            rand_float[i] = random();
        }        
        
        Self {
            rand_float,
            perm_x: Self::gen_perm(),
            perm_y: Self::gen_perm(),
            perm_z: Self::gen_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = (4.0*p.x()) as usize & 255;
        let j = (4.0*p.y()) as usize & 255;
        let k = (4.0*p.z()) as usize & 255;

        self.rand_float[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }

    fn gen_perm() -> [usize; 256] {
        let mut array = [0; 256];
        for i in 0..256 {
            array[i] = i;
        };

        for i in 255..0_usize {
            let target: usize = rand::thread_rng().gen_range(0..i);
            let tmp = array[i];
            array[i] = array[target];
            array[target] = tmp
        }
        array
    }
}