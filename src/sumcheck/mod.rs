use ark_ff::Field;
use ark_poly::{
    polynomial::multivariate::{SparsePolynomial as SparseMVPolynomial, SparseTerm},
    DenseMVPolynomial, Polynomial,
};
use itertools::Itertools;
mod util;

pub struct Prover<F: Field, const N: usize> {
    g: SparseMVPolynomial<F, SparseTerm>,
    round: usize,
    // all the challenges `r_i` from the verifier
    challenges: Vec<F>,
}

impl<F: Field, const N: usize> Prover<F, N> {
    pub fn init(g: SparseMVPolynomial<F, SparseTerm>) -> Self {
        Prover {
            g,
            round: 0,
            challenges: vec![],
        }
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
}
pub struct Verifier<F: Field, const N: usize> {
    g: SparseMVPolynomial<F, SparseTerm>,
    round: usize,
    challenges: Vec<F>,
}

impl<F: Field, const N: usize> Verifier<F, N> {
    fn init(g: SparseMVPolynomial<F, SparseTerm>) -> Self {
        Verifier {
            g,
            round: 0,
            challenges: vec![],
        }
    }

    fn verify_round(&mut self, r: SparseMVPolynomial<F, SparseTerm>) -> () {
        self.round += 1;
    }
}

pub fn setup_protocol<F: Field, const N: usize>(
    g: SparseMVPolynomial<F, SparseTerm>,
) -> (Prover<F, N>, Verifier<F, N>) {
    (Prover::init(g.clone()), Verifier::init(g))
}

#[allow(unused_imports, dead_code)]
mod test {
    use std::vec;

    use ark_ff::{Fp64, MontBackend, MontConfig, One, Zero};
    use ark_poly::{multivariate::Term, DenseMVPolynomial, Polynomial};

    use super::*;
    use crate::sumcheck::util::util::reduce_poly_to_univar_at_x;

    #[derive(MontConfig)]
    #[modulus = "71"]
    #[generator = "3"]
    pub struct FqConfig;
    pub type Fq = Fp64<MontBackend<FqConfig, 1>>;

    fn sample_poly() -> SparseMVPolynomial<Fq, SparseTerm> {
        SparseMVPolynomial::from_coefficients_slice(
            3,
            &[
                (Fq::from(2), SparseTerm::new(vec![(0, 3)])),
                (Fq::from(1), SparseTerm::new(vec![(0, 1), (2, 1)])),
                (Fq::from(1), SparseTerm::new(vec![(1, 1), (2, 1)])),
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
    fn test_multivar_reduction() {
        let g = sample_poly();
        assert_eq!(g.degree(), 3, "degree of g is not 3");

        // eg prover's polynomial in first round has no challenge elements
        // whats the diff between Fq and Fp again?
        let reduced_polynomial = reduce_poly_to_univar_at_x::<Fq, 3>(g, 0, vec![]);

        let expected_poly: SparseMVPolynomial<Fq, SparseTerm> =
            DenseMVPolynomial::from_coefficients_slice(
                1,
                &[
                    (Fq::from(8), Term::new(vec![(0, 3)])),
                    (Fq::from(2), Term::new(vec![(0, 1)])),
                    (Fq::from(1), Term::new(vec![])),
                ],
            );

        assert_eq!(reduced_polynomial, expected_poly);
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
