# groth16-implementation

A from-scratch educational implementation of the Groth16 zk-SNARK workflow over `BN254` using the `arkworks` ecosystem.

This crate includes:
- An `R1CS` representation
- Conversion from `R1CS` to `QAP`
- A trusted setup that builds proving/verifying keys
- Prover logic that generates a Groth16 proof
- Verifier logic based on pairing checks

## Requirements

- Rust toolchain (stable)
- Cargo

## Build

```bash
cargo build
```

## Run tests

```bash
cargo test
```

## Project layout

```text
src/
  common/      # Curve and field helpers (BN254, pairings, scalar ops)
  r1cs/        # Constraint representation and dense matrix form
  qap/         # R1CS -> QAP conversion + interpolation
  srs/         # Powers of tau
  setup/       # Trusted setup, proving key, verifying key
  prover/      # Proof generation
  verifier/    # Proof verification
  proof.rs     # Proof struct (A, B, C)
```

## High-level flow

1. Define constraints as `Constraint` values and build an `R1CS`.
2. Convert the `R1CS` into a `QAP` using `QAP::from_r1cs`.
3. Run `Setup::construct_setup` to generate proving and verifying keys.
4. Call `prove(...)` with a witness to create a `Proof`.
5. Call `verify(...)` with the proof and public inputs.

## Minimal library usage example

```rust
use ark_bn254::Fr;
use groth16_implementation::{
    prover::prover::prove,
    qap::qap::QAP,
    r1cs::{constraint::Constraint, r1cs::R1CS},
    setup::trusted_setup::Setup,
    verifier::verify::verify,
};

fn demo() {
    // Example witness convention in this codebase:
    // witness[0] is commonly the constant 1.
    // public inputs are expected from witness[1..num_public_inputs].
    let witness = vec![Fr::from(1u64), Fr::from(3u64), Fr::from(9u64)];

    // Example single constraint (shape only):
    // (a · w) * (b · w) = (c · w)
    let constraints = vec![
        Constraint::new(
            vec![(1, Fr::from(1u64))], // a
            vec![(1, Fr::from(1u64))], // b
            vec![(2, Fr::from(1u64))], // c
        )
    ];

    let num_public_inputs = 2; // includes witness[0] convention in this implementation
    let r1cs = R1CS::new(constraints, witness.len(), num_public_inputs);
    let qap = QAP::from_r1cs(&r1cs);

    let setup = Setup::construct_setup(&qap, r1cs.num_constraints, r1cs.witness_length);
    let (proof, public_witness) = prove(&qap, &setup.proving_key, &witness, num_public_inputs);
    let ok = verify(&proof, &setup.verifying_key, &public_witness);

    assert!(ok);
}
```

## Notes

- This repository is educational and focuses on clarity over production hardening.
- Trusted setup here is local and randomized for demonstration.
- Serialization, parameter persistence, and robust test vectors are not yet included.

## License

MIT (see `LICENSE`).
