pub use ark_bn254::Fr;
use ark_std::UniformRand; 

pub type Scalar = Fr;

pub fn random_fr(rng: &mut impl rand::Rng) -> Scalar {
    Fr::rand(rng)
}
