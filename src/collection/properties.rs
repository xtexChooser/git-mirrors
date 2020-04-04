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

//! Namespace for properties of standard collection types
//!
//! To use these properties in your own types, you can flatten them into your struct with serde:
//!
//! ```rust
//! use activitystreams::{
//!     collection::{
//!         properties::CollectionProperties,
//!         Collection, CollectionBox,
//!     },
//!     ext::Ext,
//!     object::{
//!         properties::ObjectProperties,
//!         Object, ObjectBox,
//!     },
//!     Base, BaseBox, PropRefs,
//! };
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
//! #[serde(transparent)]
//! pub struct ObjProps(pub ObjectProperties);
//!
//! #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
//! #[serde(transparent)]
//! pub struct CollectionProps(pub CollectionProperties);
//!
//! #[derive(Clone, Debug, Default, Serialize, Deserialize, PropRefs)]
//! #[serde(rename_all = "camelCase")]
//! #[prop_refs(Object)]
//! #[prop_refs(Collection)]
//! pub struct MyCollection {
//!     #[serde(rename = "type")]
//!     pub kind: String,
//!
//!     /// Define a require property for the MyCollection type
//!     pub my_property: String,
//!
//!     #[serde(flatten)]
//!     #[prop_refs]
//!     pub object_properties: ObjProps,
//!
//!     #[serde(flatten)]
//!     #[prop_refs]
//!     pub collection_properties: CollectionProps,
//! }
//! #
//! # fn main() {}
//! ```

use crate::{primitives::*, properties, BaseBox};

properties! {
    Collection {
        docs [
            "`Collection` objects are a specialization of the base `Object` that serve as a container for",
            "other `Objects` or `Links`.",
            "",
            "The items within a `Collection` can be ordered or unordered. The `OrderedCollection` type MAY be",
            "used to identify a `Collection` whose items are always ordered. In the JSON serialization, the",
            "unordered items of a `Collection` are represented using the `items` property while ordered items",
            "are represented using the `ordered_items` property.",
        ],

        items {
            docs [
                "Identifies the items contained in a collection. The items might be ordered or unordered.",
                "",
                "- Range: `Object` | `Link` | Ordered List of [ `Object` | `Link` ]",
                "- Functional: false",
            ],
            types [
                XsdString,
                BaseBox,
            ],
            required,
        },

        total_items {
            docs [
                "A non-negative integer specifying the total number of objects contained by the logical view",
                "of the collection.",
                "",
                "This number might not reflect the actual number of items serialized within the `Collection`",
                "object instance.",
                "",
                "- Range: `xsd:nonNegativeInteger`",
                "- Functional: true",
            ],
            types [
                XsdNonNegativeInteger,
            ],
            functional,
        },

        current {
            docs [
                "In a paged `Collection`, indicates the page that contains the most recently updated member",
                "items.",
                "",
                "- Range: `CollectionPage` | `Link`",
                "- Functional: true",
            ],
            types [
                XsdAnyUri,
                BaseBox,
            ],
            functional,
        },

        first {
            docs [
                "In a paged `Collection`, indicates the furthest preceeding page of items in the collection.",
                "",
                "- Range: `CollectionPage` | `Link`",
                "- Functional: true",
            ],
            types [
                XsdAnyUri,
                BaseBox,
            ],
            functional,
        },

        last {
            docs [
                "In a paged `Collection`, indicates the furthest proceeding page of the collection.",
                "",
                "- Range: `CollectionPage` | `Link`",
                "- Functional: true",
            ],
            types [
                XsdAnyUri,
                BaseBox,
            ],
        },
    }
}

properties! {
    CollectionPage {
        docs [
            "The `CollectionPage` type extends from the base `Collection` type and inherits all of it's",
            "properties.",
        ],

        part_of {
            docs [
                "Identifies the `Collection` to which a `CollectionPage` objects items belong.",
                "",
                "Range: `Collection` | `Link`",
                "Functional: true",
            ],
            types [
                XsdAnyUri,
                BaseBox,
            ],
            functional,
        },

        next {
            docs [
                "In a paged `Collection`, indicates the next page of items.",
                "",
                "- Range: `CollectionPage` | `Link`",
                "- Functional: true",
            ],
            types [
                XsdAnyUri,
                BaseBox,
            ],
            functional,
        },

        prev {
            docs [
                "In a paged `Collection`, identifies the previous page of items.",
                "",
                "- Range: `CollectionPage` | `Link`",
                "- Functional: true",
            ],
            types [
                XsdAnyUri,
                BaseBox,
            ],
            functional,
        },
    }
}

properties! {
    OrderedCollectionPage {
        docs ["The OrderedCollectionPage type MAY be used to identify a page whose items are strictly ordered." ],
        start_index {
            docs ["A non-negative integer value identifying the relative position within the logical view of a",
                "strictly ordered collection.",
                "",
                "- Range: `xsd:nonNegativeInteger`",
                "- Functional: true",
            ],
            types [
                XsdNonNegativeInteger,
            ],
            functional,
        },
    }
}
