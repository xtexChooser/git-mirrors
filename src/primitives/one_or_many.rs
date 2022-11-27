use crate::primitives::Either;

/// A type representing at least one value
///
/// When translated to JSON, it can represent the following structures:
/// ```json
/// {
///     "key": value
/// }
/// ```
/// ```json
/// {
///     "key": [],
/// }
/// ```
/// ```json
/// {
///     "key": [value, ...]
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OneOrMany<T>(pub(crate) Either<[T; 1], Vec<T>>);

/// An iterator over a OneOrMany's borrowed contents
#[derive(Clone, Debug)]
pub struct Iter<'a, T>(Either<Option<&'a T>, std::slice::Iter<'a, T>>);

/// An iterator over a OneOrMany's mutably borrowed contents
#[derive(Debug)]
pub struct IterMut<'a, T>(Either<Option<&'a mut T>, std::slice::IterMut<'a, T>>);

/// An iterator consuming a OneOrMany
#[derive(Clone, Debug)]
pub struct IntoIter<T>(Either<Option<T>, std::vec::IntoIter<T>>);

impl<T> OneOrMany<T> {
    /// Construct an iterator over borrows of the OneOrMany's contents
    ///
    /// ```rust
    /// use activitystreams::primitives::OneOrMany;
    ///
    /// let value = OneOrMany::from_one(String::from("hi"));
    ///
    /// for item in value.iter() {
    ///     println!("{}", item);
    /// }
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        match self.0 {
            Either::Left([ref t]) => Iter(Either::Left(Some(t))),
            Either::Right(ref v) => Iter(Either::Right(v.iter())),
        }
    }

    /// Construct an iterator over mutable borrows of the OneOrMany's contents
    ///
    /// ```rust
    /// use activitystreams::primitives::OneOrMany;
    ///
    /// let mut value = OneOrMany::from_one(String::from("hi"));
    ///
    /// for item in value.iter_mut() {
    ///     item.push_str("hey");
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        match self.0 {
            Either::Left([ref mut t]) => IterMut(Either::Left(Some(t))),
            Either::Right(ref mut v) => IterMut(Either::Right(v.iter_mut())),
        }
    }

    /// Create a OneOrMany referencing the existing one
    ///
    /// ```rust
    /// use activitystreams::primitives::OneOrMany;
    ///
    /// let value = OneOrMany::from_one(String::from("hi"));
    /// let value_ref = value.as_ref();
    ///
    /// assert_eq!(value_ref.one(), Some(&String::from("hi")));
    /// ```
    pub fn as_ref(&self) -> OneOrMany<&T> {
        OneOrMany(self.0.as_ref().map(|[l]| [l], |r| r.iter().collect()))
    }

    /// Map the value inside the OneOrMany from T to U
    ///
    /// ```rust
    /// use activitystreams::primitives::OneOrMany;
    ///
    /// let value = OneOrMany::from_one("Jake from StateFarm");
    /// let new_value = value.map(|s| format!("Hi, {}", s));
    ///
    /// assert_eq!(new_value.one(), Some(String::from("Hi, Jake from StateFarm")));
    /// ```
    pub fn map<F, U>(self, f: F) -> OneOrMany<U>
    where
        F: Fn(T) -> U + Copy,
    {
        OneOrMany(self.0.map(|[l]| [f(l)], |v| v.into_iter().map(f).collect()))
    }

    /// Create a OneOrMany mutably referencing the existing one
    ///
    /// ```rust
    /// use activitystreams::primitives::OneOrMany;
    ///
    /// let mut value = OneOrMany::from_one(5);
    /// let value_mut = value.as_mut();
    /// ```
    pub fn as_mut(&mut self) -> OneOrMany<&mut T> {
        OneOrMany(self.0.as_mut().map(|[l]| [l], |r| r.iter_mut().collect()))
    }

    /// Get a reference to a single value
    ///
    /// ```rust
    /// # use activitystreams::primitives::OneOrMany;
    /// # let value = OneOrMany::from_one(1);
    /// if let Some(v) = value.as_one() {
    ///     println!("{:?}", v);
    /// }
    /// ```
    pub fn as_one(&self) -> Option<&T> {
        self.0.as_ref().left().map(|[t]| t)
    }

    /// Borrow one as mutable
    ///
    /// ```rust
    /// use activitystreams::primitives::OneOrMany;
    ///
    /// let mut value = OneOrMany::from_one(1);
    ///
    /// if let Some(i) = value.one_mut() {
    ///     *i += 1;
    /// }
    ///
    /// assert_eq!(value.one(), Some(2));
    /// ```
    pub fn one_mut(&mut self) -> Option<&mut T> {
        self.0.as_mut().left().map(|[t]| t)
    }

    /// Take a single value
    ///
    /// ```rust
    /// # use activitystreams::primitives::OneOrMany;
    /// # let value = OneOrMany::from_one(1);
    /// if let Some(v) = value.one() {
    ///     println!("{:?}", v);
    /// }
    /// ```
    pub fn one(self) -> Option<T> {
        self.0.left().map(|[t]| t)
    }

    /// Get a slice of values
    ///
    /// ```rust
    /// # use activitystreams::primitives::OneOrMany;
    /// # let value = OneOrMany::from_many(vec![1, 2, 3]);
    /// if let Some(v) = value.as_many() {
    ///     for item in v.iter() {
    ///         println!("{:?}", item);
    ///     }
    /// }
    /// ```
    pub fn as_many(&self) -> Option<&[T]> {
        self.0.as_ref().right().map(|v| v.as_ref())
    }

    /// Borrow many as mutable
    ///
    /// ```rust
    /// use activitystreams::primitives::OneOrMany;
    ///
    /// let mut value = OneOrMany::from_many(vec![1, 2, 3]);
    ///
    /// if let Some(v) = value.many_mut() {
    ///     for i in v.iter_mut() {
    ///         *i += 3;
    ///     }
    /// }
    ///
    /// assert_eq!(value.many(), Some(vec![4, 5, 6]));
    /// ```
    pub fn many_mut(&mut self) -> Option<&mut [T]> {
        self.0.as_mut().right().map(|v| v.as_mut())
    }

    /// Take a Vec of values
    ///
    /// ```rust
    /// # use activitystreams::primitives::OneOrMany;
    /// # let value = OneOrMany::from_many(vec![1, 2, 3]);
    /// if let Some(v) = value.many() {
    ///     for item in v.into_iter() {
    ///         println!("{:?}", item);
    ///     }
    /// }
    /// ```
    pub fn many(self) -> Option<Vec<T>> {
        self.0.right()
    }

    /// Consume the type, returning a vec
    ///
    /// ```rust
    /// # use activitystreams::primitives::OneOrMany;
    /// # let value = OneOrMany::from_many(vec![1, 2, 3]);
    /// for item in value.into_vec() {
    ///     println!("{:?}", item);
    /// }
    /// ```
    pub fn into_vec(self) -> Vec<T> {
        match self.0 {
            Either::Left(t) => t.into(),
            Either::Right(v) => v,
        }
    }

    /// Return a slice of values contained by the OneOrMany
    ///
    /// ```rust
    /// # use activitystreams::primitives::OneOrMany;
    /// # let value = OneOrMany::from_many(vec![1, 2, 3]);
    /// for item in value.as_slice() {
    ///     println!("{:?}", item)
    /// }
    /// ```
    pub fn as_slice(&self) -> &[T] {
        match self.0 {
            Either::Left(ref t) => t,
            Either::Right(ref v) => v,
        }
    }

    /// Produce a new object from one value
    ///
    /// ```
    /// use activitystreams::primitives::OneOrMany;
    /// let v = OneOrMany::from_one(1234);
    /// ```
    pub fn from_one(t: T) -> Self {
        OneOrMany(Either::Left([t]))
    }

    /// Produce a new object from a vec of values
    ///
    /// ```
    /// use activitystreams::primitives::OneOrMany;
    /// let v = OneOrMany::from_many(vec![1, 2, 3, 4]);
    /// ```
    pub fn from_many(items: Vec<T>) -> Self {
        OneOrMany(Either::Right(items))
    }

    /// Overwrite the contents with a single value
    ///
    /// ```
    /// # use activitystreams::primitives::OneOrMany;
    /// # let mut value = OneOrMany::from_many(vec![1, 2, 3]);
    /// value.set_one(3);
    ///
    /// assert!(value.as_one().is_some());
    /// ```
    pub fn set_one<U>(&mut self, u: U) -> &mut Self
    where
        U: Into<T>,
    {
        self.0 = Either::Left([u.into()]);
        self
    }

    /// Overwrite the contents with vec of values
    ///
    /// ```
    /// # use activitystreams::primitives::OneOrMany;
    /// # let mut value = OneOrMany::from_one(1234);
    /// value.set_many(vec![1, 2, 3, 4]);
    ///
    /// assert!(value.as_many().is_some());
    /// ```
    pub fn set_many<U>(&mut self, items: impl IntoIterator<Item = U>) -> &mut Self
    where
        U: Into<T>,
    {
        self.0 = Either::Right(items.into_iter().map(Into::into).collect());
        self
    }

    /// Add a value to the object
    ///
    /// This appends the value to the existing vec, or converts the single value into a vec, and
    /// then appends the new value
    ///
    /// ```
    /// use activitystreams::primitives::OneOrMany;
    /// let mut value = OneOrMany::from_one(1234);
    /// value.add(4321);
    ///
    /// assert!(value.as_many().is_some());
    /// ```
    pub fn add<U>(&mut self, u: U) -> &mut Self
    where
        U: Into<T>,
    {
        let mut v = match std::mem::replace(&mut self.0, Either::Right(vec![])) {
            Either::Left(one) => one.into(),
            Either::Right(v) => v,
        };
        v.push(u.into());
        self.0 = Either::Right(v);
        self
    }

    /// Add many values to the object
    ///
    /// This appends the values to the existing vec, or converts the single value into a vec, and
    /// then appends the new values
    ///
    /// ```
    /// use activitystreams::primitives::OneOrMany;
    /// let mut value = OneOrMany::from_one(1234);
    /// value.add_many(vec![4321, 2345]);
    ///
    /// assert!(value.as_many().is_some());
    /// ```
    pub fn add_many<U>(&mut self, items: impl IntoIterator<Item = U>) -> &mut Self
    where
        U: Into<T>,
    {
        let mut v = match std::mem::replace(&mut self.0, Either::Right(vec![])) {
            Either::Left(one) => one.into(),
            Either::Right(v) => v,
        };
        v.extend(items.into_iter().map(Into::into));
        self.0 = Either::Right(v);
        self
    }
}

