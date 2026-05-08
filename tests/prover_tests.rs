use ark_bn254::Fr;
use groth16_implementation::prover::prover::prove;
use groth16_implementation::qap::qap::QAP;
use groth16_implementation::r1cs::{constraint::Constraint, r1cs::R1CS};
use groth16_implementation::setup::trusted_setup::Setup;
use groth16_implementation::verifier::verify::verify;

fn sample_instance() -> (QAP, Setup, Vec<Fr>, usize) {
    let witness = vec![Fr::from(1u64), Fr::from(3u64), Fr::from(9u64)];
    let num_public_inputs = 2;
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

    let r1cs = R1CS::new(constraints, witness.len(), num_public_inputs);
    let qap = QAP::from_r1cs(&r1cs);
    let setup = Setup::construct_setup(&qap, r1cs.num_constraints, r1cs.witness_length);

    (qap, setup, witness, num_public_inputs)
}

#[test]
fn prover_outputs_verifiable_proof_and_public_witness_slice() {
    let (qap, setup, witness, num_public_inputs) = sample_instance();
    let (proof, public_witness) = prove(&qap, &setup.proving_key, &witness, num_public_inputs);

    assert_eq!(public_witness.len(), num_public_inputs - 1);
    assert_eq!(public_witness[0], witness[1]);
    assert!(verify(&proof, &setup.verifying_key, &public_witness));
}
