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
                for j in 0..(n as i64) { // 
                    if j == (i as i64) {
                        continue;
                    }
                    else {
                        accumulator *= ((x as i64) - j) / ((i as i64) - j);
                    }
                }
                accumulator
            }
        }
    }
}

// fn generate_all_lagrange_basis(n: usize) -> Vec<LagrangeBasis> {
//     // iterate from 0 to n
//         // generate a LagrangeBasis for that index
//     //return array(?) o' LagrangeBasis
//     unimplemented!()
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lagrange_univar_works() {
        let l_0 = LagrangeBasis::new(4, 0);
        let l_1= LagrangeBasis::new(4, 1);
        
        assert_eq!(l_0.evaluate(0), 1);
        assert_eq!(l_0.evaluate(1), 0);
        assert_eq!(l_0.evaluate(2), 0);
        assert_eq!(l_0.evaluate(3), 0);

        assert_eq!(l_1.evaluate(0), 0);
        assert_eq!(l_1.evaluate(1), 1);
        assert_eq!(l_1.evaluate(2), 0);
        assert_eq!(l_1.evaluate(3), 0);
    }
}