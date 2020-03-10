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

/// A MIME Media Type consists of a top-level type and a subtype, which is further structured into
/// trees.
///
/// Optionally, media types can define companion data, known as parameters.
///
/// See [`RFC 2045`](https://tools.ietf.org/html/rfc2045) and
/// [`RFC 2046`](https://tools.ietf.org/html/rfc2046) for more information.
#[derive(Clone, Debug)]
pub struct MimeMediaType(mime::Mime);

#[derive(Clone, Debug, thiserror::Error)]
#[error("Error parsing MIME")]
/// The error type produced when a MimeMediaType cannot be parsed
pub struct MimeMediaTypeError;

impl From<mime::Mime> for MimeMediaType {
    fn from(m: mime::Mime) -> Self {
        MimeMediaType(m)
    }
}

impl From<MimeMediaType> for mime::Mime {
    fn from(m: MimeMediaType) -> Self {
        m.0
    }
}

impl AsRef<mime::Mime> for MimeMediaType {
    fn as_ref(&self) -> &mime::Mime {
        &self.0
    }
}

impl AsMut<mime::Mime> for MimeMediaType {
    fn as_mut(&mut self) -> &mut mime::Mime {
        &mut self.0
    }
}

impl std::convert::TryFrom<String> for MimeMediaType {
    type Error = MimeMediaTypeError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&str> for MimeMediaType {
    type Error = MimeMediaTypeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&mut str> for MimeMediaType {
    type Error = MimeMediaTypeError;

    fn try_from(s: &mut str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::str::FromStr for MimeMediaType {
    type Err = MimeMediaTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MimeMediaType(s.parse().map_err(|_| MimeMediaTypeError)?))
    }
}

impl serde::ser::Serialize for MimeMediaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> serde::de::Deserialize<'de> for MimeMediaType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
