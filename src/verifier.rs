use num_bigint::BigInt;
use crate::utils::mod_exp;

pub struct Verifier {
    pub p: BigInt,
    pub g: BigInt,
    pub y: BigInt,
}

impl Verifier {
    pub fn verify(&self, a: &BigInt, c: &BigInt, z: &BigInt) -> bool {
        let left = mod_exp(&self.g, z, &self.p);
        let right = (a * mod_exp(&self.y, c, &self.p)) % &self.p;
        left == right
    }
}
