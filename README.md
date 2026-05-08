# groth16-implementation

Educational Groth16 zk-SNARK implementation over BN254 using `arkworks`.

This repository implements the full flow:
- R1CS constraint system
- R1CS -> QAP transformation
- Trusted setup (proving + verifying keys)
- Prover
- Verifier (pairing equation)

## Current Status

- Core flow is working end-to-end.
- `cargo test` passes.
- `cargo run` executes a full Groth16 example for:
  - `x^4 + 5x^2y^2 = z`

## Requirements

- Rust (stable)
- Cargo

## Quick Start

Build:

```bash
cargo build
```

Run tests:

```bash
cargo test
```

Run the demo circuit from `main.rs`:

```bash
cargo run
```

Expected output:

```text
Groth16 proof valid: true
```

## Demo Circuit in `main.rs`

The default executable proves and verifies:

`x^4 + 5x^2y^2 = z`

with example values:
- `x = 2`
- `y = 3`
- `z = 196`

It builds R1CS constraints for intermediate wires (`x^2`, `y^2`, `x^4`, `5x^2y^2`), runs setup, creates a proof, and verifies it.

## Project Structure

```text
src/
  common/      # Curve/field utilities and pairing helpers
  r1cs/        # Constraint representation and dense R1CS matrices
  qap/         # R1CS -> QAP interpolation and target polynomial
  srs/         # Powers of tau utilities
  setup/       # Trusted setup, proving key, verifying key
  prover/      # Groth16 prover
  verifier/    # Groth16 verifier
  proof.rs     # Proof type (A, B, C)
  main.rs      # End-to-end demo example

tests/
  common_tests.rs
  r1cs_tests.rs
  qap_tests.rs
  srs_tests.rs
  setup_tests.rs
  prover_tests.rs
  verifier_tests.rs
```

## Test Components Separately

You can run tests per component:

```bash
cargo test --test common_tests
cargo test --test r1cs_tests
cargo test --test qap_tests
cargo test --test srs_tests
cargo test --test setup_tests
cargo test --test prover_tests
cargo test --test verifier_tests
```

## Notes

- This codebase is educational and not production-hardened.
- Trusted setup is generated locally for demonstration.
- No serialization format or persistent parameter storage is included yet.

## License

MIT. See [LICENSE](LICENSE).
