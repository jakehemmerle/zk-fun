pub mod multivariate {
    use ark_ff::Field;

    pub struct MultivarBasis<F: Field, const N: usize> {
        // many of the i64 vars and friends shuold be bools, since they represent some set {0, 1} ^ N
        w: [F; N],
        basis: fn(x: [F; N], w: [F; N]) -> F,
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
}

#[cfg(test)]
mod tests {
    use super::multivariate::MultivarBasis;
    use ark_ff::{Fp64, MontBackend, MontConfig, One, Zero};

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
        // let example_fn = |x: [i64; 2]| -> i64 {
        //     match x {
        //         [0, 0] => 1,
        //         [0, 1] => 2,
        //         [1, 0] => 1,
        //         [1, 1] => 4,
        //         _ => panic!("invalid input"),
        //     }
        // };
        // let mut accumulator = 0;
        // for all w in set_w {
        //     let two_bit_lagrange = multivar::LagrangeBasis::new(w);
        //     let term = two_bit_lagrange.evaluate(x) * example_fn(x)
        //     accumulator += term;
        // }
    }
}
