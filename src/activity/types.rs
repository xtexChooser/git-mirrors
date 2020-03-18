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
    activity::{
        kind::*, properties::*, Activity, ActivityBox, IntransitiveActivity,
        IntransitiveActivityBox,
    },
    ext::Ext,
    object::{
        properties::{ApObjectProperties, ObjectProperties},
        Object, ObjectBox,
    },
    Base, Extensible, PropRefs,
};

/// Indicates that the actor accepts the object.
///
/// The target property can be used in certain circumstances to indicate the context into which the
/// object has been accepted.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct Accept {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: AcceptType,

    /// Adds all valid accept properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub accept_props: AcceptProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// Indicates that the actor has added the object to the target.
///
/// If the target property is not explicitly specified, the target would need to be determined
/// implicitly by context. The origin can be used to identify the context from which the object
/// originated.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct Add {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: AddType,

    /// Adds all valid add properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub add_props: AddProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// Indicates that the actor has moved object from origin to target.
///
/// If the origin or target are not specified, either can be determined by context.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct AMove {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: MoveType,

    /// Adds all valid move properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub move_props: MoveProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// Indicates that the actor is calling the target's attention the object.
///
/// The origin typically has no defined meaning.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct Announce {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: AnnounceType,

    /// Adds all valid announce properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub announce_props: AnnounceProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// An IntransitiveActivity that indicates that the actor has arrived at the location.
///
/// The origin can be used to identify the context from which the actor originated. The target
/// typically has no defined meaning.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
#[prop_refs(IntransitiveActivity)]
pub struct Arrive {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: ArriveType,

    /// Adds all valid arrive properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub arrive_props: ArriveProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// Indicates that the actor is blocking the object.
///
/// Blocking is a stronger form of Ignore. The typical use is to support social systems that allow
/// one user to block activities or content of other users. The target and origin typically have no
/// defined meaning.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct Block {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: BlockType,

    /// Adds all valid block properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub block_props: BlockProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// Indicates that the actor has created the object.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct Create {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: CreateType,

    /// Adds all valid create properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub create_props: CreateProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// Indicates that the actor has deleted the object.
///
/// If specified, the origin indicates the context from which the object was deleted.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct Delete {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: DeleteType,

    /// Adds all valid delete properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub delete_props: DeleteProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// Indicates that the actor dislikes the object.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct Dislike {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: DislikeType,

    /// Adds all valid dislike properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub dislike_props: DislikeProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// Indicates that the actor is "flagging" the object.
///
/// Flagging is defined in the sense common to many social platforms as reporting content as being
/// inappropriate for any number of reasons.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct Flag {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: FlagType,

    /// Adds all valid flag properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub flag_props: FlagProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// Indicates that the actor is "following" the object.
///
/// Following is defined in the sense typically used within Social systems in which the actor is
/// interested in any activity performed by or on the object. The target and origin typically have
/// no defined meaning.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct Follow {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: FollowType,

    /// Adds all valid follow properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub follow_props: FollowProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// Indicates that the actor is ignoring the object.
///
/// The target and origin typically have no defined meaning.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct Ignore {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: IgnoreType,

    /// Adds all valid ignore properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub ignore_props: IgnoreProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// A specialization of Offer in which the actor is extending an invitation for the object to the
/// target.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct Invite {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: InviteType,

    /// Adds all valid invite properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub invite_props: InviteProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// Indicates that the actor has joined the object.
///
/// The target and origin typically have no defined meaning
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct Join {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: JoinType,

    /// Adds all valid join properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub join_props: JoinProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// Indicates that the actor has left the object.
///
/// The target and origin typically have no meaning.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct Leave {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: LeaveType,

    /// Adds all valid leave properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub leave_props: LeaveProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// Indicates that the actor likes, recommends or endorses the object.
///
/// The target and origin typically have no defined meaning.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct Like {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: LikeType,

    /// Adds all valid like properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub like_props: LikeProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// Indicates that the actor has listened to the object.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct Listen {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    kind: ListenType,

    /// Adds all valid listen properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub listen_props: ListenProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// Indicates that the actor is offering the object.
///
/// If specified, the target indicates the entity to which the object is being offered.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct Offer {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: OfferType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub offer_props: OfferProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// Represents a question being asked.
///
/// Question objects are an extension of IntransitiveActivity. That is, the Question object is an
/// Activity, but the direct object is the question itself and therefore it would not contain an
/// object property.
///
/// Either of the anyOf and oneOf properties MAY be used to express possible answers, but a
/// Question object MUST NOT have both properties.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
#[prop_refs(IntransitiveActivity)]
pub struct Question {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: QuestionType,

    /// Adds all valid question properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub question_props: QuestionProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// Indicates that the actor has read the object.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct Read {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: ReadType,

    /// Adds all valid read properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub read_props: ReadProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// Indicates that the actor is rejecting the object.
///
/// The target and origin typically have no defined meaning.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct Reject {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: RejectType,

    /// Adds all valid reject properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub reject_props: RejectProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// Indicates that the actor is removing the object.
///
/// If specified, the origin indicates the context from which the object is being removed.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct Remove {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: RemoveType,

    /// Adds all valid remove properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub remove_props: RemoveProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// A specialization of Accept indicating that the acceptance is tentative.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct TentativeAccept {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: TentativeAcceptType,

    /// Adds all valid tentative_accept properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub tentative_accept_props: TentativeAcceptProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// A specialization of Reject in which the rejection is considered tentative.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct TentativeReject {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: TentativeRejectType,

    /// Adds all valid tentative_reject properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub tentative_reject_props: TentativeRejectProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// Indicates that the actor is traveling to target from origin.
///
/// Travel is an IntransitiveObject whose actor specifies the direct object. If the target or
/// origin are not specified, either can be determined by context.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
#[prop_refs(IntransitiveActivity)]
pub struct Travel {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: TravelType,

    /// Adds all valid travel properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub travel_props: TravelProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// Indicates that the actor is undoing the object.
///
/// In most cases, the object will be an Activity describing some previously performed action (for
/// instance, a person may have previously "liked" an article but, for whatever reason, might
/// choose to undo that like at some later point in time).
///
/// The target and origin typically have no defined meaning.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct Undo {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: UndoType,

    /// Adds all valid undo properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub undo_props: UndoProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// Indicates that the actor has updated the object.
///
/// Note, however, that this vocabulary does not define a mechanism for describing the actual set
/// of modifications made to object.
///
/// The target and origin typically have no defined meaning.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct Update {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: UpdateType,

    /// Adds all valid update properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub update_props: UpdateProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}

/// Indicates that the actor has viewed the object.
#[derive(Clone, Debug, Default, Extensible, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
#[prop_refs(Activity)]
#[extension(ApObjectProperties)]
pub struct View {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    pub kind: ViewType,

    /// Adds all valid view properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub view_props: ViewProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub activity_props: ActivityProperties,
}
