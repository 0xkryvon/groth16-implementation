use ark_bn254::Fr;
use ark_std::UniformRand;
use ark_std::Zero;
use crate::common::curve::g1_mul;
use crate::common::curve::g2_mul;
use crate::proof::Proof;
use crate::{common::curve::{G1, G2}, qap::qap::QAP, setup::prover_key::ProvingKey};
use ark_poly::univariate::DensePolynomial;
use ark_poly::DenseUVPolynomial;

fn eval_polynomial_g1(polynomial: &Vec<Fr>, powers_of_tau: &Vec<G1>) -> G1 {
    assert!(polynomial.len() <= powers_of_tau.len(), "polynomial degree exceeds SRS size");
    let mut sum = G1::zero();
    for i in 0..polynomial.len() {
        sum = sum + powers_of_tau[i] * polynomial[i];
    }
    sum
}

fn eval_polynomial_g2(polynomial: &Vec<Fr>, powers_of_tau: &Vec<G2>) -> G2 {
    assert!(polynomial.len() <= powers_of_tau.len(), "polynomial degree exceeds SRS size");
    let mut sum = G2::zero();
    for i in 0..polynomial.len() {
        sum = sum + powers_of_tau[i] * polynomial[i];
    }
    sum
}

fn polynomial_add_assign(target: &mut DensePolynomial<Fr>, other: &DensePolynomial<Fr>) {
    if target.coeffs.len() < other.coeffs.len() {
        target.coeffs.resize(other.coeffs.len(), Fr::from(0u64));
    }
    for i in 0..other.coeffs.len() {
        target.coeffs[i] += other.coeffs[i];
    }
}

fn polynomial_scaled_from_coeffs(coeffs: &[Fr], scalar: Fr) -> DensePolynomial<Fr> {
    DensePolynomial::from_coefficients_vec(coeffs.iter().map(|c| *c * scalar).collect())
}

fn polynomial_sub_assign(target: &mut DensePolynomial<Fr>, other: &DensePolynomial<Fr>) {
    if target.coeffs.len() < other.coeffs.len() {
        target.coeffs.resize(other.coeffs.len(), Fr::from(0u64));
    }
    for i in 0..other.coeffs.len() {
        target.coeffs[i] -= other.coeffs[i];
    }
}

pub fn prove(qap: &QAP, proving_key: &ProvingKey, witness: &Vec<Fr>, num_public_inputs: usize) -> (Proof, Vec<Fr>) {
    assert!(num_public_inputs < witness.len() && num_public_inputs != 0);
    assert_eq!(qap.u.len(), witness.len(), "witness length must match qap variable count");
    assert_eq!(qap.v.len(), witness.len(), "witness length must match qap variable count");
    assert_eq!(qap.w.len(), witness.len(), "witness length must match qap variable count");
    let mut rng = ark_std::rand::thread_rng();
    let r = Fr::rand(&mut rng);
    let s = Fr::rand(&mut rng);

    let mut sum_a1 = G1::zero();
    let mut sum_b1 = G1::zero();
    let mut sum_b2 = G2::zero();
    for i in 0..witness.len() {
        sum_a1 = sum_a1 + eval_polynomial_g1(&qap.u[i], &proving_key.srs_g1) * witness[i];
        sum_b1 = sum_b1 + eval_polynomial_g1(&qap.v[i], &proving_key.srs_g1) * witness[i];
        sum_b2 = sum_b2 + eval_polynomial_g2(&qap.v[i], &proving_key.srs_g2) * witness[i];
    }

    let mut sum_c1 = G1::zero();
    for j in num_public_inputs..witness.len() {
        sum_c1 = sum_c1 + proving_key.phi_i_g1[j] * witness[j]
    }

    let mut a_poly = DensePolynomial::from_coefficients_vec(vec![Fr::from(0u64)]);
    let mut b_poly = DensePolynomial::from_coefficients_vec(vec![Fr::from(0u64)]);
    let mut c_poly = DensePolynomial::from_coefficients_vec(vec![Fr::from(0u64)]);
    for i in 0..witness.len() {
        polynomial_add_assign(&mut a_poly, &polynomial_scaled_from_coeffs(&qap.u[i], witness[i]));
        polynomial_add_assign(&mut b_poly, &polynomial_scaled_from_coeffs(&qap.v[i], witness[i]));
        polynomial_add_assign(&mut c_poly, &polynomial_scaled_from_coeffs(&qap.w[i], witness[i]));
    }
    let mut numerator = &a_poly * &b_poly;
    polynomial_sub_assign(&mut numerator, &c_poly);
    let t_poly = DensePolynomial::from_coefficients_vec(qap.t.clone());
    let h_poly = &numerator / &t_poly;
    assert_eq!(&h_poly * &t_poly, numerator, "invalid witness: (A*B-C) is not divisible by T");

    let a_g1 = proving_key.alpha_g1 + sum_a1 + g1_mul(proving_key.sigma_g1, r);
    let b_g1 = proving_key.beta_g1 + sum_b1 + g1_mul(proving_key.sigma_g1, s);
    let b_g2 = proving_key.beta_g2 + sum_b2 + g2_mul(proving_key.sigma_g2, s);
    let c_g1 = sum_c1 + eval_polynomial_g1(&h_poly.coeffs, &proving_key.h_t_tau)
                                    + g1_mul(a_g1, s) + g1_mul(b_g1, r)
                                    - g1_mul(proving_key.sigma_g1, r * s);

    let mut public_witness = Vec::with_capacity(num_public_inputs);
    for i in 1..num_public_inputs {
        public_witness.push(witness[i]);
    }

    (Proof::from_projective_to_affine(a_g1, b_g2, c_g1), public_witness)
}
