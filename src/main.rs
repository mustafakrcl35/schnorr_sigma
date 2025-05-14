mod prover;
mod verifier;
mod utils;

use num_bigint::BigInt;
use prover::Prover;
use verifier::Verifier;

fn main() {
    let p = BigInt::parse_bytes(b"23", 10).unwrap();
    let g = BigInt::from(5u32);
    let x = BigInt::from(6u32);

    let prover = Prover::new(p.clone(), g.clone(), x);
    let verifier = Verifier { p, g, y: prover.y.clone() };

    let (a, c, z) = prover.prove();

    println!("Verification: {}", verifier.verify(&a, &c, &z));
}
