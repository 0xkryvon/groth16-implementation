use crate::common::curve::{G1, G2};

pub struct ProvingKey {
    pub alpha_g1: G1,
    pub beta_g1: G1,
    pub beta_g2: G2,
    pub gamma_g2: G2,
    pub sigma_g1: G1,
    pub sigma_g2: G2,
    pub srs_g1: Vec<G1>,
    pub srs_g2: Vec<G2>,
    pub h_t_tau: Vec<G1>,
    pub phi_i_g1: Vec<G1>,
}