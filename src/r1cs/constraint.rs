use ark_bn254::Fr;

pub struct Constraint {
    pub a: Vec<(usize, Fr)>,
    pub b: Vec<(usize, Fr)>,
    pub c: Vec<(usize, Fr)>
}

impl Constraint {
    pub fn new(
        a: Vec<(usize, Fr)>,
        b: Vec<(usize, Fr)>,
        c: Vec<(usize, Fr)>
    ) -> Self {
        Self { a, b, c }
    }
}