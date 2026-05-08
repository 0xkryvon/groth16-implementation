use ark_bn254::Fr;
use ark_ff::Zero;
use groth16_implementation::qap::qap::QAP;
use groth16_implementation::r1cs::{constraint::Constraint, r1cs::R1CS};

fn eval_poly(coeffs: &[Fr], x: Fr) -> Fr {
    let mut acc = Fr::from(0u64);
    let mut x_pow = Fr::from(1u64);
    for c in coeffs {
        acc += *c * x_pow;
        x_pow *= x;
    }
    acc
}

#[test]
fn qap_shapes_and_target_polynomial_roots_are_correct() {
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

    let r1cs = R1CS::new(constraints, 3, 2);
    let qap = QAP::from_r1cs(&r1cs);

    assert_eq!(qap.u.len(), r1cs.witness_length);
    assert_eq!(qap.v.len(), r1cs.witness_length);
    assert_eq!(qap.w.len(), r1cs.witness_length);

    // t(x) = (x-1)(x-2), so t(1)=0 and t(2)=0 for 2 constraints.
    assert!(eval_poly(&qap.t, Fr::from(1u64)).is_zero());
    assert!(eval_poly(&qap.t, Fr::from(2u64)).is_zero());
}
