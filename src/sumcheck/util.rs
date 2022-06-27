pub mod util {
    use ark_ff::Field;
    use ark_poly::{
        multivariate::{SparsePolynomial as SparseMVPolynomial, SparseTerm, Term},
        DenseMVPolynomial, Polynomial,
    };
    use itertools::Itertools;

    /// Given a multivariate polynomial `g` over {x_1, ..., x_N}, evaluate it at all challenge points `0..x`,
    /// variable at `x` will remain the X of the univariate polynomial,
    /// and the rest of the variables will be evaluated at a boolean hypercube of size {0,1}^(g.degree() - index).
    pub fn reduce_poly_to_univar_at_x<F: Field, const N: usize>(
        g: SparseMVPolynomial<F, SparseTerm>,
        x_i: usize,
        challenges: Vec<F>,
    ) -> SparseMVPolynomial<F, SparseTerm> {
        // empty polynomial
        let mut accumulator = SparseMVPolynomial::<F, SparseTerm>::from_coefficients_slice(
            1,
            &[(F::zero(), SparseTerm::new(vec![]))],
        );
        // iterate over the boolean hypercube {0,1}^(g.degree() - x_i)
        for b in ((x_i + 1)..g.degree())
            .map(|_| 0..2u64)
            .multi_cartesian_product()
        {
            let mut partial_point: [Option<F>; N] = [None; N];

            // fill out the partial point with challenges
            for (index, element) in challenges.iter().enumerate() {
                partial_point[index] = Some(*element);
            }
            for (index, bool_elem) in b.iter().enumerate() {
                // fill out the partial point with the boolean hypercube
                partial_point[index + x_i + 1] = Some(F::from(*bool_elem));
            }
            let eval: SparseMVPolynomial<F, SparseTerm> =
                g.partial_evaluate(&partial_point.try_into().unwrap());
            accumulator += &eval;
        }
        accumulator
    }
}
