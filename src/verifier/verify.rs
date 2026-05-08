use ark_bn254::Fr;
use ark_ff::Zero;
use crate::{common::curve::{G1, pair}, proof::Proof, setup::verifier_key::VerifyingKey};

pub fn verify(proof: &Proof, verifying_key: &VerifyingKey, public_witness: &Vec<Fr>) -> bool {
    let mut x_g1 = G1::zero();
    for j in 0..public_witness.len() {
        x_g1 = x_g1 + verifying_key.phi_i_g1[j] * public_witness[j]
    }

    let left_side = pair(proof.a, proof.b);
    let right_side_1 = pair(verifying_key.alpha_g1, verifying_key.beta_g2);
    let right_side_2 = pair(x_g1, verifying_key.gamma_g2);
    let right_side_3 = pair(proof.c, verifying_key.sigma_g2);

    left_side == right_side_1 * right_side_2 * right_side_3
}