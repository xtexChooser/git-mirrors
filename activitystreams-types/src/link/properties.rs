/*
 * This file is part of ActivityStreams Types.
 *
 * Copyright © 2018 Riley Trautman
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

//! Namespace for properties of standard link types
//!
//! To use these properties in your own types, you can flatten them into your struct with serde:
//!
//! ```rust
//! use activitystreams_traits::Link;
//! use activitystreams_types::link::properties::LinkProperties;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Clone, Debug, Serialize, Deserialize)]
//! #[serde(rename_all = "camelCase")]
//! pub struct MyLink {
//!     #[serde(rename = "type")]
//!     pub kind: String,
//!
//!     /// Define a require property for the MyLink type
//!     pub my_property: String,
//!
//!     #[serde(flatten)]
//!     pub link_properties: LinkProperties,
//! }
//!
//! impl Link for MyLink {}
//! #
//! # fn main() {}
//! ```

use activitystreams_derive::properties;

use crate::{link::LinkBox, object::ObjectBox, primitives::*};

properties! {
    Link {
        docs [
            "Define all the properties of the Object base type as described by the Activity Streams vocabulary.",
            "",
            "The properties of the `Link` object are not the properties of the referenced resource, but are",
            "provided as hints for rendering agents to understand how to make use of the resource. For",
            "example, height and width might represent the desired rendered size of a referenced image,",
            "rather than the actual pixel dimensions of the referenced image.",
            "",
            "The target URI of the Link is expressed using the required href property.",
            "",
            "For example, all Objects can contain an image property whose value describes a graphical",
            "representation of the containing object. This property will typically be used to provide the URL",
            "to an image (e.g. JPEG, GIF or PNG) resource that can be displayed to the user. Any given object",
            "might have multiple such visual representations -- multiple screenshots, for instance, or the",
            "same image at different resolutions. In Activity Streams 2.0, there are essentially three ways",
            "of describing such references.",
        ],
        id {
            docs [
                "Provides the globally unique identifier for an Object or Link.",
                "",
                "The `id` property is expressed as an absolute IRI in the spec, but for now is represented",
                "as a string.",
                "",
                "- Range: `anyUri`",
                "- Functional: true",
            ],
            types [
                XsdAnyUri,
            ],
            functional,
        },

        context {
            docs [
                "Identifies the context within which the object exists or an activity was performed.",
                "",
                "The notion of \"context\" used is intentionally vague. The intended function is to serve as a",
                "means of grouping objects and activities that share a common originating context or purpose.",
                "An example could be all activities relating to a common project or event.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                ObjectBox,
                LinkBox,
            ],
            rename("@context"),
        },

        name {
            docs [
                "A simple, human-readable, plain-text name for the object.",
                "",
                "HTML markup MUST NOT be included. The name MAY be expressed using multiple language-tagged",
                "values.",
                "",
                "- Range: `xsd:string` | `rdf:langString`",
                "- Functional: false",
            ],
            types [
                XsdString,
                RdfLangString,
            ],
        },

        href {
            docs [
                "The target resource pointed to by a Link.",
                "",
                "- Range: `xsd:anyUri`",
                "- Functional: true",
            ],
            types [
                XsdAnyUri,
            ],
            functional,
        },

        hreflang {
            docs [
                "Hints as to the language used by the target resource.",
                "",
                "Value MUST be a [[BCP47](https://tools.ietf.org/html/bcp47)]",
                "Language-Tag.",
                "",
                "- Range: [[BCP47](https://tools.ietf.org/html/bcp47)] Language",
                "Tag",
                "- Functional: true",
            ],
            types [
                XsdString,
            ],
            functional,
        },

        media_type {
            docs [
                "When used on a `Link`, identifies the MIME media type of the referenced resource.",
                "",
                "If not specified, the content property is assumed to contain text/html content.",
                "",
                "- Range: `Mime Media Type`",
                "- Functional: true",
            ],
            types [
                MimeMediaType,
            ],
            functional,
        },

        rel {
            docs [
                "A link relation associated with a Link.",
                "",
                "The value MUST conform to both the",
                "[[HTML5](https://www.w3.org/TR/html5/)] and [[RFC5988](https://tools.ietf.org/html/rfc5988)]",
                "\"link relation\" definitions.",
                "",
                "In the [[HTML5](https://www.w3.org/TR/html5/)], any string",
                "not containing the \"space\" U+0020, \"tab\" (U+0009), \"LF\" (U+000A), \"FF\" (U+000C), \"CR\"",
                "(U+000D) or \",\" (U+002C) characters can be used as a valid link relation.",
                "",
                "- Range:",
                "    [[RFC5988](https://tools.ietf.org/html/rfc5988)] or",
                "    [[HTML5](https://www.w3.org/TR/html5/)] Link Relation",
                "- Functional: false",
            ],
            types [
                XsdString,
            ],
        },

        height {
            docs ["On a `Link`, specifies a hint as to the rendering height in device-independent pixels of the",
                "linked resource.",
                "",
                "- Range: `xsd:nonNegativeInteger`",
                "- Functional: true",
            ],
            types [
                XsdNonNegativeInteger,
            ],
            functional,
        },

        width {
            docs [
                "On a `Link`, specifies a hint as to the rendering width in device-independent pixels of the",
                "linked resource.",
                "",
                "- Range: `xsd:nonNegativeInteger`",
                "- Functional: true",
            ],
            types [
                XsdNonNegativeInteger,
            ],
        },

        preview {
            docs [
                "Identifies an entity that provides a preview of this object.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                ObjectBox,
                LinkBox,
            ],
        },
    }
}
