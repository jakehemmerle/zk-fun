pub mod univar {
    pub struct LagrangeBasis {
        n: usize,
        i: usize,
        basis: fn(x: usize, n: usize, i: usize) -> i64,
    }

    impl LagrangeBasis {
        pub fn evaluate(&self, point: usize) -> i64 {
            if point >= self.n {
                panic!("i must be less than n");
            }
            (self.basis)(point, self.n, self.i)
        }

        pub fn new(n: usize, i: usize) -> Self {
            LagrangeBasis {
                n,
                i,
                basis: |x, n, i| -> i64 {
                    let mut accumulator: i64 = 1;
                    for j in 0..(n as i64) {
                        //
                        if j == (i as i64) {
                            continue;
                        } else {
                            accumulator *= ((x as i64) - j) / ((i as i64) - j);
                        }
                    }
                    accumulator
                },
            }
        }
    }
}

pub mod multivar {
    #[allow(dead_code)]
    pub struct LagrangeBasis<const N: usize> {
        w: [i64; N],
        basis: fn(x: [i64; N], w: [i64; N]) -> i64,
    }

    impl<const N: usize> LagrangeBasis<N> {
        pub fn new(w: [i64; N]) -> Self {
            LagrangeBasis {
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
    use super::*;
    #[test]
    fn lagrange_univar_works() {
        let l_0 = univar::LagrangeBasis::new(4, 0);
        let l_1 = univar::LagrangeBasis::new(4, 1);
        let l_2 = univar::LagrangeBasis::new(4, 2);
        let l_3 = univar::LagrangeBasis::new(4, 3);

        assert_eq!(l_0.evaluate(0), 1);
        assert_eq!(l_0.evaluate(1), 0);
        assert_eq!(l_0.evaluate(2), 0);
        assert_eq!(l_0.evaluate(3), 0);

        assert_eq!(l_1.evaluate(0), 0);
        assert_eq!(l_1.evaluate(1), 1);
        assert_eq!(l_1.evaluate(2), 0);
        assert_eq!(l_1.evaluate(3), 0);

        assert_eq!(l_2.evaluate(0), 0);
        assert_eq!(l_2.evaluate(1), 0);
        assert_eq!(l_2.evaluate(2), 1);
        assert_eq!(l_2.evaluate(3), 0);

        assert_eq!(l_3.evaluate(0), 0);
        assert_eq!(l_3.evaluate(1), 0);
        assert_eq!(l_3.evaluate(2), 0);
        assert_eq!(l_3.evaluate(3), 1);
    }

    #[test]
    #[should_panic]
    fn lagrange_univar_panics_at_n() {
        let l = univar::LagrangeBasis::new(4, 0);
        l.evaluate(4);
    }

    #[test]
    fn lagrange_multivar_works() {
        let two_bit_lagrange = multivar::LagrangeBasis::new([0, 0]);
        assert_eq!(two_bit_lagrange.evaluate([0, 0]), 1);
        assert_eq!(two_bit_lagrange.evaluate([0, 1]), 0);
        assert_eq!(two_bit_lagrange.evaluate([1, 0]), 0);
        assert_eq!(two_bit_lagrange.evaluate([1, 1]), 0);
    }
}
