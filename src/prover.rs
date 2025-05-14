use num_bigint::BigInt;
use rand::{Rng, rngs::ThreadRng, thread_rng};

use crate::utils::{mod_exp, hash_to_bigint};

pub struct Prover {
    pub p: BigInt,
    pub g: BigInt,
    pub x: BigInt,
    pub y: BigInt,
}

impl Prover {
    pub fn new(p: BigInt, g: BigInt, x: BigInt) -> Self {
        let y = mod_exp(&g, &x, &p);
        Self { p, g, x, y }
    }

    pub fn prove(&self) -> (BigInt, BigInt, BigInt) {
        let mut rng: ThreadRng = rand::thread_rng();
        let r_u32: u32 = rng.gen_range(1..10);
        let r = BigInt::from(r_u32);
        let a = mod_exp(&self.g, &r, &self.p);
        let c = hash_to_bigint(&a, &self.p);
        let z = (&r + &c * &self.x) % &self.p;
        (a, c, z)
    }
}
