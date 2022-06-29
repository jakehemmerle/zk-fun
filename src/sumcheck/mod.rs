use ark_ff::Field;
use ark_poly::{
    polynomial::multivariate::{SparsePolynomial as SparseMVPolynomial, SparseTerm},
    DenseMVPolynomial, Polynomial,
};
use ark_std::rand::RngCore;

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
pub struct Verifier<F: Field, const N: usize> {
    g: SparseMVPolynomial<F, SparseTerm>,
    round: usize,
    challenges: Vec<F>,
    previous_poly: Option<SparseMVPolynomial<F, SparseTerm>>,
    claim: F,
}

impl<F: Field, const N: usize> Verifier<F, N> {
    pub fn init(g: SparseMVPolynomial<F, SparseTerm>, initial_claim: F) -> Self {
        Verifier {
            g,
            round: 0,
            challenges: vec![],
            previous_poly: None,
            claim: initial_claim,
        }
    }

    fn query_oracle(&mut self) -> F {
        self.g.evaluate(&self.challenges)
    }

    pub fn verify_round(
        &mut self,
        current_poly: SparseMVPolynomial<F, SparseTerm>,
        rng: &mut dyn RngCore,
    ) -> Option<F> {
        // since our polynomials are univariate only in theory (in practice it's represented as a multivariate polynomial),
        // to evaluate it at any variable X, we need to evaluate the whole polynomial at [X, X, ...]

        // if first round, don't use prev poly, just eval at 0 and 1, assert its equal to claim, then return our first challenge element
        assert!(
            current_poly.num_vars() == 1,
            "polynomial should be univariate"
        );
        let computed_0 = current_poly.evaluate(&vec![F::zero(); N]);
        let computed_1 = current_poly.evaluate(&vec![F::one(); N]);
        println!("current poly: {:?}", current_poly);
        println!("previous poly: {:?}", self.previous_poly);
        println!("current poly(0) = {:?}", computed_0);
        println!("current_poly(1) = {:?}", computed_1);

        let computed = computed_0 + computed_1;

        if self.round == 0 {
            assert_eq!(computed, self.claim, "polynomials should be equal");
        } else {
            match &self.previous_poly {
                Some(prev_poly) => {
                    assert_eq!(
                        computed,
                        prev_poly.evaluate(&vec![*self.challenges.last().unwrap(); N]),
                        "polynomials should be equal"
                    );
                }
                None => {
                    panic!("previous poly should have been set by now");
                }
            }
        }

        let r: F = F::rand(rng);
        self.challenges.push(r);

        // final check that g(r_1, r_2, ..., r_n) = g_v(r_n)
        if self.round == N - 1 {
            assert_eq!(
                current_poly.evaluate(&vec![*self.challenges.last().unwrap(); N]),
                self.query_oracle(),
                "Final round should be equal"
            )
        }
        self.round += 1;
        self.previous_poly = Some(current_poly);
        Some(r)
    }
}
#[allow(unused_imports, dead_code)]
mod test {
    use std::vec;

    use ark_ff::{Fp64, MontBackend, MontConfig, One, Zero};
    use ark_poly::{multivariate::Term, DenseMVPolynomial, Polynomial};

    use super::*;
    use crate::sumcheck::util::util::{get_claim, reduce_poly_to_univar_at_x};
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
    fn test_multivar_reduction_at_0() {
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
    fn test_multivar_reduction_at_1() {
        let g = sample_poly();
        assert_eq!(g.degree(), 3, "degree of g is not 3");

        // eg prover's polynomial in first round has no challenge elements
        // whats the diff between Fq and Fp again?
        let reduced_polynomial = reduce_poly_to_univar_at_x::<Fq, 3>(g, 1, vec![Fq::from(2u8)]);

        let expected_poly: SparseMVPolynomial<Fq, SparseTerm> =
            DenseMVPolynomial::from_coefficients_slice(
                2,
                &[
                    (Fq::from(1), Term::new(vec![(1, 1)])),
                    (Fq::from(34), Term::new(vec![])),
                ],
            );
        let rand_point = vec![Fq::from(2u8); 3];
        assert_eq!(
            reduced_polynomial.evaluate(&rand_point),
            expected_poly.evaluate(&rand_point)
        );
    }
    #[test]

    fn test_multivar_reduction_at_2() {
        let g = sample_poly();
        assert_eq!(g.degree(), 3, "degree of g is not 3");

        // eg prover's polynomial in first round has no challenge elements
        // whats the diff between Fq and Fp again?
        let reduced_polynomial =
            reduce_poly_to_univar_at_x::<Fq, 3>(g, 2, vec![Fq::from(2u8), Fq::from(3u8)]);

        let expected_poly: SparseMVPolynomial<Fq, SparseTerm> =
            DenseMVPolynomial::from_coefficients_slice(
                3,
                &[
                    (Fq::from(5), Term::new(vec![(2, 1)])),
                    (Fq::from(16), Term::new(vec![])),
                ],
            );
        let rand_point = vec![Fq::from(2u8); 3];
        assert_eq!(
            reduced_polynomial.evaluate(&rand_point),
            expected_poly.evaluate(&rand_point)
        );
    }

    // This test is a from the example Sum Check in "Proofs, Arguments, and Zero-Knowledge"
    #[test]
    fn test_protocol() {
        const V: usize = 3usize;
        let g = sample_poly();
        let rng = &mut test_rng();
        let claim = get_claim::<Fq, V>(g.clone());

        let mut prover: Prover<Fq, V> = Prover::init(g.clone());
        let mut verifier: Verifier<Fq, V> = Verifier::init(g, claim);

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
            r = verifier.verify_round(poly_i, rng);
            challenges.push(r);
        }
    }
}

/*

g(x, y, z) = some poly

round 1
prover provides polynomial such that

f1(x) = g(x, {0, 1}, {0, 1})        = g(x, 0, 0) + g(x, 1, 0) + g(x, 0, 1) + g(x, 1, 1)

computed = f1(0) + f1(1);
f1(0) = g(0, {0, 1}, {0, 1})
f1(1) = g(1, {0, 1}, {0, 1})
initial_claim =? f1(0) + f1(1)

then round 2, prover gets r1. provides verifier with

f2(0) = g(r1, 0, {0, 1})
f2(1) = g(r1, 1, {0, 1})

next round

f3(0) = g(r1, r2, 0)
f3(1) = g(r1, r2, 1)

if f3(0) + f3(1) = g(r1, r2, r3)


 */
