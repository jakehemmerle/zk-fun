pub mod multivariate {
    use ark_ff::Field;
    use itertools::Itertools;

    #[derive(Debug, Clone)]
    pub struct MultivarBasis<Fp: Field, const V: usize> {
        // many of the i64 vars and friends shuold be bools, since they represent some set {0, 1} ^ N
        w: [Fp; V],
        basis: fn(x: [Fp; V], w: [Fp; V]) -> Fp,
    }

    pub struct MulitvarInterpolation<Fp: Field, const V: usize> {
        f: fn(w: [u8; V]) -> Fp,
        bases: Vec<MultivarBasis<Fp, V>>,
        interpolation:
            fn(x: [Fp; V], f: fn([u8; V]) -> Fp, bases: &Vec<MultivarBasis<Fp, V>>) -> Fp,
    }

    impl<Fp: Field, const V: usize> MulitvarInterpolation<Fp, V> {
        pub fn new(f: fn(x: [u8; V]) -> Fp) -> Self {
            // iterate over the boolean hypercube {0,1}^V
            let bases: Vec<MultivarBasis<Fp, V>> = (0..V)
                .map(|_| 0..2u8)
                .multi_cartesian_product()
                .map(|w| MultivarBasis::from(w))
                .collect();

            MulitvarInterpolation {
                f,
                bases,
                interpolation: |x: [Fp; V], f: fn([u8; V]) -> Fp, bases| -> Fp {
                    let mut accumulator: Fp = Fp::zero();
                    for (index, w) in (0..V).map(|_| 0..2u8).multi_cartesian_product().enumerate() {
                        accumulator += bases[index].evaluate(x) * f(w.try_into().unwrap());
                    }
                    accumulator
                },
            }
        }
        pub fn interpolate(&self, x: [Fp; V]) -> Fp {
            (self.interpolation)(x, self.f, &self.bases)
        }
    }

    impl<F: Field, const N: usize> MultivarBasis<F, N> {
        pub fn new(w: [F; N]) -> Self {
            MultivarBasis {
                w,
                basis: |x: [F; N], w: [F; N]| {
                    let mut accumulator: F = F::one();
                    for (x_i, w_i) in x.iter().zip(w) {
                        accumulator *= (w_i * x_i) + (F::one() - w_i) * (F::one() - x_i);
                    }
                    accumulator
                },
            }
        }
        pub fn evaluate(&self, x: [F; N]) -> F {
            (self.basis)(x, self.w)
        }
    }

    impl<F: Field, const N: usize> From<Vec<u8>> for MultivarBasis<F, N> {
        fn from(w: Vec<u8>) -> Self {
            let x = w
                .iter()
                .map(|&x| F::from(x))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            MultivarBasis::new(x)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::multivariate::{MulitvarInterpolation, MultivarBasis};
    use ark_ff::{Fp128, Fp64, MontBackend, MontConfig, One, Zero};

    #[derive(MontConfig)]
    #[modulus = "5"]
    #[generator = "3"]
    pub struct FqConfig5;
    pub type Fq5 = Fp64<MontBackend<FqConfig5, 1>>;

    #[test]
    fn lagrange_multivar_works() {
        let two_bit_lagrange = MultivarBasis::new([Fq5::zero(), Fq5::zero()]);
        assert_eq!(
            two_bit_lagrange.evaluate([Fq5::zero(), Fq5::zero()]),
            Fq5::one()
        );
        assert_eq!(
            two_bit_lagrange.evaluate([Fq5::zero(), Fq5::one()]),
            Fq5::zero()
        );
        assert_eq!(
            two_bit_lagrange.evaluate([Fq5::one(), Fq5::zero()]),
            Fq5::zero()
        );
        assert_eq!(
            two_bit_lagrange.evaluate([Fq5::one(), Fq5::one()]),
            Fq5::zero()
        );
    }

    #[test]
    fn multilineal_extension() {
        fn example_fn(x: [u8; 2]) -> Fq5 {
            match x {
                [0, 0] => Fq5::from(1),
                [0, 1] => Fq5::from(2),
                [1, 0] => Fq5::from(1),
                [1, 1] => Fq5::from(4),
                _ => panic!("invalid input"),
            }
        };
        let interpolation = MulitvarInterpolation::<Fq5, 2>::new(example_fn);
        assert_eq!(interpolation.interpolate([Fq5::zero(), Fq5::zero()]), Fq5::one());
        assert_eq!(interpolation.interpolate([Fq5::zero(), Fq5::one()]), Fq5::from(2u8));
        assert_eq!(interpolation.interpolate([Fq5::one(), Fq5::zero()]), Fq5::one());
        assert_eq!(interpolation.interpolate([Fq5::one(), Fq5::one()]), Fq5::from(4u8));
        assert_eq!(interpolation.interpolate([Fq5::from(3), Fq5::from(4)]), Fq5::from(4u8));
        assert_eq!(interpolation.interpolate([Fq5::from(4), Fq5::from(4)]), Fq5::from(2u8));
    }
}
