/* This file is part of ActivityStreams Types.
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

//! Namespace for properties of standard Activity types
//!
//! To use these properties in your own types, you can flatten them into your struct with serde:
//!
//! ```rust
//! use activitystreams_derive::PropRefs;
//! use activitystreams_traits::{Activity, Object};
//! use activitystreams_types::{
//!     activity::{
//!         properties::ActivityProperties,
//!         ActivityExt,
//!     },
//!     object::{
//!         properties::ObjectProperties,
//!         ObjectExt,
//!     },
//! };
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Clone, Debug, Serialize, Deserialize, PropRefs)]
//! #[serde(rename_all = "camelCase")]
//! pub struct MyActivity {
//!     #[serde(rename = "type")]
//!     #[serde(alias = "objectType")]
//!     #[serde(alias = "verb")]
//!     pub kind: String,
//!
//!     /// Define a require property for the MyActivity type
//!     pub my_property: String,
//!
//!     #[serde(flatten)]
//!     #[activitystreams(Object)]
//!     pub object_properties: ObjectProperties,
//!
//!     #[serde(flatten)]
//!     #[activitystreams(Activity)]
//!     pub activity_properties: ActivityProperties,
//! }
//! #
//! # fn main() {}
//! ```

use crate::{link::LinkBox, object::ObjectBox, primitives::*};
use activitystreams_derive::properties;

properties! {
    Activity {
        docs [
            "Activity objects are specializations of the base Object type that provide information about",
            "actions that have either already occurred, are in the process of occurring, or may occur in the",
            "future.",
        ],

        result {
            docs [
                "Describes the result of the activity.",
                "",
                "For instance, if a particular action results in the creation of a new resource, the result",
                "property can be used to describe that new resource.",
                "",
                "- Range: `Object` | `Link`",
                "- Funcitonal: false",
            ],
            types [
                XsdAnyUri,
                ObjectBox,
                LinkBox,
            ],
        },

        instrument {
            docs [
                "Identifies one or more objects used (or to be used) in the completion of an `Activity`.",
                "",
                "- Range: `Object` | `Link`",
                "- Funcitonal: false",
            ],
            types [
                XsdAnyUri,
                ObjectBox,
                LinkBox,
            ],
        },
    }
}

properties! {
    ActorOptOriginAndTarget {
        docs [ "Struct with `actor` and optional `origin` and `target` properties" ],

        actor {
            docs [
                "Describes one or more entities that either performed or are expected to perform the",
                "activity.",
                "",
                "Any single activity can have multiple actors. The actor MAY be specified using an indirect",
                "Link.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                ObjectBox,
                LinkBox,
            ],
            required
        },

        origin {
            docs [
                "Describes an indirect object of the activity from which the activity is directed.",
                "",
                "The precise meaning of the origin is the object of the English preposition \"from\". For",
                "instance, in the activity \"John moved an item to List B from List A\", the origin of the",
                "activity is \"List A\".",
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

        target {
            docs [
                "Describes the indirect object, or target, of the activity.",
                "",
                "The precise meaning of the target is largely dependent on the type of action being",
                "described but will often be the object of the English preposition \"to\". For instance, in",
                "the activity \"John added a movie to his wishlist\", the target of the activity is John's",
                "wishlist. An activity can have more than one target",
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

properties! {
    ActorAndObject {
        docs [ "Struct with `actor` and `object` properties" ],

        actor {
            docs [
                "Describes one or more entities that either performed or are expected to perform the",
                "activity.",
                "",
                "Any single activity can have multiple actors. The actor MAY be specified using an indirect",
                "Link.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                ObjectBox,
                LinkBox,
            ],
            required,
        },

        object {
            docs [
                "When used within an Activity, describes the direct object of the activity.",
                "",
                "For instance, in the activity \"John added a movie to his wishlist\", the object of the",
                "activity is the movie added.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                ObjectBox,
                LinkBox,
            ],
            required,
        },
    }
}

properties! {
    ActorObjectAndTarget {
        docs [ "Struct with `actor`, `object`, and `target` properties" ],

        actor {
            docs [
                "Describes one or more entities that either performed or are expected to perform the",
                "activity.",
                "",
                "Any single activity can have multiple actors. The actor MAY be specified using an indirect",
                "Link.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                ObjectBox,
                LinkBox,
            ],
            required,
        },

        object {
            docs [
                "When used within an Activity, describes the direct object of the activity.",
                "",
                "For instance, in the activity \"John added a movie to his wishlist\", the object of the",
                "activity is the movie added.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                ObjectBox,
                LinkBox,
            ],
            required,
        },

        target {
            docs [
                "Describes the indirect object, or target, of the activity.",
                "",
                "The precise meaning of the target is largely dependent on the type of action being",
                "described but will often be the object of the English preposition \"to\". For instance, in",
                "the activity \"John added a movie to his wishlist\", the target of the activity is John's",
                "wishlist. An activity can have more than one target",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                ObjectBox,
                LinkBox,
            ],
            required,
        },
    }
}

properties! {
    ActorAndObjectOptTarget {
        docs [ "Struct with `actor`, `object`, and optional `target` properties" ],

        actor {
            docs [
                "Describes one or more entities that either performed or are expected to perform the",
                "activity.",
                "",
                "Any single activity can have multiple actors. The actor MAY be specified using an indirect",
                "Link.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                ObjectBox,
                LinkBox,
            ],
            required,
        },

        object {
            docs [
                "When used within an Activity, describes the direct object of the activity.",
                "",
                "For instance, in the activity \"John added a movie to his wishlist\", the object of the",
                "activity is the movie added.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                ObjectBox,
                LinkBox,
            ],
            required,
        },

        target {
            docs [
                "Describes the indirect object, or target, of the activity.",
                "",
                "The precise meaning of the target is largely dependent on the type of action being",
                "described but will often be the object of the English preposition \"to\". For instance, in",
                "the activity \"John added a movie to his wishlist\", the target of the activity is John's",
                "wishlist. An activity can have more than one target",
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

properties! {
    ActorAndObjectOptOrigin {
        docs [ "Struct with `actor`, `object`, and optional `origin` properties" ],

        actor {
            docs [
                "Describes one or more entities that either performed or are expected to perform the",
                "activity.",
                "",
                "Any single activity can have multiple actors. The actor MAY be specified using an indirect",
                "Link.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                ObjectBox,
                LinkBox,
            ],
            required,
        },

        object {
            docs [
                "When used within an Activity, describes the direct object of the activity.",
                "",
                "For instance, in the activity \"John added a movie to his wishlist\", the object of the",
                "activity is the movie added.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                ObjectBox,
                LinkBox,
            ],
            required,
        },

        origin {
            docs [
                "Describes an indirect object of the activity from which the activity is directed.",
                "",
                "The precise meaning of the origin is the object of the English preposition \"from\". For",
                "instance, in the activity \"John moved an item to List B from List A\", the origin of the",
                "activity is \"List A\".",
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

properties! {
    ActorAndObjectOptOthers {
        docs [ "Struct with `actor`, `object`, and optional `origin` and `target` properties" ],

        actor {
            docs [
                "Describes one or more entities that either performed or are expected to perform the",
                "activity.",
                "",
                "Any single activity can have multiple actors. The actor MAY be specified using an indirect",
                "Link.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                ObjectBox,
                LinkBox,
            ],
            required,
        },

        object {
            docs [
                "When used within an Activity, describes the direct object of the activity.",
                "",
                "For instance, in the activity \"John added a movie to his wishlist\", the object of the",
                "activity is the movie added.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                ObjectBox,
                LinkBox,
            ],
            required,
        },

        origin {
            docs [
                "Describes an indirect object of the activity from which the activity is directed.",
                "",
                "The precise meaning of the origin is the object of the English preposition \"from\". For",
                "instance, in the activity \"John moved an item to List B from List A\", the origin of the",
                "activity is \"List A\".",
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

        target {
            docs [
                "Describes the indirect object, or target, of the activity.",
                "",
                "The precise meaning of the target is largely dependent on the type of action being",
                "described but will often be the object of the English preposition \"to\". For instance, in",
                "the activity \"John added a movie to his wishlist\", the target of the activity is John's",
                "wishlist. An activity can have more than one target",
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

properties! {
    ActorAndOrigin {
        docs [ "Struct with `actor` and `origin` properties" ],

        actor {
            docs [
                "Describes one or more entities that either performed or are expected to perform the",
                "activity.",
                "",
                "Any single activity can have multiple actors. The actor MAY be specified using an indirect",
                "Link.",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdString,
                ObjectBox,
                LinkBox,
            ],
            required,
        },

        origin {
            docs [
                "Describes an indirect object of the activity from which the activity is directed.",
                "",
                "The precise meaning of the origin is the object of the English preposition \"from\". For",
                "instance, in the activity \"John moved an item to List B from List A\", the origin of the",
                "activity is \"List A\".",
                "",
                "- Range: `Object` | `Link`",
                "- Functional: false",
            ],
            types [
                XsdAnyUri,
                ObjectBox,
                LinkBox,
            ],
            required,
        },
    }
}

/// Properties for the Accept activity
pub type AcceptProperties = ActorAndObjectProperties;

/// Properties for the Add activity
pub type AddProperties = ActorAndObjectProperties;

/// Properties for the Move activity
pub type MoveProperties = ActorAndObjectOptOthersProperties;

/// Properties for the Announce activity
pub type AnnounceProperties = ActorAndObjectOptTargetProperties;

/// Properties for the Arrive activity
pub type ArriveProperties = ActorAndOriginProperties;

/// Properties for the Block activity
pub type BlockProperties = ActorAndObjectProperties;

/// Properties for the Create activity
pub type CreateProperties = ActorAndObjectProperties;

/// Properties for the Delete activity
pub type DeleteProperties = ActorAndObjectOptOriginProperties;

/// Properties for the Dislike activity
pub type DislikeProperties = ActorAndObjectProperties;

/// Properties for the Flag activity
pub type FlagProperties = ActorAndObjectProperties;

/// Properties for the Follow activity
pub type FollowProperties = ActorAndObjectProperties;

/// Properties for the Ignore activity
pub type IgnoreProperties = ActorAndObjectProperties;

/// Properties for the Invite activity
pub type InviteProperties = ActorObjectAndTargetProperties;

/// Properties for the Join activity
pub type JoinProperties = ActorAndObjectProperties;

/// Properties for the Leave activity
pub type LeaveProperties = ActorAndObjectProperties;

/// Properties for the Like activity
pub type LikeProperties = ActorAndObjectProperties;

/// Properties for the Listen activity
pub type ListenProperties = ActorAndObjectProperties;

/// Properties for the Offer activity
pub type OfferProperties = ActorAndObjectOptTargetProperties;

properties! {
    Question {
        docs [ "Properties for the Question activity" ],

        one_of {
            docs [
                "Identifies an exclusive option for a Question.",
                "",
                "Use of `one_of` implies that the Question can have only a single answer. To indicate that a",
                "`Question` can have multiple answers, use `any_of`.",
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

        any_of {
            docs [
                "Identifies an inclusive option for a Question.",
                "",
                "Use of `any_of` implies that the Question can have multiple answers. To indicate that a",
                "`Question` can have only one answer, use `one_of`.",
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

/// Properties for the Read activity
pub type ReadProperties = ActorAndObjectProperties;

/// Properties for the Reject activity
pub type RejectProperties = ActorAndObjectProperties;

/// Properties for the Remove activity
pub type RemoveProperties = ActorAndObjectOptOthersProperties;

/// Properties for the TentativeAccept activity
pub type TentativeAcceptProperties = ActorAndObjectProperties;

/// Properties for the TentativeReject activity
pub type TentativeRejectProperties = ActorAndObjectProperties;

/// Properties for the Travel activity
pub type TravelProperties = ActorOptOriginAndTargetProperties;

/// Properties for the Undo activity
pub type UndoProperties = ActorAndObjectProperties;

/// Properties for the Update activity
pub type UpdateProperties = ActorAndObjectProperties;

/// Properties for the View activity
pub type ViewProperties = ActorAndObjectProperties;
