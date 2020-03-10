#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum Length {
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
pub struct LengthError;

impl Length {
    pub fn is_centimeters(&self) -> bool {
        match self {
            Length::Centimeters => true,
            _ => false,
        }
    }

    pub fn is_feet(&self) -> bool {
        match self {
            Length::Feet => true,
            _ => false,
        }
    }

    pub fn is_inches(&self) -> bool {
        match self {
            Length::Inches => true,
            _ => false,
        }
    }

    pub fn is_kilometers(&self) -> bool {
        match self {
            Length::Kilometers => true,
            _ => false,
        }
    }

    pub fn is_meters(&self) -> bool {
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
