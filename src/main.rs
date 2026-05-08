use ark_bn254::Fr;
use groth16_implementation::prover::prover::prove;
use groth16_implementation::qap::qap::QAP;
use groth16_implementation::r1cs::{constraint::Constraint, r1cs::R1CS};
use groth16_implementation::setup::trusted_setup::Setup;
use groth16_implementation::verifier::verify::verify;

fn main() {
    let witness = vec![
        Fr::from(1u64),
        Fr::from(2u64),
        Fr::from(3u64),
        Fr::from(196u64),
        Fr::from(4u64),
        Fr::from(9u64),
        Fr::from(16u64),
        Fr::from(180u64),
    ];
    let num_public_inputs = 4;

    let constraints = vec![
        // x * x = x^2
        Constraint::new(
            vec![(1, Fr::from(1u64))],
            vec![(1, Fr::from(1u64))],
            vec![(4, Fr::from(1u64))],
        ),
        // y * y = y^2
        Constraint::new(
            vec![(2, Fr::from(1u64))],
            vec![(2, Fr::from(1u64))],
            vec![(5, Fr::from(1u64))],
        ),
        // x^2 * x^2 = x^4
        Constraint::new(
            vec![(4, Fr::from(1u64))],
            vec![(4, Fr::from(1u64))],
            vec![(6, Fr::from(1u64))],
        ),
        // (5*x^2) * y^2 = 5x^2y^2
        Constraint::new(
            vec![(4, Fr::from(5u64))],
            vec![(5, Fr::from(1u64))],
            vec![(7, Fr::from(1u64))],
        ),
        // (x^4 + 5x^2y^2) * 1 = z
        Constraint::new(
            vec![(6, Fr::from(1u64)), (7, Fr::from(1u64))],
            vec![(0, Fr::from(1u64))],
            vec![(3, Fr::from(1u64))],
        ),
    ];

    let r1cs = R1CS::new(constraints, witness.len(), num_public_inputs);
    let qap = QAP::from_r1cs(&r1cs);
    let setup = Setup::construct_setup(&qap, r1cs.num_constraints, r1cs.witness_length);
    let (proof, public_witness) = prove(&qap, &setup.proving_key, &witness, num_public_inputs);
    let valid = verify(&proof, &setup.verifying_key, &public_witness);

    println!("Groth16 proof valid: {valid}");
}
