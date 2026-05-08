use crate::common::curve::{G1, G2};

pub struct VerifyingKey {
    pub alpha_g1: G1,
    pub beta_g2: G2,
    pub gamma_g2: G2,
    pub sigma_g2: G2,
    pub phi_i_g1: Vec<G1>, // only public portion
}