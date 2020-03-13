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

//! Collection traits and types

use crate::{
    collection::{
        kind::*, properties::*, Collection, CollectionBox, CollectionPage, CollectionPageBox,
    },
    object::{
        properties::{ApObjectProperties, ObjectProperties},
        Object, ObjectBox,
    },
    PropRefs,
};
use serde::{Deserialize, Serialize};

/// The default `Collection` type.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Collection)]
pub struct UnorderedCollection {
    #[serde(rename = "type")]
    kind: CollectionType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid ap object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub ap_object_props: ApObjectProperties,

    /// Adds all valid collection properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub collection_props: CollectionProperties,
}

/// Used to represent distinct subsets of items from a `Collection`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Collection)]
#[prop_refs(CollectionPage)]
pub struct UnorderedCollectionPage {
    #[serde(rename = "type")]
    kind: CollectionPageType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid ap object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub ap_object_props: ApObjectProperties,

    /// Adds all valid collection properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub collection_props: CollectionProperties,

    /// Adds all valid collection page properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub collection_page_props: CollectionPageProperties,
}

/// A subtype of `Collection` in which members of the logical collection are assumed to always be
/// strictly ordered.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Collection)]
pub struct OrderedCollection {
    #[serde(rename = "type")]
    kind: OrderedCollectionType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid ap object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub ap_object_props: ApObjectProperties,

    /// Adds all valid collection properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub collection_props: CollectionProperties,
}

/// Used to represent ordered subsets of items from an `OrderedCollection`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Collection)]
#[prop_refs(CollectionPage)]
pub struct OrderedCollectionPage {
    #[serde(rename = "type")]
    kind: OrderedCollectionPageType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid ap object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub ap_object_props: ApObjectProperties,

    /// Adds all valid collection properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub collection_props: CollectionProperties,

    /// Adds all valid collection page properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub collection_page_props: CollectionPageProperties,

    /// Adds all valid ordered collection page properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub ordered_collection_page_props: OrderedCollectionPageProperties,
}
