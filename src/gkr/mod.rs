use std::marker::PhantomData;

use ark_ff::Field;
use ark_poly::{
    polynomial::multivariate::{SparsePolynomial as SparseMVPolynomial, SparseTerm},
    DenseMVPolynomial, Polynomial,
};
use ark_std::rand::RngCore;

pub struct Prover<F: Field, const N: usize> {
    _phantom: PhantomData<F>,
}

impl<F: Field, const N: usize> Prover<F, N> {
    pub fn init() -> Self {
        Prover {
            _phantom: PhantomData,
        }
    }

    pub fn prove_round(&mut self, r: Option<F>) -> SparseMVPolynomial<F, SparseTerm> {
        unimplemented!()
    }
}
pub struct Verifier<F: Field, const N: usize> {
    _phantom: PhantomData<F>,
}

impl<F: Field, const N: usize> Verifier<F, N> {
    pub fn init() -> Self {
        Verifier {
            _phantom: PhantomData,
        }
    }

    pub fn verify_round(
        &mut self,
        current_poly: SparseMVPolynomial<F, SparseTerm>,
        rng: &mut dyn RngCore,
    ) -> Option<F> {
        unimplemented!()
    }
}

#[allow(unused_imports, dead_code)]
mod test {
    use std::vec;

    use ark_ff::{Fp64, MontBackend, MontConfig, One, Zero};
    use ark_poly::{multivariate::Term, DenseMVPolynomial, Polynomial};

    use super::*;
    use ark_ff::UniformRand;
    use ark_std::test_rng;

    #[derive(MontConfig)]
    #[modulus = "71"]
    #[generator = "3"]
    pub struct FqConfig;
    pub type Fq = Fp64<MontBackend<FqConfig, 1>>;

    // This test is a from the example Sum Check in "Proofs, Arguments, and Zero-Knowledge"
    #[test]
    fn test_protocol() {
        const V: usize = 3usize;
        let rng = &mut test_rng();

        let mut prover: Prover<Fq, V> = Prover::init();
        let mut verifier: Verifier<Fq, V> = Verifier::init();

        let mut r: Option<Fq> = None;

        let mut polynomials: Vec<SparseMVPolynomial<Fq, SparseTerm>> = vec![];

        for _ in 0..V {
        }
    }
}
