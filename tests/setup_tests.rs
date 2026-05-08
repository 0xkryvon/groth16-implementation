use ark_bn254::Fr;
use groth16_implementation::qap::qap::QAP;
use groth16_implementation::r1cs::{constraint::Constraint, r1cs::R1CS};
use groth16_implementation::setup::trusted_setup::Setup;

fn sample_r1cs() -> R1CS {
    let constraints = vec![
        Constraint::new(
            vec![(1, Fr::from(1u64))],
            vec![(1, Fr::from(1u64))],
            vec![(2, Fr::from(1u64))],
        ),
        Constraint::new(
            vec![(2, Fr::from(1u64))],
            vec![(0, Fr::from(1u64))],
            vec![(2, Fr::from(1u64))],
        ),
    ];
    R1CS::new(constraints, 3, 2)
}

#[test]
fn setup_key_material_has_consistent_sizes() {
    let r1cs = sample_r1cs();
    let qap = QAP::from_r1cs(&r1cs);
    let setup = Setup::construct_setup(&qap, r1cs.num_constraints, r1cs.witness_length);

    assert_eq!(setup.proving_key.srs_g1.len(), r1cs.num_constraints + 1);
    assert_eq!(setup.proving_key.srs_g2.len(), r1cs.num_constraints + 1);
    assert_eq!(setup.proving_key.phi_i_g1.len(), r1cs.witness_length);
    assert_eq!(setup.verifying_key.phi_i_g1.len(), r1cs.witness_length);
    assert_eq!(setup.proving_key.h_t_tau.len(), r1cs.num_constraints - 1);
}
