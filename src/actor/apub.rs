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
    actor::{kind::*, properties::*, Actor, ActorBox},
    object::{
        properties::{ApObjectProperties, ObjectProperties},
        Object, ObjectBox,
    },
    PropRefs,
};
use serde::{Deserialize, Serialize};

/// Describes a software application.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Actor)]
pub struct Application {
    #[serde(rename = "type")]
    kind: ApplicationType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activitypub object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub ap_object_props: ApObjectProperties,

    /// Adds all valid activitypub actor properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub ap_actor_props: ApActorProperties,
}

/// Represents a formal or informal collective of Actors.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Actor)]
pub struct Group {
    #[serde(rename = "type")]
    kind: GroupType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activitypub object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub ap_object_props: ApObjectProperties,

    /// Adds all valid activitypub actor properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub ap_actor_props: ApActorProperties,
}

/// Represents an organization.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Actor)]
pub struct Organization {
    #[serde(rename = "type")]
    kind: OrganizationType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activitypub object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub ap_object_props: ApObjectProperties,

    /// Adds all valid activitypub actor properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub ap_actor_props: ApActorProperties,
}

/// Represents an individual person.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Actor)]
pub struct Person {
    #[serde(rename = "type")]
    kind: PersonType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activitypub object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub ap_object_props: ApObjectProperties,

    /// Adds all valid activitypub actor properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub ap_actor_props: ApActorProperties,
}

/// Represents a service of any kind.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Actor)]
pub struct Service {
    #[serde(rename = "type")]
    kind: ServiceType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activitypub object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub ap_object_props: ApObjectProperties,

    /// Adds all valid activitypub actor properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub ap_actor_props: ApActorProperties,
}
