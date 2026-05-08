use ark_bn254::Fr;
use ark_poly::univariate::DensePolynomial;
use ark_poly::DenseUVPolynomial;
use ark_ff::Field;
use crate::r1cs::r1cs::R1CS;

pub struct QAP {
    pub u: Vec<Vec<Fr>>,
    pub v: Vec<Vec<Fr>>,
    pub w: Vec<Vec<Fr>>,
    pub t: Vec<Fr>,        
}

fn compute_t(num_constraints: usize) -> Vec<Fr> {
    let mut t_poly = DensePolynomial::from_coefficients_vec(vec![Fr::ONE]);

    for d in 1..=num_constraints {
        let factor = DensePolynomial::from_coefficients_vec(vec![
            -Fr::from(d as u64),
            Fr::ONE,
        ]);
        t_poly = &t_poly * &factor;
    }

    t_poly.coeffs
}

fn lagrange_interpolate(points: &[(Fr, Fr)]) -> Vec<Fr> {
    let n = points.len();
    assert!(n > 0, "Need at least one point");

    let mut result = DensePolynomial::from_coefficients_vec(vec![Fr::ZERO]);

    for (k, (x_k, y_k)) in points.iter().enumerate() {
        if *y_k == Fr::ZERO {
            continue;
        }

        let mut basis = DensePolynomial::from_coefficients_vec(vec![Fr::ONE]);

        for (j, (x_j, _)) in points.iter().enumerate() {
            if j == k {
                continue;
            }

            let factor = DensePolynomial::from_coefficients_vec(vec![
                -*x_j,
                Fr::ONE,
            ]);
            basis = &basis * &factor;

            let denom_inv = (*x_k - *x_j).inverse().unwrap();
            basis = &basis * denom_inv;
        }

        result = &result + &(&basis * *y_k);
    }

    result.coeffs
}

impl QAP {
    pub fn from_r1cs(r1cs: &R1CS) -> Self {
        let mut u = Vec::with_capacity(r1cs.witness_length);
        let mut v = Vec::with_capacity(r1cs.witness_length);
        let mut w = Vec::with_capacity(r1cs.witness_length);
        let t = compute_t(r1cs.num_constraints);

        for i in 0..=r1cs.witness_length - 1 {
            let mut l_tuples: Vec<(Fr, Fr)> = Vec::with_capacity(r1cs.num_constraints);
            let mut r_tuples: Vec<(Fr, Fr)> = Vec::with_capacity(r1cs.num_constraints);
            let mut o_tuples: Vec<(Fr, Fr)> = Vec::with_capacity(r1cs.num_constraints);

            for j in 1..=r1cs.num_constraints {
                l_tuples.push((Fr::from(j as u64), r1cs.l[j-1][i]));
                r_tuples.push((Fr::from(j as u64), r1cs.r[j-1][i]));
                o_tuples.push((Fr::from(j as u64), r1cs.o[j-1][i]));
            }

            u.push(lagrange_interpolate(&l_tuples));
            v.push(lagrange_interpolate(&r_tuples));
            w.push(lagrange_interpolate(&o_tuples));
        }

        Self { u, v, w, t }
    }
}
