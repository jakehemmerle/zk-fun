use ark_ff::Field;
use itertools::Itertools;

pub struct Prover<F: Field, const N: usize> {
    g: fn([F; N]) -> F,
    round: usize,
}

impl<F: Field, const N: usize> Prover<F, N> {
    pub fn init(g: fn([F; N]) -> F) -> Self {
        Prover { g, round: 0 }
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
            accumulator += (self.g)(temp);
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
    g: fn([F; N]) -> F,
    round: usize,
}

impl<F: Field, const N: usize> Verifier<F, N> {
    fn init(g: fn([F; N]) -> F) -> Self {
        Verifier { g, round: 0 }
    }

    fn verify_round(&mut self, r: F) -> () {
        self.round += 1;
    }
}

pub fn setup_protocol<F: Field, const N: usize>(
    g: fn([F; N]) -> F,
) -> (Prover<F, N>, Verifier<F, N>) {
    (Prover::init(g), Verifier::init(g))
}

mod test {
    use super::*;
    use ark_ff::{Fp64, MontBackend, MontConfig, One, Zero};

    #[derive(MontConfig)]
    #[modulus = "71"]
    #[generator = "3"]
    pub struct FqConfig;
    pub type Fq = Fp64<MontBackend<FqConfig, 1>>;

    fn sample_g(x: [Fq; 3]) -> Fq {
        Fq::from(2) * (x[0] * x[0] * x[0]) + x[0] * x[2] + x[1] * x[2]
    }

    #[test]
    fn test_g() {
        let h: Fq = sample_g([Fq::zero(), Fq::zero(), Fq::zero()]);
        assert_eq!(h, Fq::zero());
        let h: Fq = sample_g([Fq::one(), Fq::zero(), Fq::zero()]);
        assert_eq!(h, Fq::from(2));
        let h: Fq = sample_g([Fq::one(), Fq::zero(), Fq::one()]);
        assert_eq!(h, Fq::from(3));
    }

    #[test]
    fn test_protocol() {
        let v = 3usize;
        let (prover, verifier) = setup_protocol(sample_g);
        let claim = prover.get_claim();
        assert_eq!(claim, Fq::from(12));
        let mut r = Fq::zero();
        for _ in 0..(v as usize) {
            // let poly = prover.prove_round(r);
            // r = verifier.verify_round(poly);
        }
    }
}