impl<T> IntoIterator for OneOrMany<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Construct an iterator over the OneOrMany's contents, consuming the OneOrMany
    ///
    /// ```rust
    /// use activitystreams::primitives::OneOrMany;
    ///
    /// let value = OneOrMany::from_one(String::from("hi"));
    /// let vec = value.into_iter().map(|s| s + "hello").collect::<Vec<_>>();
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        match self.0 {
            Either::Left([t]) => IntoIter(Either::Left(Some(t))),
            Either::Right(v) => IntoIter(Either::Right(v.into_iter())),
        }
    }
}

impl<'a, T> IntoIterator for &'a OneOrMany<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut OneOrMany<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Either::Left(ref mut opt) => opt.take(),
            Either::Right(ref mut iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            Either::Left(ref opt) if opt.is_some() => (1, Some(1)),
            Either::Left(_) => (0, Some(0)),
            Either::Right(ref iter) => iter.size_hint(),
        }
    }

    fn count(self) -> usize {
        match self.0 {
            Either::Left(opt) => opt.map_or(0, |_| 1),
            Either::Right(iter) => iter.count(),
        }
    }

    fn last(self) -> Option<Self::Item> {
        match self.0 {
            Either::Left(mut opt) => opt.take(),
            Either::Right(iter) => iter.last(),
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match self.0 {
            Either::Left(ref mut opt) if n == 0 => opt.take(),
            Either::Left(_) => None,
            Either::Right(ref mut iter) => iter.nth(n),
        }
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.0 {
            Either::Left(ref mut opt) => opt.take(),
            Either::Right(ref mut iter) => iter.next_back(),
        }
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        match self.0 {
            Either::Left(ref mut opt) if n == 0 => opt.take(),
            Either::Left(_) => None,
            Either::Right(ref mut iter) => iter.nth_back(n),
        }
    }
}

