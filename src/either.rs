#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub fn left(self) -> Option<L> {
        if let Either::Left(l) = self {
            Some(l)
        } else {
            None
        }
    }

    pub fn right(self) -> Option<R> {
        if let Either::Right(r) = self {
            Some(r)
        } else {
            None
        }
    }

    pub fn as_ref(&self) -> Either<&L, &R> {
        match self {
            Either::Left(ref l) => Either::Left(l),
            Either::Right(ref r) => Either::Right(r),
        }
    }

    pub fn as_mut(&mut self) -> Either<&mut L, &mut R> {
        match self {
            Either::Left(ref mut l) => Either::Left(l),
            Either::Right(ref mut r) => Either::Right(r),
        }
    }

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
}
