use crate::{
    common::curve::G1,
    qap::qap::QAP,
    setup::{prover_key::ProvingKey, verifier_key::VerifyingKey}, srs::powers_of_tau::{ PowersOfTau}
};
use ark_ff::Field;
use ark_bn254::Fr;
use ark_std::UniformRand;
use crate::common::curve::{g1_gen, g2_gen, g1_mul, g2_mul};

pub struct Setup {
   pub proving_key: ProvingKey,
   pub verifying_key: VerifyingKey 
}

fn phi_computation(
    alpha: Fr,
    beta: Fr,
    gamma: Fr,
    tau: Fr,
    witness_length: usize,
    qap: &QAP
) -> Vec<G1> {
    let mut phi_vector = Vec::with_capacity(witness_length);
    for i in 0..witness_length {
        let point: G1 = g1_mul(
            g1_gen(),
            phi_factor(
                &qap.u[i],
                &qap.v[i],
                &qap.w[i],
                alpha,
                beta,
                tau,
                gamma
            )
        );
        phi_vector.push(point);
    }
    phi_vector
}

fn phi_factor(u: &Vec<Fr>, v: &Vec<Fr>, w: &Vec<Fr>, alpha: Fr, beta: Fr, tau: Fr, gamma: Fr) -> Fr {
    let denominator = alpha * polynomial_at_tau(v, tau) 
                        + beta * polynomial_at_tau(u, tau)
                        + polynomial_at_tau(w, tau);
    let quotient = denominator * gamma.inverse().unwrap();
    quotient
}

fn polynomial_at_tau(polynomial: &Vec<Fr>, tau: Fr) -> Fr {
    let mut result: Fr = Fr::from(0u64);
    let mut power_of_tau = Fr::from(1u64);
    for i in (0..polynomial.len()).rev() {
        result = result + polynomial[i] * power_of_tau;
        power_of_tau = power_of_tau * tau;
    }
    result
}

fn power(base: Fr, exponent: usize) -> Fr {
    let mut result = Fr::from(1u64);
    for _ in 0..exponent {
        result = result * base;
    }
    result
}

fn h_srs_computation(qap: &QAP, num_constraints: usize, tau: Fr, sigma: Fr) -> Vec<G1> {
    let t_of_tau = polynomial_at_tau(&qap.t, tau);
    let mut h_srs_vector: Vec<G1> = Vec::with_capacity(num_constraints - 1);
    for i in (0..(num_constraints - 1)).rev() {
        let factor = power(tau, i) * t_of_tau * sigma.inverse().unwrap();
        let point = g1_mul(g1_gen(), factor);
        h_srs_vector.push(point);
    }
    h_srs_vector
}

impl Setup {
    pub fn construct_setup(qap: &QAP, num_constraints: usize, witness_length: usize) -> Self {
        let mut rng = ark_std::rand::thread_rng();
        let alpha = Fr::rand(&mut rng);
        let beta  = Fr::rand(&mut rng);
        let tau   = Fr::rand(&mut rng);
        let gamma = Fr::rand(&mut rng);
        let sigma = Fr::rand(&mut rng);
        let powers_of_tau = PowersOfTau::new(num_constraints, tau);

        let proving_key = ProvingKey {
            alpha_g1: g1_mul(g1_gen(), Fr::from(alpha)),
            beta_g1: g1_mul(g1_gen(), Fr::from(beta)),
            beta_g2: g2_mul(g2_gen(), Fr::from(beta)),
            gamma_g2: g2_mul(g2_gen(), Fr::from(gamma)),
            sigma_g1: g1_mul(g1_gen(), Fr::from(sigma)),
            sigma_g2: g2_mul(g2_gen(), Fr::from(sigma)),
            srs_g1: powers_of_tau.powers_g1,
            srs_g2: powers_of_tau.powers_g2,
            h_t_tau: h_srs_computation(qap, num_constraints, tau, sigma),
            phi_i_g1: phi_computation(alpha, beta, gamma, tau, witness_length, qap)
        };
        let verifying_key = VerifyingKey {
            alpha_g1: g1_mul(g1_gen(), Fr::from(alpha)),
            beta_g2: g2_mul(g2_gen(), Fr::from(beta)),
            gamma_g2: g2_mul(g2_gen(), Fr::from(gamma)),
            sigma_g2: g2_mul(g2_gen(), Fr::from(sigma)),
            phi_i_g1: phi_computation(alpha, beta, gamma, tau, witness_length, qap)
        };

        Self { proving_key, verifying_key }
    }
}