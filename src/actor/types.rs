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

use crate::{
    actor::{kind::*, properties::ApActorProperties, Actor, ActorBox},
    ext::Ext,
    object::{
        properties::{ApObjectProperties, ObjectProperties},
        Object, ObjectBox,
    },
    Base, Extensible, PropRefs,
};

/// Describes a software application.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Actor)]
#[extension(ApObjectProperties)]
#[extension(ApActorProperties)]
pub struct Application {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    kind: ApplicationType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,
}

/// Represents a formal or informal collective of Actors.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Actor)]
#[extension(ApObjectProperties)]
#[extension(ApActorProperties)]
pub struct Group {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    kind: GroupType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,
}

/// Represents an organization.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    kind: OrganizationType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,
}

/// Represents an individual person.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Actor)]
#[extension(ApObjectProperties)]
#[extension(ApActorProperties)]
pub struct Person {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    kind: PersonType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,
}

/// Represents a service of any kind.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Actor)]
#[extension(ApObjectProperties)]
#[extension(ApActorProperties)]
pub struct Service {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    kind: ServiceType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,
}
