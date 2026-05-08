use ark_bn254::Fr;
use groth16_implementation::r1cs::{constraint::Constraint, r1cs::R1CS};

#[test]
fn r1cs_dense_matrices_are_built_correctly() {
    let constraints = vec![
        Constraint::new(
            vec![(1, Fr::from(1u64))],
            vec![(2, Fr::from(3u64))],
            vec![(3, Fr::from(5u64))],
        ),
        Constraint::new(
            vec![(0, Fr::from(7u64))],
            vec![(1, Fr::from(11u64))],
            vec![(2, Fr::from(13u64))],
        ),
    ];

    let r1cs = R1CS::new(constraints, 4, 2);

    assert_eq!(r1cs.num_constraints, 2);
    assert_eq!(r1cs.witness_length, 4);
    assert_eq!(r1cs.l[0][1], Fr::from(1u64));
    assert_eq!(r1cs.r[0][2], Fr::from(3u64));
    assert_eq!(r1cs.o[0][3], Fr::from(5u64));
    assert_eq!(r1cs.l[1][0], Fr::from(7u64));
    assert_eq!(r1cs.r[1][1], Fr::from(11u64));
    assert_eq!(r1cs.o[1][2], Fr::from(13u64));
}
