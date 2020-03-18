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

//! Namespace for properties of standard object types
//!
//! To use these properties in your own types, you can flatten them into your struct with serde:
//!
//! ```rust
//! use activitystreams::{
//!     object::{
//!         properties::ObjectProperties,
//!         Object, ObjectBox,
//!     },
//!     Base, BaseBox, PropRefs,
//! };
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Clone, Debug, Serialize, Deserialize, PropRefs)]
//! #[serde(rename_all = "camelCase")]
//! #[prop_refs(Object)]
//! pub struct MyObject {
//!     #[serde(rename = "type")]
//!     pub kind: String,
//!
//!     /// Define a required property for the MyObject type
//!     pub my_property: String,
//!
//!     #[serde(flatten)]
//!     #[prop_refs]
//!     pub object_properties: ObjectProperties,
//! }
//! #
//! # fn main() {}
//! ```

use crate::{
    ext::Extension,
    link::LinkBox,
    object::{AnyImage, Object, ObjectBox},
    primitives::*,
    properties, BaseBox,
};

impl<T> Extension<T> for ApObjectProperties where T: Object {}

properties! {
    Object {
        docs [
            "Define all the properties of the Object base type as described by the Activity Streams",
            "vocabulary.",
            "",
            "In addition to having a global identifier (expressed as an absolute IRI using the id property)",
            "and an \"object type\"(expressed using the type property), all instances of the Object type share",
            "a common set of properties normatively defined by the Activity Vocabulary.",
            "",
            "This struct does not provide an optional `type` property, if you are implementing your own",
            "object type, you must supply your own type. This crate's provided object types all supply their",
            "own `type` properties as Unit Structs with custom serde behaviour.",
            "",
            "All properties are optional (including the id and type).",
        ],
        id {
            docs [
                "Provides the globally unique identifier for an Object or Link.",
                "",
                "The `id` property is expressed as an absolute IRI in the spec, but for now is represented",
                "as a string.",
                "",
                "- Range: `xsd:anyUri`",
                "- Functional: true",
            ],
            types [
                XsdAnyUri,
            ],
            functional,
            alias [ "@id" ],
        },

        attachment {
            docs [
                "Identifies a resource attached or related to an object that potentially requires special",
                "handling.",
                "",
                "The intent is to provide a model that is at least semantically similar to attachments in",
                "email.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                BaseBox,
            ],
        },

        attributed_to {
            docs [
                "Identifies one or more entities to which this object is attributed.",
                "",
                "The attributed entities might not be Actors. For instance, an object might be attributed to",
                "the completion of another activity.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                BaseBox,
            ],
        },

        audience {
            docs [
                "Identifies one or more entities that represent the total population of entities for which",
                "the object can considered to be relevant.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                BaseBox,
            ],
        },

        content {
            docs [
                "The content or textual representation of the Object encoded as a JSON string.",
                "",
                "By default, the value of content is HTML. The mediaType property can be used in the object",
                "to indicate a different content type.",
                "",
                "The content MAY be expressed using multiple language-tagged values.",
                "",
                "- Range: `xsd:string` | `rdf:langString`",
                "- Functional: false",
            ],
            types [
                XsdString,
                RdfLangString,
            ],
        },

        context {
            docs [
                "Identifies the context within which the object exists or an activity was performed.",
                "",
                "The notion of \"context\"used is intentionally vague. The intended function is to serve as a",
                "means of grouping objects and activities that share a common originating context or purpose.",
                "An example could be all activities relating to a common project or event.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                BaseBox,
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
            alias [ "displayName" ],
        },

        end_time {
            docs [
                "The date and time describing the actual or expected ending time of the object.",
                "",
                "When used with an Activity object, for instance, the endTime property specifies the moment",
                "the activity concluded or is expected to conclude.",
                "",
                "- Range: `xsd:dateTime`",
                "- Functional: true",
            ],
            types [
                XsdDateTime,
            ],
            functional,
        },

        generator {
            docs [
                "Identifies the entity (e.g. an application) that generated the object.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                BaseBox,
            ],
        },

        icon {
            docs [
                "Indicates an entity that describes an icon for this object.",
                "",
                "The image should have an aspect ratio of one (horizontal) to one (vertical) and should be",
                "suitable for presentation at a small size.",
                "",
                "- Range: `Image` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                AnyImage,
                LinkBox,
            ],
        },

        image {
            docs [
                "Indicates an entity that describes an image for this object.",
                "",
                "Unlike the icon property, there are no aspect ratio or display size limitations assumed.",
                "",
                "- Range: `Image` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                AnyImage,
                LinkBox,
            ],
        },

        in_reply_to {
            docs [
                "Indicates one or more entities for which this object is considered a response.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                BaseBox,
            ],
        },

        location {
            docs [
                "Indicates one or more physical or logical locations associated with the object.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                BaseBox,
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
                BaseBox,
            ],
        },

        published {
            docs [
                "The date and time at which the object was published.",
                "",
                "- Range: `xsd:dateTime`",
                "- Functional: true",
            ],
            types [
                XsdDateTime,
            ],
            functional,
        },

        replies {
            docs [
                "Identifies a `Collection` containing objects considered to be responses to this object.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                BaseBox,
            ],
        },

        start_time {
            docs [
                "The date and time describing the actual or expected starting time of the object.",
                "",
                "When used with an `Activity` object, for instance, the `start_time` property specifies the",
                "moment the activity began or is scheduled to begin.",
                "",
                "- Range: `xsd:DateTime`",
                "- Functional: true",
            ],
            types [
                XsdDateTime,
            ],
            functional,
        },

        summary {
            docs [
                "A natural language summarization of the object encoded as HTML.",
                "",
                "Multiple language tagged summaries MAY be provided.",
                "",
                "- Range: `xsd:string` | `rdf:langString`",
                "- Functional: false",
            ],
            types [
                XsdString,
                RdfLangString,
            ],
        },

        tag {
            docs [
                "One or more \"tags\" that have been associated with an objects. A tag can be any kind of",
                "`Object`.",
                "",
                "The key difference between attachment and tag is that the former implies association by",
                "inclusion, while the latter implies associated by reference.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                BaseBox,
            ],
        },

        updated {
            docs [
                "The date and time at which the object was updated,",
                "",
                "- Range: `xsd:dateTime`",
                "- Functional: true",
            ],
            types [
                XsdDateTime,
            ],
            functional,
        },

        url {
            docs [
                "Identifies one or more links to representations of the object.",
                "",
                "- Range: `xsd:anyUri` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                LinkBox,
            ],
        },

        to {
            docs [
                "Identifies an entity considered to be part of the public primary audience of an `Object`.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                BaseBox,
            ],
        },

        bto {
            docs [
                "Identifies an `Object` that is part of the private primary audience of this `Object`.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                BaseBox,
            ],
        },

        cc {
            docs [
                "Identifies an `Object` that is part of the public secondary audience of this `Object`.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                BaseBox,
            ],
        },

        bcc {
            docs [
                "Identifies one or more `Objects` that are part of the private secondary audience of this",
                "`Object`.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                BaseBox,
            ],
        },

        media_type {
            docs [
                "When used on an `Object`, identifies the MIME media type of the value of the content",
                "property.",
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

        duration {
            docs [
                "When the object describes a time-bound resource, such as an audio or video, a meeting, etc,",
                "the duration property indicates the object's approximate duration.",
                "",
                "The value MUST be expressed as an xsd:duration as defined by",
                "[[xmlschema11-2](https://www.w3.org/TR/xmlschema11-2/)], section",
                "3.3.6 (e.g. a period of 5 seconds is represented as \"PT5S\").",
                "",
                "- Range: `xsd:duration`",
                "- Functional: true",
            ],
            types [
                XsdDuration,
            ],
            functional,
        },
    }
}

properties! {
    Place {
        docs [ "Define all the properties of the Location type as described by the Activity Streams vocabulary." ],
        accuracy {
            docs [
                "Indicates the accuracy of position coordinates on a `Place` objects.",
                "",
                "Expressed in properties of percentage. e.g. \"94.0\"means \"94.0% accurate\".",
                "",
                "- Range: `xsd:float` [>= 0.0f, <= 100.0f]",
                "- Functional: true",
            ],
            types [
                XsdFloat,
            ],
            functional,
        },

        altitude {
            docs [
                "Indicates the altitude of a place. The measurement units is indicated using the units",
                "property.",
                "",
                "If units is not specified, the default is assumed to be \"m\" indicating meters.",
                "",
                "- Range: `xsd:float`",
                "- Functional: true",
            ],
            types [
                XsdFloat,
            ],
            functional,
        },

        latitude {
            docs [
                "The latitude of a place.",
                "",
                "- Range: `xsd:float`",
                "- Functional: true",
            ],
            types [
                XsdFloat,
            ],
            functional,
        },

        longitude {
            docs [
                "The longitude of a place.",
                "",
                "- Range: `xsd:float`",
                "- Functional: true",
            ],
            types [
                XsdFloat,
            ],
            functional,
        },

        radius {
            docs [
                "The radius from the given latitude and longitude for a Place.",
                "",
                "The units is expressed by the units property. If units is not specified, the default is",
                "assumed to be \"m\" indicating meters.",
                "",
                "- Range: `xsd:float`",
                "- Functional: true",
            ],
            types [
                XsdFloat,
            ],
            functional,
        },

        units {
            docs [
                "Specifies the measurement units for the radius and altitude properties on a `Place` object.",
                "",
                "If not specified, the default is assumed to be \"m\" for meters.",
                "",
                "- Range: `\"cm\"` | `\"feet\"` | `\"inches\"` | `\"km\"` | `\"m\"` | `xsd:anyUri` | `xsd:anyUri`",
                "- Functional: true",
            ],
            types [
                Length,
                XsdAnyUri,
            ],
            functional,
        },
    }
}

properties! {
    Profile {
        docs [ "Define all the properties of the Profile type as described by the Activity Streams vocabulary." ],
        describes {
            docs [
                "On a `Profile` object, the describes property identifies the object described by the",
                "`Profile`.",
                "",
                "- Range: `Object`",
                "- Functional: true",
            ],
            types [
                XsdAnyUri,
                ObjectBox,
            ],
            functional,
        },
    }
}

properties! {
    Relationship {
        docs [
            "Define all the properties of the Relationship type as described by the Activity Streams",
            "vocabulary.",
        ],
        subject {
            docs [
                "On a `Relationship` object, the subject property identifies one of the connected",
                "individuals.",
                "",
                "For instance, for a `Relationship` object describing \"John is related to Sally\", subject",
                "would refer to John.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: true",
            ],
            types [
                XsdAnyUri,
                BaseBox,
            ],
            functional,
        },

        object {
            docs [
                "When used within a `Relationship` describes the entity to which the subject is related.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                BaseBox,
            ],
        },

        relationship {
            docs [
                "On a `Relationship` object, the relationship property identifies the kind of relationship",
                "that exists between subject and object.",
                "",
                "- Range: `Object`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                ObjectBox,
            ],
        },
    }
}

properties! {
    Tombstone {
        docs [ "Define all the properties of the Tombstone type as described by the Activity Streams vocabulary." ],
        former_type {
            docs [
                "On a `Tombstone` object, the formerType property identifies the type of the object that was",
                "deleted.",
                "",
                "- Range: `Object`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                ObjectBox,
            ],
        },

        deleted {
            docs [
                "On a `Tombstone` object, the deleted property is a timestamp for when the object was",
                "deleted.",
                "",
                "- Range: `xsd:dateTime`",
                "- Functional: true",
            ],
            types [
                XsdDateTime,
            ],
            functional,
        },
    }
}

properties! {
    ApObject {
        docs [
            "Define activitypub properties for the Object type as described by the Activity Pub vocabulary.",
        ],

        shares {
            docs [
                "This is a list of all Announce activities with this object as the object property, added as",
                "a side effect.",
                "",
                "The shares collection MUST be either an OrderedCollection or a Collection and MAY be",
                "filtered on privileges of an authenticated user or as appropriate when no authentication is",
                "given.",
                "",
                "- Range: `anyUri`",
                "- Functional: true",
            ],
            types [ XsdAnyUri ],
            functional,
        },

        likes {
            docs [
                "This is a list of all Like activities with this object as the object property, added as a",
                "side effect.",
                "",
                "The likes collection MUST be either an OrderedCollection or a Collection and MAY be",
                "filtered on privileges of an authenticated user or as appropriate when no authentication is",
                "given.",
                "",
                "- Range: `anyUri`",
                "- Functional: true",
            ],
            types [ XsdAnyUri ],
            functional,
        },

        source {
            docs [
                "The source property is intended to convey some sort of source from which the content markup",
                "was derived, as a form of provenance, or to support future editing by clients.",
                "",
                "In general, clients do the conversion from source to content, not the other way around.",
                "",
                "The value of source is itself an object which uses its own content and mediaType fields to",
                "supply source information.",
                "",
                "- Range: `Object`",
                "- Functional: true",
            ],
            types [
                XsdAnyUri,
                ObjectBox,
            ],
            functional,
        },

        upload_media {
            docs [
                "Servers MAY support uploading document types to be referenced in activites, such as images,",
                "video or other binary data, but the precise mechanism is out of scope for this version of",
                "`ActivityPub`.",
                "",
                "The Social Web Community Group is refining the protocol in the",
                "[`ActivityPub` Media Upload report](https://www.w3.org/wiki/SocialCG/ActivityPub/MediaUpload).",
                "",
                "- Range: `anyUri`",
                "- Functional: false",
            ],
            types [ XsdAnyUri ],
        },
    }
}
