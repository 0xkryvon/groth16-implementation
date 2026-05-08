use ark_bn254::{G1Affine, G2Affine};
use ark_ec::CurveGroup;
use crate::common::curve::{G1, G2};

pub struct Proof {
    pub a: G1Affine,
    pub b: G2Affine,
    pub c: G1Affine
}

impl Proof {
    pub fn from_projective_to_affine(a_g1: G1, b_g2: G2, c_g1: G1) -> Self {
        let a_aff = a_g1.into_affine();
        let b_aff = b_g2.into_affine();
        let c_aff = c_g1.into_affine();
        Self { a: a_aff, b: b_aff, c: c_aff }
    }
}