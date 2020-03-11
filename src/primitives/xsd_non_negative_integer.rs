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

/// The type xsd:nonNegativeInteger represents an arbitrarily large non-negative integer.
///
/// An xsd:nonNegativeInteger is a sequence of digits, optionally preceded by a + sign. Leading
/// zeros are permitted, but decimal points are not.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct XsdNonNegativeInteger(u64);

/// The error type produced when an XsdNonNegativeInteger cannot be parsed
#[derive(Clone, Debug, thiserror::Error)]
#[error("Error parsing NonNegativeInteger")]
pub struct XsdNonNegativeIntegerError;

impl AsRef<u64> for XsdNonNegativeInteger {
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}

impl From<XsdNonNegativeInteger> for u64 {
    fn from(i: XsdNonNegativeInteger) -> Self {
        i.0
    }
}

impl std::convert::TryFrom<u64> for XsdNonNegativeInteger {
    type Error = XsdNonNegativeIntegerError;

    fn try_from(f: u64) -> Result<Self, Self::Error> {
        Ok(XsdNonNegativeInteger(f))
    }
}

impl std::convert::TryFrom<String> for XsdNonNegativeInteger {
    type Error = XsdNonNegativeIntegerError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&str> for XsdNonNegativeInteger {
    type Error = XsdNonNegativeIntegerError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&mut str> for XsdNonNegativeInteger {
    type Error = XsdNonNegativeIntegerError;

    fn try_from(s: &mut str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::str::FromStr for XsdNonNegativeInteger {
    type Err = XsdNonNegativeIntegerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f = s.parse().map_err(|_| XsdNonNegativeIntegerError)?;
        Ok(XsdNonNegativeInteger(f))
    }
}

impl std::fmt::Display for XsdNonNegativeInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