impl<'a, T> std::iter::FusedIterator for Iter<'a, T> {}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Either::Left(ref mut opt) => opt.take(),
            Either::Right(ref mut iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            Either::Left(ref opt) if opt.is_some() => (1, Some(1)),
            Either::Left(_) => (0, Some(0)),
            Either::Right(ref iter) => iter.size_hint(),
        }
    }

    fn count(self) -> usize {
        match self.0 {
            Either::Left(opt) => opt.map_or(0, |_| 1),
            Either::Right(iter) => iter.count(),
        }
    }

    fn last(self) -> Option<Self::Item> {
        match self.0 {
            Either::Left(mut opt) => opt.take(),
            Either::Right(iter) => iter.last(),
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match self.0 {
            Either::Left(ref mut opt) if n == 0 => opt.take(),
            Either::Left(_) => None,
            Either::Right(ref mut iter) => iter.nth(n),
        }
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.0 {
            Either::Left(ref mut opt) => opt.take(),
            Either::Right(ref mut iter) => iter.next_back(),
        }
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        match self.0 {
            Either::Left(ref mut opt) if n == 0 => opt.take(),
            Either::Left(_) => None,
            Either::Right(ref mut iter) => iter.nth_back(n),
        }
    }
}

impl<'a, T> std::iter::FusedIterator for IterMut<'a, T> {}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Either::Left(ref mut opt) => opt.take(),
            Either::Right(ref mut iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            Either::Left(ref opt) if opt.is_some() => (1, Some(1)),
            Either::Left(_) => (0, Some(0)),
            Either::Right(ref iter) => iter.size_hint(),
        }
    }

    fn count(self) -> usize {
        match self.0 {
            Either::Left(opt) => opt.map_or(0, |_| 1),
            Either::Right(iter) => iter.count(),
        }
    }

    fn last(self) -> Option<Self::Item> {
        match self.0 {
            Either::Left(mut opt) => opt.take(),
            Either::Right(iter) => iter.last(),
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match self.0 {
            Either::Left(ref mut opt) if n == 0 => opt.take(),
            Either::Left(_) => None,
            Either::Right(ref mut iter) => iter.nth(n),
        }
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.0 {
            Either::Left(ref mut opt) => opt.take(),
            Either::Right(ref mut iter) => iter.next_back(),
        }
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        match self.0 {
            Either::Left(ref mut opt) if n == 0 => opt.take(),
            Either::Left(_) => None,
            Either::Right(ref mut iter) => iter.nth_back(n),
        }
    }
}

