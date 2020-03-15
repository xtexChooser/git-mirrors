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
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct XsdFloat(f64);

/// The error type produced when an XsdFloat cannot be parsed
#[derive(Clone, Debug, thiserror::Error)]
#[error("Error parsing Float")]
pub struct XsdFloatError;

impl XsdFloat {
    /// Get an f64 from the XsdFloat
    pub fn to_f64(&self) -> f64 {
        self.0
    }

    /// Get an XsdFloat from an f64
    pub fn from_f64(f: f64) -> Self {
        f.into()
    }
}

impl AsRef<f64> for XsdFloat {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

impl AsMut<f64> for XsdFloat {
    fn as_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
}

impl From<f64> for XsdFloat {
    fn from(f: f64) -> Self {
        XsdFloat(f)
    }
}

impl From<XsdFloat> for f64 {
    fn from(f: XsdFloat) -> Self {
        f.0
    }
}

impl std::convert::TryFrom<String> for XsdFloat {
    type Error = XsdFloatError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&str> for XsdFloat {
    type Error = XsdFloatError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&mut str> for XsdFloat {
    type Error = XsdFloatError;

    fn try_from(s: &mut str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::str::FromStr for XsdFloat {
    type Err = XsdFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(XsdFloat(s.parse().map_err(|_| XsdFloatError)?))
    }
}

impl std::fmt::Display for XsdFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
