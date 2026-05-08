use ark_bn254::Fr;
use groth16_implementation::common::curve::{g1_gen, g1_mul};

#[test]
fn g1_scalar_mul_matches_repeated_addition() {
    let g = g1_gen();
    let two_g_via_mul = g1_mul(g, Fr::from(2u64));
    let two_g_via_add = g + g;

    assert_eq!(two_g_via_mul, two_g_via_add);
}