impl<T> std::iter::FusedIterator for IntoIter<T> {}

impl<T> From<T> for OneOrMany<T> {
    fn from(t: T) -> Self {
        OneOrMany::from_one(t)
    }
}

impl<T> From<Vec<T>> for OneOrMany<T> {
    fn from(t: Vec<T>) -> Self {
        OneOrMany::from_many(t)
    }
}

impl<'de, T> serde::de::Deserialize<'de> for OneOrMany<T>
where
    T: serde::de::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct OneOrManyInner<T>(Either<T, Vec<T>>);

        OneOrManyInner::deserialize(deserializer).map(|inner| match inner.0 {
            Either::Left(one) => OneOrMany(Either::Left([one])),
            Either::Right(vec) => OneOrMany(Either::Right(vec)),
        })
    }
}

impl<T> serde::Serialize for OneOrMany<T>
where
    T: serde::ser::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(serde::Serialize)]
        struct OneOrManyInner<'a, T>(Either<&'a T, &'a [T]>);
        let to_ser = match self.0 {
            Either::Left([ref one]) => OneOrManyInner(Either::Left(one)),
            Either::Right(ref v) => OneOrManyInner(Either::Right(v)),
        };

        to_ser.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::OneOrMany;

    #[test]
    fn ser_de() {
        #[derive(Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
        struct Hi {
            inner: OneOrMany<String>,
        }

        let h1 = Hi {
            inner: OneOrMany::from_one(String::from("hello")),
        };
        let s = serde_json::to_string(&h1).unwrap();
        assert_eq!(s, r#"{"inner":"hello"}"#);

        let h2: Hi = serde_json::from_str(&s).unwrap();
        assert_eq!(h2, h1);

        let h1 = Hi {
            inner: OneOrMany::from_many(vec![String::from("hello"), String::from("hi")]),
        };
        let s = serde_json::to_string(&h1).unwrap();
        assert_eq!(s, r#"{"inner":["hello","hi"]}"#);

        let h2: Hi = serde_json::from_str(&s).unwrap();
        assert_eq!(h2, h1);
    }

    #[test]
    fn iter_works() {
        let single = OneOrMany::from_one(1);
        assert_eq!(single.iter().collect::<Vec<_>>(), vec![&1]);

        let many = OneOrMany::from_many(vec![1, 2, 3]);
        assert_eq!(many.iter().collect::<Vec<_>>(), vec![&1, &2, &3]);
    }

    #[test]
    fn iter_mut_works() {
        let mut single = OneOrMany::from_one(1);
        for item in single.iter_mut() {
            *item += 1;
        }
        assert_eq!(single.as_one(), Some(&2));

        let mut many = OneOrMany::from_many(vec![1, 2, 3]);
        for item in many.iter_mut() {
            *item += 1;
        }
        assert_eq!(many.as_many(), Some(&[2, 3, 4][..]));
    }

    #[test]
    fn into_iter_works() {
        let single = OneOrMany::from_one(1);
        let v = single.into_iter().collect::<Vec<_>>();
        assert_eq!(v, vec![1]);

        let many = OneOrMany::from_many(vec![1, 2, 3]);
        let v = many.into_iter().collect::<Vec<_>>();
        assert_eq!(v, vec![1, 2, 3]);
    }
}
