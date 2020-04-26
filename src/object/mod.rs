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

//! Namespace for Object types

#[cfg(feature = "kinds")]
pub mod kind;
#[cfg(feature = "types")]
pub mod properties;
#[cfg(feature = "types")]
mod types;

#[cfg(feature = "types")]
pub use self::types::{
    Article, Audio, Document, Event, Image, Note, Page, Place, Profile, Relationship, Tombstone,
    Video,
};

/// Describes an object of any kind.
///
/// The Object type serves as the base type for most of the other kinds of objects defined in the
/// Activity Vocabulary, including other Core types such as `Activity`, `IntransitiveActivity`,
/// `Collection` and `OrderedCollection`.
#[cfg_attr(feature = "derive", crate::wrapper_type)]
pub trait Object: crate::Base {}

#[cfg(feature = "types")]
/// Describes any kind of Image
///
/// Since Image is "concrete" in the ActivityStreams spec, but multiple fields in ObjectProperties
/// require an "Image", this type acts as a filter to ensure only Images can be serialized or
/// deserialized, but allows any adjacent fields through
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AnyImage {
    #[serde(rename = "type")]
    kind: self::kind::ImageType,

    #[serde(flatten)]
    rest: std::collections::HashMap<String, serde_json::Value>,
}

#[cfg(feature = "types")]
impl AnyImage {
    pub fn from_concrete<T>(t: T) -> Result<Self, serde_json::Error>
    where
        T: Object + serde::ser::Serialize,
    {
        serde_json::from_value(serde_json::to_value(t)?)
    }

    pub fn into_concrete<T>(self) -> Result<T, serde_json::Error>
    where
        T: Object + serde::de::DeserializeOwned,
    {
        serde_json::from_value(serde_json::to_value(self)?)
    }
}
