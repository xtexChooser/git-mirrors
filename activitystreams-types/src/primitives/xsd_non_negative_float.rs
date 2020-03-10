/*
 * This file is part of ActivityStreams Types.
 *
 * Copyright © 2020 Riley Trautman
 *
 * ActivityStreams Types is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * ActivityStreams Types is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with ActivityStreams Types.  If not, see <http://www.gnu.org/licenses/>.
 */

/// The type xsd:float represents an IEEE single-precision 32-bit floating-point number.
///
/// TODO: handle exponents, infinity, not-a-number
///
/// The format of xsd:float values is a mantissa (a number which conforms to the type decimal)
/// followed, optionally, by the character "E" or "e" followed by an exponent. The exponent must be
/// an integer. For example, 3E2 represents 3 times 10 to the 2nd power, or 300. The exponent must
/// be an integer.
///
/// In addition, the following values are valid: INF (infinity), -INF (negative infinity), and NaN
/// (Not a Number). INF is considered to be greater than all other values, while -INF is less than
/// all other values. The value NaN cannot be compared to any other values, although it equals
/// itself.
///
/// This type also validates that a float is at least 0.0
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct XsdNonNegativeFloat(f64);

/// The error type produced when an XsdNonNegativeFloat cannot be parsed
#[derive(Clone, Debug, thiserror::Error)]
#[error("Error parsing NonNegativeFloat")]
pub struct XsdNonNegativeFloatError;

impl AsRef<f64> for XsdNonNegativeFloat {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

impl From<XsdNonNegativeFloat> for f64 {
    fn from(f: XsdNonNegativeFloat) -> Self {
        f.0
    }
}

impl std::convert::TryFrom<f64> for XsdNonNegativeFloat {
    type Error = XsdNonNegativeFloatError;

    fn try_from(f: f64) -> Result<Self, Self::Error> {
        if f < 0.0 {
            return Err(XsdNonNegativeFloatError);
        }

        Ok(XsdNonNegativeFloat(f))
    }
}

impl std::convert::TryFrom<String> for XsdNonNegativeFloat {
    type Error = XsdNonNegativeFloatError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&str> for XsdNonNegativeFloat {
    type Error = XsdNonNegativeFloatError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&mut str> for XsdNonNegativeFloat {
    type Error = XsdNonNegativeFloatError;

    fn try_from(s: &mut str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::str::FromStr for XsdNonNegativeFloat {
    type Err = XsdNonNegativeFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f = s.parse().map_err(|_| XsdNonNegativeFloatError)?;
        if f < 0.0 {
            return Err(XsdNonNegativeFloatError);
        }
        Ok(XsdNonNegativeFloat(f))
    }
}

impl std::fmt::Display for XsdNonNegativeFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
