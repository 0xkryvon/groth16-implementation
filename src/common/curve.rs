use ark_bn254::{Bn254, G1Projective, G2Projective, G1Affine, G2Affine};
use ark_ec::Group;
use ark_ec::pairing::Pairing;
use crate::common::fr::Scalar;

pub type G1 = G1Projective;
pub type G2 = G2Projective;
pub type G1Aff = G1Affine;
pub type G2Aff = G2Affine;
pub type Gt = ark_bn254::Fq12;

pub fn g1_gen() -> G1 {
    G1::generator()
}
pub fn g2_gen() -> G2 {
    G2::generator()
}

pub fn g1_mul(point: G1, scalar: Scalar) -> G1 {
    point * scalar
}

pub fn g2_mul(point: G2, scalar: Scalar) -> G2 {
    point * scalar
}

pub fn pair(a: impl Into<G1Aff>, b: impl Into<G2Aff>) -> Gt {
    Bn254::pairing(a.into(), b.into()).0
}
