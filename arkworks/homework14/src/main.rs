use ark_bls12_381::Fq as F; // Prime Field
use ark_ff::{Field, FpParameters, PrimeField};
use ark_std::{One, UniformRand};

fn main() {
    let mut rng = ark_std::rand::thread_rng();
    // select a random value from the field
    let a = F::rand(&mut rng);

    let modulus = <F as PrimeField>::Params::MODULUS;

    // show that 1 + 1 = 2
    assert_eq!(F::one() + F::one(), F::from(2));

    // show that the multiplicative inverse of a number multipled by itself equals one.
    assert_eq!(a.inverse().unwrap() * a, F::one());

    // show that a value raised to the power of the modulus is equal to itself
    // use the `pow` function to raise to a power
    assert_eq!(a.pow(&modulus), a);
}
