use crate::either::Either;

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
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct OneOrMany<T>(pub(crate) Either<T, Vec<T>>);

impl<T> OneOrMany<T> {
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
        OneOrMany(self.0.as_ref().map(|l| l, |r| r.iter().collect()))
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
        OneOrMany(self.0.map(f, |v| v.into_iter().map(f).collect()))
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
        OneOrMany(self.0.as_mut().map(|l| l, |r| r.iter_mut().collect()))
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
        self.0.as_ref().left()
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
        self.0.as_mut().left()
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
        self.0.left()
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
    /// for item in value.unwrap_to_vec() {
    ///     println!("{:?}", item);
    /// }
    /// ```
    pub fn unwrap_to_vec(self) -> Vec<T> {
        match self.0 {
            Either::Left(t) => vec![t],
            Either::Right(v) => v,
        }
    }

    /// Produce a new object from one value
    ///
    /// ```
    /// use activitystreams::primitives::OneOrMany;
    /// let v = OneOrMany::from_one(1234);
    /// ```
    pub fn from_one(t: T) -> Self {
        OneOrMany(Either::Left(t))
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
        self.0 = Either::Left(u.into());
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
            Either::Left(one) => vec![one],
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
            Either::Left(one) => vec![one],
            Either::Right(v) => v,
        };
        v.extend(items.into_iter().map(Into::into));
        self.0 = Either::Right(v);
        self
    }
}

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
