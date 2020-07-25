use crate::either::Either;

/// A type representing units of length
///
/// It can be any of the following
/// - Centimeters
/// - Meters
/// - Kilometers
/// - Inches
/// - Feet
/// - A custom value
#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(transparent)]
pub struct Unit(Either<Length, String>);

impl Unit {
    /// Create a new unit measuring Centimeters
    ///
    /// ```rust
    /// use activitystreams::primitives::Unit;
    ///
    /// Unit::centimeters();
    /// ```
    pub fn centimeters() -> Self {
        Unit(Either::Left(Length::Centimeters))
    }

    /// Check if the unit is Centimeters
    ///
    /// ```rust
    /// use activitystreams::primitives::Unit;
    ///
    /// assert!(Unit::centimeters().is_centimeters());
    /// ```
    pub fn is_centimeters(&self) -> bool {
        self.0
            .as_ref()
            .left()
            .map(|l| l.is_centimeters())
            .unwrap_or(false)
    }

    /// Create a new unit measuring Meters
    ///
    /// ```rust
    /// use activitystreams::primitives::Unit;
    ///
    /// Unit::meters();
    /// ```
    pub fn meters() -> Self {
        Unit(Either::Left(Length::Meters))
    }

    /// Check if the unit is Meters
    ///
    /// ```rust
    /// use activitystreams::primitives::Unit;
    ///
    /// assert!(Unit::meters().is_meters());
    /// ```
    pub fn is_meters(&self) -> bool {
        self.0
            .as_ref()
            .left()
            .map(|l| l.is_meters())
            .unwrap_or(false)
    }

    /// Create a new unit measuring Kilometers
    ///
    /// ```rust
    /// use activitystreams::primitives::Unit;
    ///
    /// Unit::kilometers();
    /// ```
    pub fn kilometers() -> Self {
        Unit(Either::Left(Length::Kilometers))
    }

    /// Check if the unit is Kilometers
    ///
    /// ```rust
    /// use activitystreams::primitives::Unit;
    ///
    /// assert!(Unit::kilometers().is_kilometers());
    /// ```
    pub fn is_kilometers(&self) -> bool {
        self.0
            .as_ref()
            .left()
            .map(|l| l.is_kilometers())
            .unwrap_or(false)
    }

    /// Create a new unit measuring Feet
    ///
    /// ```rust
    /// use activitystreams::primitives::Unit;
    ///
    /// Unit::feet();
    /// ```
    pub fn feet() -> Self {
        Unit(Either::Left(Length::Feet))
    }

    /// Check if the unit is Feet
    ///
    /// ```rust
    /// use activitystreams::primitives::Unit;
    ///
    /// assert!(Unit::feet().is_feet());
    /// ```
    pub fn is_feet(&self) -> bool {
        self.0.as_ref().left().map(|l| l.is_feet()).unwrap_or(false)
    }

    /// Create a new unit measuring Inches
    ///
    /// ```rust
    /// use activitystreams::primitives::Unit;
    ///
    /// Unit::inches();
    /// ```
    pub fn inches() -> Self {
        Unit(Either::Left(Length::Inches))
    }

    /// Check if the unit is Inches
    ///
    /// ```rust
    /// use activitystreams::primitives::Unit;
    ///
    /// assert!(Unit::inches().is_inches());
    /// ```
    pub fn is_inches(&self) -> bool {
        self.0
            .as_ref()
            .left()
            .map(|l| l.is_inches())
            .unwrap_or(false)
    }

    /// Create a new custom unit
    ///
    /// ```rust
    /// use activitystreams::primitives::Unit;
    ///
    /// Unit::custom("Yards");
    /// ```
    pub fn custom<T>(string: T) -> Self
    where
        T: Into<String>,
    {
        Unit(Either::Right(string.into()))
    }

    /// Check if a unit is custom
    ///
    /// ```rust
    /// use activitystreams::primitives::Unit;
    ///
    /// assert!(Unit::custom("Yards").is_custom());
    /// ```
    pub fn is_custom(&self) -> bool {
        self.as_custom().is_some()
    }

    /// Fetch a custom unit
    ///
    /// ```rust
    /// use activitystreams::primitives::Unit;
    ///
    /// assert!(Unit::custom("Yards").as_custom() == Some("Yards"));
    /// ```
    pub fn as_custom(&self) -> Option<&str> {
        self.0.as_ref().right().map(|r| r.as_str())
    }
}

impl Default for Unit {
    fn default() -> Self {
        Self::meters()
    }
}

impl std::str::FromStr for Unit {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let unit = match s {
            "cm" => Self::centimeters(),
            "m" => Self::meters(),
            "km" => Self::kilometers(),
            "inches" => Self::inches(),
            "feet" => Self::feet(),
            other => Self::custom(other),
        };

        Ok(unit)
    }
}

impl From<String> for Unit {
    fn from(s: String) -> Self {
        match s.parse() {
            Ok(u) => u,
            Err(e) => match e {},
        }
    }
}

impl From<&str> for Unit {
    fn from(s: &str) -> Self {
        match s.parse() {
            Ok(u) => u,
            Err(e) => match e {},
        }
    }
}

/// A list of units of length that represent valid units for certain ActivityStreams objects
#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
enum Length {
    #[serde(rename = "cm")]
    Centimeters,

    #[serde(rename = "feet")]
    Feet,

    #[serde(rename = "inches")]
    Inches,

    #[serde(rename = "km")]
    Kilometers,

    #[serde(rename = "m")]
    Meters,
}

#[derive(Clone, Debug, thiserror::Error)]
#[error("Could not parse units")]
/// The error type produced when a Length cannot be parsed
struct LengthError;

impl Length {
    fn is_centimeters(&self) -> bool {
        match self {
            Length::Centimeters => true,
            _ => false,
        }
    }

    fn is_feet(&self) -> bool {
        match self {
            Length::Feet => true,
            _ => false,
        }
    }

    fn is_inches(&self) -> bool {
        match self {
            Length::Inches => true,
            _ => false,
        }
    }

    fn is_kilometers(&self) -> bool {
        match self {
            Length::Kilometers => true,
            _ => false,
        }
    }

    fn is_meters(&self) -> bool {
        match self {
            Length::Meters => true,
            _ => false,
        }
    }
}

impl Default for Length {
    fn default() -> Self {
        Length::Meters
    }
}

impl std::str::FromStr for Length {
    type Err = LengthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cm" => Ok(Length::Centimeters),
            "feet" => Ok(Length::Feet),
            "inches" => Ok(Length::Inches),
            "km" => Ok(Length::Kilometers),
            "m" => Ok(Length::Meters),
            _ => Err(LengthError),
        }
    }
}

impl std::convert::TryFrom<&str> for Length {
    type Error = LengthError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&mut str> for Length {
    type Error = LengthError;

    fn try_from(s: &mut str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<String> for Length {
    type Error = LengthError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::fmt::Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Length::Centimeters => write!(f, "cm"),
            Length::Feet => write!(f, "feet"),
            Length::Inches => write!(f, "inches"),
            Length::Kilometers => write!(f, "km"),
            Length::Meters => write!(f, "meters"),
        }
    }
}
