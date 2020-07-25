/// A struct that wraps a type implementing FromStr and Display implements Serde's Deserialize and
/// Serialize
#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct SerdeParse<T>(T);

impl<T> SerdeParse<T> {
    /// Extract the inner item from SerdeParse
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> std::ops::Deref for SerdeParse<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for SerdeParse<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<T> for SerdeParse<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for SerdeParse<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> std::str::FromStr for SerdeParse<T>
where
    T: std::str::FromStr,
{
    type Err = <T as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        T::from_str(s).map(SerdeParse)
    }
}

impl<T> std::fmt::Display for SerdeParse<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> serde::ser::Serialize for SerdeParse<T>
where
    T: std::fmt::Display,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.0.to_string().serialize(serializer)
    }
}

impl<'de, T> serde::de::Deserialize<'de> for SerdeParse<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl<T> From<T> for SerdeParse<T> {
    fn from(t: T) -> Self {
        SerdeParse(t)
    }
}
