use ark_ff::Field;
use ark_poly::{
    polynomial::multivariate::{
        SparsePolynomial as SparseMVPolynomial, SparseTerm as SparseMVTerm,
    },
    univariate::SparsePolynomial as SparseUVPolynomial,
    Polynomial,
};
use itertools::Itertools;

pub struct Prover<F: Field, const N: usize> {
    g: SparseMVPolynomial<F, SparseMVTerm>,
    round: usize,
    // all the challenges `r_i` from the verifier
    challenges: Vec<F>,
}

impl<F: Field, const N: usize> Prover<F, N> {
    pub fn init(g: SparseMVPolynomial<F, SparseMVTerm>) -> Self {
        Prover { g, round: 0, challenges: vec![] }
    }

    pub fn get_claim(&self) -> F {
        let mut accumulator = F::zero();
        // iterate over the boolean hypercube {0,1}^N
        for b in (0..N).map(|_| 0..2u64).multi_cartesian_product() {
            let temp: [F; N] = b
                .into_iter()
                .map(|x| F::from(x))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            accumulator += self.g.evaluate(&temp.to_vec());
        }

        accumulator
    }

    pub fn prove_round(&mut self, r: F) -> () {
        unimplemented!();
        // if self.round == 0 {
        //     self.run_first_round();
        // } else {
        //     self.run_other_round();
        // }
        // self.round += 1;
    }

    /// Given a multivariate polynomial `self.g` over {x_1, ..., x_N}, evaluate it at all challenge points `0..self.round`,
    /// variable at `index` will remain the X of the univariate polynomial, 
    /// and the rest of the variables will be evaluated at a boolean hypercube of size {0,1}^(g.degree() - index).
    fn reduce_poly_to_univar_at_x(x: usize) -> SparseUVPolynomial<F> {
        unimplemented!()
    }
}

pub struct Verifier<F: Field, const N: usize> {
    g: SparseMVPolynomial<F, SparseMVTerm>,
    round: usize,
}

impl<F: Field, const N: usize> Verifier<F, N> {
    fn init(g: SparseMVPolynomial<F, SparseMVTerm>) -> Self {
        Verifier { g, round: 0 }
    }

    fn verify_round(&mut self, r: F) -> () {
        self.round += 1;
    }
}

pub fn setup_protocol<F: Field, const N: usize>(
    g: SparseMVPolynomial<F, SparseMVTerm>,
) -> (Prover<F, N>, Verifier<F, N>) {
    (Prover::init(g.clone()), Verifier::init(g))
}

mod test {
    use std::vec;

    use super::*;
    use ark_ff::{Fp64, MontBackend, MontConfig, One, Zero};
    use ark_poly::{multivariate::Term, DenseMVPolynomial, Polynomial};

    #[derive(MontConfig)]
    #[modulus = "71"]
    #[generator = "3"]
    pub struct FqConfig;
    pub type Fq = Fp64<MontBackend<FqConfig, 1>>;

    fn sample_poly() -> SparseMVPolynomial<Fq, SparseMVTerm> {
        SparseMVPolynomial::from_coefficients_slice(
            3,
            &[
                (Fq::from(2), SparseMVTerm::new(vec![(0, 3)])),
                (Fq::from(1), SparseMVTerm::new(vec![(0, 1), (2, 1)])),
                (Fq::from(1), SparseMVTerm::new(vec![(1, 1), (2, 1)])),
            ],
        )
    }

    #[test]
    fn test_g() {
        let h: Fq = sample_poly().evaluate(&vec![Fq::zero(), Fq::zero(), Fq::zero()]);
        assert_eq!(h, Fq::zero());
        let h: Fq = sample_poly().evaluate(&vec![Fq::one(), Fq::zero(), Fq::zero()]);
        assert_eq!(h, Fq::from(2));
        let h: Fq = sample_poly().evaluate(&vec![Fq::one(), Fq::zero(), Fq::one()]);
        assert_eq!(h, Fq::from(3));
    }

    #[test]
    fn test_protocol() {
        const V: usize = 3usize;
        let g = sample_poly();
        let (prover, verifier) = setup_protocol::<Fq, V>(g);
        let claim = prover.get_claim();
        assert_eq!(claim, Fq::from(12));
        let mut r = Fq::zero();
        for _ in 0..V {
            // let poly = prover.prove_round(r);
            // r = verifier.verify_round(poly);
        }
    }
}
