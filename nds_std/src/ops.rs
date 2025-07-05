/// The hardware accelerated division operator `/`.
pub trait Div<Rhs = Self> {
    /// The resulting type after applying the `/` operator.
    type Output;

    /// Performs the `/` operation.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn div(self, rhs: Rhs) -> Self::Output;
}
