use ark_bn254::Fr;
use crate::r1cs::constraint::Constraint;
use ark_ff::Field;

pub struct R1CS {
    pub l: Vec<Vec<Fr>>,
    pub r: Vec<Vec<Fr>>,
    pub o: Vec<Vec<Fr>>,
    pub witness_length: usize,
    pub num_constraints: usize,
    pub num_public_inputs: usize,
}

impl R1CS {
    pub fn new(
        constraints: Vec<Constraint>,
        witness_length: usize,
        num_public_inputs: usize
    ) -> Self {
        let num_constraints = constraints.len();
        assert!(num_constraints != 0, "the number of the constraints can't be zero");
        let mut l: Vec<Vec<Fr>> = Vec::with_capacity(num_constraints);
        let mut r: Vec<Vec<Fr>> = Vec::with_capacity(num_constraints);
        let mut o: Vec<Vec<Fr>> = Vec::with_capacity(num_constraints);

        for constraint in &constraints {
            let mut row_l = vec![Fr::ZERO; witness_length];
            let mut row_r = vec![Fr::ZERO; witness_length];
            let mut row_o = vec![Fr::ZERO; witness_length];

            for (index, element) in &constraint.a {
                row_l[*index] = *element;
            }
            for (index, element) in &constraint.b {
                row_r[*index] = *element;
            }
            for (index, element) in &constraint.c {
                row_o[*index] = *element;
            }

            l.push(row_l);
            r.push(row_r);
            o.push(row_o);
        }
        Self { l, r, o, witness_length, num_constraints, num_public_inputs }
    }
}