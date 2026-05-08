use crate::common::curve::{G1, G2};
use crate::common::fr::Fr;
use ark_ec::Group;
use ark_ff::Field;

pub struct PowersOfTau {
    pub powers_g1: Vec<G1>,
    pub powers_g2: Vec<G2>,
}

impl PowersOfTau {
    pub fn commit_g1(&self, coeffs: &[Fr]) -> G1 {
        assert!(
            coeffs.len() <= self.powers_g1.len(),
            "SRS too short: need at least {} powers, got {}",
            coeffs.len(),
            self.powers_g1.len()
        );

        coeffs
            .iter()
            .zip(&self.powers_g1)
            .map(|(&c, base)| *base * c)
            .sum()
    }

    pub fn commit_g2(&self, coeffs: &[Fr]) -> G2 {
        assert!(
            coeffs.len() <= self.powers_g2.len(),
            "SRS too short: need at least {} powers, got {}",
            coeffs.len(),
            self.powers_g2.len()
        );

        coeffs
            .iter()
            .zip(&self.powers_g2)
            .map(|(&c, base)| *base * c)
            .sum()
    }

    pub fn new(max_degree: usize, tau: Fr) -> Self {
        let g1_gen = G1::generator();
        let g2_gen = G2::generator();

        let mut powers_g1 = Vec::with_capacity(max_degree + 1);
        let mut powers_g2 = Vec::with_capacity(max_degree + 1);

        let mut current_power = Fr::ONE;   // τ⁰
        for _ in 0..=max_degree {
            powers_g1.push(g1_gen * current_power);
            powers_g2.push(g2_gen * current_power);
            current_power *= tau;
        }

        Self { powers_g1, powers_g2 }
    }
}
