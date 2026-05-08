use ark_bn254::Fr;
use groth16_implementation::common::curve::g1_gen;
use groth16_implementation::srs::powers_of_tau::PowersOfTau;

#[test]
fn powers_of_tau_has_expected_length_and_commit_behavior() {
    let tau = Fr::from(5u64);
    let srs = PowersOfTau::new(4, tau);

    assert_eq!(srs.powers_g1.len(), 5);
    assert_eq!(srs.powers_g2.len(), 5);

    let coeffs = vec![Fr::from(3u64)];
    let commit = srs.commit_g1(&coeffs);
    assert_eq!(commit, g1_gen() * Fr::from(3u64));
}
