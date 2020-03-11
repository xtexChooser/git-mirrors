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

/// The type xsd:anyURI represents a Uniform Resource Identifier (URI) reference.
///
/// URIs are used to identify resources, and they may be absolute or relative. Absolute URIs
/// provide the entire context for locating the resources, such as http://datypic.com/prod.html.
/// Relative URIs are specified as the difference from a base URI, such as ../prod.html. It is also
/// possible to specify a fragment identifier, using the # character, such as ../prod.html#shirt.
///
/// The three previous examples happen to be HTTP URLs (Uniform Resource Locators), but URIs also
/// encompass URLs of other schemes (e.g., FTP, gopher, telnet), as well as URNs (Uniform Resource
/// Names). URIs are not required to be dereferencable; that is, it is not necessary for there to
/// be a web page at http://datypic.com/prod.html in order for this to be a valid URI.
///
/// URIs require that some characters be escaped with their hexadecimal Unicode code point preceded
/// by the % character. This includes non-ASCII characters and some ASCII characters, namely
/// control characters, spaces, and the following characters (unless they are used as deliimiters
/// in the URI): <>#%{}|\^`. For example, ../Ã©dition.html must be represented instead as
/// ../%C3%A9dition.html, with the Ã© escaped as %C3%A9. However, the anyURI type will accept these
/// characters either escaped or unescaped. With the exception of the characters % and #, it will
/// assume that unescaped characters are intended to be escaped when used in an actual URI,
/// although the schema processor will do nothing to alter them. It is valid for an anyURI value to
/// contain a space, but this practice is strongly discouraged. Spaces should instead be escaped
/// using %20.
///
/// The schema processor is not required to parse the contents of an xsd:anyURI value to determine
/// whether it is valid according to any particular URI scheme. Since the bare minimum rules for
/// valid URI references are fairly generic, the schema processor will accept most character
/// strings, including an empty value. The only values that are not accepted are ones that make
/// inappropriate use of reserved characters, such as ones that contain multiple # characters or
/// have % characters that are not followed by two hexadecimal digits.
///
/// Note that when relative URI references such as "../prod" are used as values of xsd:anyURI, no
/// attempt is made to determine or keep track of the base URI to which they may be applied. For
/// more information on URIs, see RFC 2396, Uniform Resource Identifiers (URI): Generic Syntax.
#[derive(Clone, Debug)]
pub struct XsdAnyUri(url::Url);

/// The error type produced when an XsdAnyUri cannot be parsed
#[derive(Clone, Debug, thiserror::Error)]
#[error("Could not parse XsdAnyUri")]
pub struct XsdAnyUriError;

impl From<url::Url> for XsdAnyUri {
    fn from(u: url::Url) -> Self {
        XsdAnyUri(u)
    }
}

impl From<XsdAnyUri> for url::Url {
    fn from(u: XsdAnyUri) -> Self {
        u.0
    }
}

impl Default for XsdAnyUri {
    fn default() -> Self {
        "data:text/plain,uwu".parse().unwrap()
    }
}

impl std::convert::TryFrom<String> for XsdAnyUri {
    type Error = XsdAnyUriError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&str> for XsdAnyUri {
    type Error = XsdAnyUriError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&mut str> for XsdAnyUri {
    type Error = XsdAnyUriError;

    fn try_from(s: &mut str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::str::FromStr for XsdAnyUri {
    type Err = XsdAnyUriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(XsdAnyUri(s.parse().map_err(|_| XsdAnyUriError)?))
    }
}

impl std::fmt::Display for XsdAnyUri {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl serde::ser::Serialize for XsdAnyUri {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> serde::de::Deserialize<'de> for XsdAnyUri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
