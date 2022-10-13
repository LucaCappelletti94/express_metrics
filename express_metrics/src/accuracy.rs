use std::iter::Sum;

use crate::*;
use num_traits::{Float, AsPrimitive};
use rayon::prelude::*;

pub struct Accuracy {}

impl<A, B, F> BinaryMetric<A, B, F> for Accuracy
where
    A: PartialEq<B> + Clone + Send + Sync,
    B: Clone + Send + Sync,
    F: Float + Send + Sum + 'static,
    usize: AsPrimitive<F>
{
    fn compute(first: &[A], second: &[B]) -> Result<F, String> {
        if first.len() != second.len() {
            return Err(format!(
                concat!(
                    "We expected the two provided vectors to have the ",
                    "same length but they have {} and {} elements, resectively."
                ),
                first.len(),
                second.len()
            ));
        }

        Ok(first
            .par_iter()
            .cloned()
            .zip(second.par_iter().cloned())
            .map(|(a, b)| if a == b { F::one() } else { F::zero() })
            .sum::<F>()
            / first.len().as_())
    }
}
