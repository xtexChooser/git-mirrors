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

//! Namespace for Actor types

#[cfg(feature = "types")]
use crate::object::{properties::ObjectProperties, Object};
#[cfg(feature = "types")]
use activitystreams_derive::PropRefs;
#[cfg(feature = "types")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "kinds")]
pub mod kind;
#[cfg(feature = "types")]
use self::kind::*;

pub use activitystreams_traits::Actor;

#[cfg(feature = "types")]
/// Describes a software application.
#[derive(Clone, Debug, Default, Deserialize, PropRefs, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    kind: ApplicationType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[activitystreams(Object)]
    pub object_props: ObjectProperties,
}

#[cfg(feature = "types")]
impl Actor for Application {}

#[cfg(feature = "types")]
/// Represents a formal or informal collective of Actors.
#[derive(Clone, Debug, Default, Deserialize, PropRefs, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    kind: GroupType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[activitystreams(Object)]
    pub object_props: ObjectProperties,
}

#[cfg(feature = "types")]
impl Actor for Group {}

#[cfg(feature = "types")]
/// Represents an organization.
#[derive(Clone, Debug, Default, Deserialize, PropRefs, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    kind: OrganizationType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[activitystreams(Object)]
    pub object_props: ObjectProperties,
}

#[cfg(feature = "types")]
impl Actor for Organization {}

#[cfg(feature = "types")]
/// Represents an individual person.
#[derive(Clone, Debug, Default, Deserialize, PropRefs, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    kind: PersonType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[activitystreams(Object)]
    pub object_props: ObjectProperties,
}

#[cfg(feature = "types")]
impl Actor for Person {}

#[cfg(feature = "types")]
/// Represents a service of any kind.
#[derive(Clone, Debug, Default, Deserialize, PropRefs, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    kind: ServiceType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[activitystreams(Object)]
    pub object_props: ObjectProperties,
}

#[cfg(feature = "types")]
impl Actor for Service {}
