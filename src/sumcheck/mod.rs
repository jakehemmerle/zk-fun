use ark_ff::Field;
use ark_poly::{
    polynomial::multivariate::{SparsePolynomial as SparseMVPolynomial, SparseTerm},
    DenseMVPolynomial, Polynomial,
};
use ark_std::rand::{RngCore, Rng};
use itertools::Itertools;

use self::util::util::reduce_poly_to_univar_at_x;
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



    pub fn prove_round(&mut self, r: Option<F>) -> SparseMVPolynomial<F, SparseTerm> {
        // partially evaluate new polynomials with g and the challenge for each round
        // TODO: use refs, not clone; throw errors instead of assertions (since this is library code?)
        match r {
            Some(challenge) => {
                self.challenges.push(challenge);
            }
            None => {
                assert!(
                    self.round == 0,
                    "round should be 0 when no challenge is provided"
                );
            }
        }
        let poly_i =
            reduce_poly_to_univar_at_x::<F, N>(self.g.clone(), self.round, self.challenges.clone());
        // update the round as the last step
        self.round += 1;
        poly_i
    }
}
pub struct Verifier<F: Field, R: Rng, const N: usize> {
    g: SparseMVPolynomial<F, SparseTerm>,
    round: usize,
    challenges: Vec<F>, 
    previous_poly: SparseMVPolynomial<F, SparseTerm>,
    claim: F,
    rng: R,
}

impl<F: Field, R: Rng, const N: usize> Verifier<F, R, N> {
    fn init(g: SparseMVPolynomial<F, SparseTerm>, initial_claim: F, rng: R) -> Self {
        Verifier {
            g,
            round: 0,
            challenges: vec![],
            previous_poly: None,
            claim: initial_claim,
            rng,
        }
    }

    fn verify_round(&mut self, current_poly: SparseMVPolynomial<F, SparseTerm>, ) -> Option<F> {
        let r: F = F::rand(&mut self.rng);
        // if first round, don't use prev poly, just eval at 0 and 1, assert its equal to claim, then return our first challenge element
        if round == 0 {
            let claim = current_poly.evaluate(0) + current_poly.evaluate(1);
            assert_eq!(claim, self.claim, "polynomials should be equal");
        }
        // otherwise, 
        assert_eq!(current_poly.evaluate(0) + current_poly.evaluate(1), self.previous_poly.evaluate(r), "polynomials should be equal");
        self.round += 1;
        Some(r)
    }
}

pub fn setup_protocol<F: Field, R: Rng, const N: usize>(
    g: SparseMVPolynomial<F, SparseTerm>,
    claim: F,
    rng: R,
) -> (Prover<F, N>, Verifier<F, R, N>) {
    (Prover::init(g.clone()), Verifier::init(g, claim, rng))
}

#[allow(unused_imports, dead_code)]
mod test {
    use std::vec;

    use ark_ff::{Fp64, MontBackend, MontConfig, One, Zero};
    use ark_poly::{multivariate::Term, DenseMVPolynomial, Polynomial};

    use super::*;
    use crate::sumcheck::util::util::{reduce_poly_to_univar_at_x, get_claim};
    use ark_ff::UniformRand;
    use ark_std::test_rng;

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
        let rng = &mut test_rng();
        let claim = get_claim::<Fq, V>(g);
        let (mut prover, mut verifier) = setup_protocol::<Fq, RngCore, V>(g, claim, rng);
        assert_eq!(claim, Fq::from(12));
        let mut r: Option<Fq> = None;


        // the following two vectors are just for debugging purposes.
        // In reality, the verifier and prover both store all the challenges,
        // and the verifier stores the previous polynomial they received from the prover.
        let mut polynomials: Vec<SparseMVPolynomial<Fq, SparseTerm>> = vec![];
        let mut challenges: Vec<Option<Fq>> = vec![];
        challenges.push(r);

        let mut poly_i: SparseMVPolynomial<Fq, SparseTerm>;
        for _ in 0..V {
            poly_i = prover.prove_round(r);
            polynomials.push(poly_i.clone());
            r = verifier.verify_round(poly_i);
            challenges.push(r);
        }
    }
}
