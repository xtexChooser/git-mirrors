use crate::primitives::Either;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

/// The type xsd:boolean represents logical yes/no values. The valid values for xsd:boolean are
/// true, false, 0, and 1. Values that are capitalized (e.g. TRUE) or abbreviated (e.g. T) are not
/// valid.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct XsdBoolean(pub bool);

impl XsdBoolean {
    /// Construct a new XsdBoolean
    pub fn new(b: bool) -> Self {
        Self(b)
    }

    /// Retreive the inner bool
    pub fn into_inner(self) -> bool {
        self.0
    }
}

impl PartialEq<bool> for XsdBoolean {
    fn eq(&self, other: &bool) -> bool {
        self.0 == *other
    }
}

impl PartialEq<XsdBoolean> for bool {
    fn eq(&self, other: &XsdBoolean) -> bool {
        *self == other.0
    }
}

impl Deref for XsdBoolean {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for XsdBoolean {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<bool> for XsdBoolean {
    fn as_ref(&self) -> &bool {
        &self.0
    }
}

impl AsMut<bool> for XsdBoolean {
    fn as_mut(&mut self) -> &mut bool {
        &mut self.0
    }
}

impl From<bool> for XsdBoolean {
    fn from(b: bool) -> Self {
        Self(b)
    }
}

impl From<XsdBoolean> for bool {
    fn from(b: XsdBoolean) -> Self {
        b.0
    }
}

impl<'de> Deserialize<'de> for XsdBoolean {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let helper: Either<u8, bool> = Deserialize::<'de>::deserialize(deserializer)?;

        match helper {
            Either::Left(u @ 0 | u @ 1) => Ok(XsdBoolean(u == 1)),
            Either::Right(b) => Ok(XsdBoolean(b)),
            _ => Err(serde::de::Error::custom("Invalid boolean")),
        }
    }
}

impl Serialize for XsdBoolean {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::XsdBoolean;

    #[derive(Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
    struct MyStruct {
        field: XsdBoolean,
    }

    #[test]
    fn deserialize_bool() {
        let json = r#"[{"field":true},{"field":false}]"#;

        let structs: Vec<MyStruct> = serde_json::from_str(json).unwrap();

        assert_eq!(structs[0].field, true);
        assert_eq!(structs[1].field, false);
    }

    #[test]
    fn deserialize_number() {
        let json = r#"[{"field":1},{"field":0}]"#;

        let structs: Vec<MyStruct> = serde_json::from_str(json).unwrap();

        assert_eq!(structs[0].field, true);
        assert_eq!(structs[1].field, false);
    }

    #[test]
    fn dont_deserialize_invalid_number() {
        let invalids = [
            r#"{"field":2}"#,
            r#"{"field":3}"#,
            r#"{"field":4}"#,
            r#"{"field":-1}"#,
        ];

        for case in invalids {
            assert!(serde_json::from_str::<MyStruct>(case).is_err());
        }
    }

    #[test]
    fn dont_deserialize_strings() {
        let invalids = [
            r#"{"field":"1"}"#,
            r#"{"field":"0"}"#,
            r#"{"field":"true"}"#,
            r#"{"field":"false"}"#,
        ];

        for case in invalids {
            assert!(serde_json::from_str::<MyStruct>(case).is_err());
        }
    }

    #[test]
    fn round_trip() {
        let structs = vec![
            MyStruct {
                field: XsdBoolean(false),
            },
            MyStruct {
                field: XsdBoolean(true),
            },
        ];
        let string = serde_json::to_string(&structs).unwrap();
        let new_structs: Vec<MyStruct> = serde_json::from_str(&string).unwrap();

        assert_eq!(structs, new_structs);
    }
}
