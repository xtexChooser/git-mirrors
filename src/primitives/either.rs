#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
/// Represent either of two types
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<T> Either<T, T> {
    /// Extract the inner type when Left and Right are the same
    pub fn into_inner(self) -> T {
        match self {
            Self::Left(t) => t,
            Self::Right(t) => t,
        }
    }
}

impl<L, R> Either<L, R> {
    /// Try getting just the left
    pub fn left(self) -> Option<L> {
        if let Either::Left(l) = self {
            Some(l)
        } else {
            None
        }
    }

    /// Try getting just the right
    pub fn right(self) -> Option<R> {
        if let Either::Right(r) = self {
            Some(r)
        } else {
            None
        }
    }

    /// Borrow the Left and Right
    pub fn as_ref(&self) -> Either<&L, &R> {
        match self {
            Either::Left(ref l) => Either::Left(l),
            Either::Right(ref r) => Either::Right(r),
        }
    }

    /// Mutably borrow the Left and Right
    pub fn as_mut(&mut self) -> Either<&mut L, &mut R> {
        match self {
            Either::Left(ref mut l) => Either::Left(l),
            Either::Right(ref mut r) => Either::Right(r),
        }
    }

    /// Map over the Left and Right values
    pub fn map<F1, F2, L2, R2>(self, f1: F1, f2: F2) -> Either<L2, R2>
    where
        F1: Fn(L) -> L2,
        F2: Fn(R) -> R2,
    {
        match self {
            Either::Left(l) => Either::Left((f1)(l)),
            Either::Right(r) => Either::Right((f2)(r)),
        }
    }

    /// Map just the left value
    pub fn map_left<F, L2>(self, f: F) -> Either<L2, R>
    where
        F: Fn(L) -> L2,
    {
        match self {
            Either::Left(l) => Either::Left((f)(l)),
            Either::Right(r) => Either::Right(r),
        }
    }

    /// Map just the right value
    pub fn map_right<F, R2>(self, f: F) -> Either<L, R2>
    where
        F: Fn(R) -> R2,
    {
        match self {
            Either::Left(l) => Either::Left(l),
            Either::Right(r) => Either::Right((f)(r)),
        }
    }
}
