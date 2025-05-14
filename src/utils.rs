use num_bigint::BigInt;
use sha2::{Sha256, Digest};

pub fn mod_exp(base: &BigInt, exp: &BigInt, modulus: &BigInt) -> BigInt {
    base.modpow(exp, modulus)
}

pub fn hash_to_bigint(data: &BigInt, modulus: &BigInt) -> BigInt {
    let mut hasher = Sha256::new();
    hasher.update(data.to_bytes_be().1);
    let result = hasher.finalize();
    let bigint = BigInt::from_bytes_be(num_bigint::Sign::Plus, &result);
    bigint % modulus
}
