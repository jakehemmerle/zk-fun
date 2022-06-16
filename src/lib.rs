pub mod lagrange {
    use ark_ff::Field;
    /// Univariate Lagrange Basis polynomial
    #[derive(Debug)]
    pub struct UnivarBasis<F: Field> {
        n: usize,
        i: usize,
        // this function doesn't change (static function); move to `impl`, or make unique
        basis: fn(x: usize, n: usize, i: usize) -> F,
    }

    /// Express a vector `a` as the evaluations of a unique univariate polynomial degrees most n - 1 using `UnivarBasis`
    pub struct UnivarInterpolation<F: Field, const N: usize> {
        a: [F; N],
        bases: [UnivarBasis<F>; N],
        interpolation: fn(x: usize, a: &[F; N], bases: &[UnivarBasis<F>; N]) -> F,
    }

    pub struct MultivarBasis<const N: usize> {
        // many of the i64 vars and friends shuold be bools, since they represent some set {0, 1} ^ N
        w: [i64; N],
        basis: fn(x: [i64; N], w: [i64; N]) -> i64,
    }

    impl<F: Field> UnivarBasis<F> {
        pub fn evaluate(&self, point: usize) -> F {
            (self.basis)(point, self.n, self.i)
        }

        pub fn new(n: usize, i: usize) -> Self {
            UnivarBasis {
                n,
                i,
                basis: |x, n, i| -> F {
                    let mut accumulator: F = F::one();
                    for j in 0..(n as u64) {
                        //
                        if j == (i as u64) {
                            continue;
                        } else {
                            accumulator *= ((F::from(x as u64)) - (F::from(j))) / ((F::from(i as u64)) - (F::from(j)));
                        }
                    }
                    accumulator
                },
            }
        }
    }

    impl<F: Field, const N: usize> UnivarInterpolation<F, N> {
        /// Construct a new `UnivarInterpolation` from a vector of coefficients that we're extending.
        /// First, we compute the corresponding `UnivarBasis` for each coefficient, then store these (implicitly) via
        /// in `self.interpolation` closure.
        pub fn new(a: [F; N]) -> UnivarInterpolation<F, N> {
            let bases: [UnivarBasis<F>; N] = (0..N)
                .map(|i| UnivarBasis::new(N, i))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            UnivarInterpolation {
                a,
                bases,
                interpolation: |x, a, delta| -> F {
                    let mut accumulator: F = F::zero();
                    for i in 0..N {
                        accumulator += delta[i].evaluate(x) * a[i];
                    }
                    accumulator
                },
            }
        }

        /// Given a point `x` (can be viewed as index), return the value of the interpolation at that point
        /// Within the size of `self.a`, the point `x` is mapped to the value of `self.a[x]`.
        /// If x > a.len(), then the value of the interpolation is the univariate extension encoding.
        pub fn interpolate(&self, x: usize) -> F {
            (self.interpolation)(x, &self.a, &self.bases)
        }
    }

    impl<const N: usize> MultivarBasis<N> {
        pub fn new(w: [i64; N]) -> Self {
            MultivarBasis {
                w,
                basis: |x: [i64; N], w: [i64; N]| {
                    let mut accumulator: i64 = 1;
                    for (x_i, w_i) in x.iter().zip(w) {
                        accumulator *= (w_i * x_i) + (1 - w_i) * (1 - x_i);
                    }
                    accumulator
                },
            }
        }
        pub fn evaluate(&self, x: [i64; N]) -> i64 {
            (self.basis)(x, self.w)
        }
    }
}

#[cfg(test)]
mod tests {
    use ark_ff::{Fp64, MontBackend, MontConfig};

    use super::*;

    #[derive(MontConfig)]
    #[modulus = "11"]
    #[generator = "3"]
    pub struct FqConfig;
    pub type Fq = Fp64<MontBackend<FqConfig, 1>>;

    #[test]
    fn lagrange_univar_basis_small() {
        // let l = lagrange::UnivarBasis::new(3, 0);
        // assert_eq!(l.evaluate(0), 1);
        // assert_eq!(l.evaluate(1), 0);
        // assert_eq!(l.evaluate(2), 0);
        // assert_eq!(l.evaluate(3), 3);
    }

    #[test]
    fn lagrange_univar_basis_works() {
        let l_0 = lagrange::UnivarBasis::<Fq>::new(4, 0);
        let l_1 = lagrange::UnivarBasis::<Fq>::new(4, 1);
        let l_2 = lagrange::UnivarBasis::<Fq>::new(4, 2);
        let l_3 = lagrange::UnivarBasis::<Fq>::new(4, 3);

        assert_eq!(l_0.evaluate(0), Fq::from(1));
        assert_eq!(l_0.evaluate(1), Fq::from(0));
        assert_eq!(l_0.evaluate(2), Fq::from(0));
        assert_eq!(l_0.evaluate(3), Fq::from(0));

        assert_eq!(l_1.evaluate(0), Fq::from(0));
        assert_eq!(l_1.evaluate(1), Fq::from(1));
        assert_eq!(l_1.evaluate(2), Fq::from(0));
        assert_eq!(l_1.evaluate(3), Fq::from(0));

        assert_eq!(l_2.evaluate(0), Fq::from(0));
        assert_eq!(l_2.evaluate(1), Fq::from(0));
        assert_eq!(l_2.evaluate(2), Fq::from(1));
        assert_eq!(l_2.evaluate(3), Fq::from(0));

        assert_eq!(l_3.evaluate(0), Fq::from(0));
        assert_eq!(l_3.evaluate(1), Fq::from(0));
        assert_eq!(l_3.evaluate(2), Fq::from(0));
        assert_eq!(l_3.evaluate(3), Fq::from(1));
    }

    #[test]
    fn lagrange_univar_works() {
        let a: [Fq; 3] = [Fq::from(2), Fq::from(1), Fq::from(1)];
        let b: [Fq; 3] = [Fq::from(2), Fq::from(1), Fq::from(0)];
        let interpolation_a = lagrange::UnivarInterpolation::new(a);
        let interpolation_b = lagrange::UnivarInterpolation::new(b);

        assert_eq!(interpolation_a.interpolate(0), Fq::from(2));
        assert_eq!(interpolation_a.interpolate(1), Fq::from(1));
        assert_eq!(interpolation_a.interpolate(2), Fq::from(1));
        assert_eq!(interpolation_a.interpolate(3), Fq::from(2));
        assert_eq!(interpolation_a.interpolate(4), Fq::from(4));
        assert_eq!(interpolation_a.interpolate(5), Fq::from(7));

        assert_eq!(interpolation_b.interpolate(0), Fq::from(2));
        assert_eq!(interpolation_b.interpolate(1), Fq::from(1));
        assert_eq!(interpolation_b.interpolate(2), Fq::from(0));
        assert_eq!(interpolation_b.interpolate(3), Fq::from(10));
        assert_eq!(interpolation_b.interpolate(4), Fq::from(9));
        assert_eq!(interpolation_b.interpolate(5), Fq::from(8));
    }

    #[test]
    fn lagrange_multivar_works() {
        let two_bit_lagrange = lagrange::MultivarBasis::new([0, 0]);
        assert_eq!(two_bit_lagrange.evaluate([0, 0]), 1);
        assert_eq!(two_bit_lagrange.evaluate([0, 1]), 0);
        assert_eq!(two_bit_lagrange.evaluate([1, 0]), 0);
        assert_eq!(two_bit_lagrange.evaluate([1, 1]), 0);
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
