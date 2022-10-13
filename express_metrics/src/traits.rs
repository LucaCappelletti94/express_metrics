pub trait BinaryMetric<A, B, F> {
    fn compute(first: &[A], second: &[B]) -> Result<F, String>;
}