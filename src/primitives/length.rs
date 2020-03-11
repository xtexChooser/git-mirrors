/*
 * This file is part of ActivityStreams.
 *
 * Copyright © 2020 Riley Trautman
 *
 * ActivityStreams is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * ActivityStreams is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with ActivityStreams.  If not, see <http://www.gnu.org/licenses/>.
 */

/// A list of units of length that represent valid units for certain ActivityStreams objects
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
/// The error type produced when a Length cannot be parsed
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
