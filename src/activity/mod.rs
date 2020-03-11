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

#[cfg(feature = "types")]
mod accept;
#[cfg(feature = "types")]
mod add;
#[cfg(feature = "types")]
mod amove;
#[cfg(feature = "types")]
mod announce;
#[cfg(feature = "types")]
mod arrive;
#[cfg(feature = "types")]
mod block;
#[cfg(feature = "types")]
mod create;
#[cfg(feature = "types")]
mod delete;
#[cfg(feature = "types")]
mod dislike;
#[cfg(feature = "types")]
mod flag;
#[cfg(feature = "types")]
mod follow;
#[cfg(feature = "types")]
mod ignore;
#[cfg(feature = "types")]
mod invite;
#[cfg(feature = "types")]
mod join;
#[cfg(feature = "types")]
mod leave;
#[cfg(feature = "types")]
mod like;
#[cfg(feature = "types")]
mod listen;
#[cfg(feature = "types")]
mod offer;
#[cfg(feature = "types")]
pub mod properties;
#[cfg(feature = "types")]
mod question;
#[cfg(feature = "types")]
mod read;
#[cfg(feature = "types")]
mod reject;
#[cfg(feature = "types")]
mod remove;
#[cfg(feature = "types")]
mod tentative_accept;
#[cfg(feature = "types")]
mod tentative_reject;
#[cfg(feature = "types")]
mod travel;
#[cfg(feature = "types")]
mod undo;
#[cfg(feature = "types")]
mod update;
#[cfg(feature = "types")]
mod view;

#[cfg(feature = "types")]
pub use self::{
    accept::Accept, add::Add, amove::AMove, announce::Announce, arrive::Arrive, block::Block,
    create::Create, delete::Delete, dislike::Dislike, flag::Flag, follow::Follow, ignore::Ignore,
    invite::Invite, join::Join, leave::Leave, like::Like, listen::Listen, offer::Offer,
    question::Question, read::Read, reject::Reject, remove::Remove,
    tentative_accept::TentativeAccept, tentative_reject::TentativeReject, travel::Travel,
    undo::Undo, update::Update, view::View,
};

#[cfg(feature = "kinds")]
pub mod kind;

use crate::object::Object;

/// An Activity is a subtype of `Object` that describes some form of action that may happen, is
/// currently happening, or has already happened.
///
/// The `Activity` type itself serves as an abstract base type for all types of activities. It is
/// important to note that the `Activity` type itself does not carry any specific semantics about
/// the kind of action being taken.
pub trait Activity: Object {}

/// Instances of `IntransitiveActivity` are a subtype of `Activity` representing intransitive
/// actions.
///
/// The `object` property is therefore inappropriate for these activities.
pub trait IntransitiveActivity: Activity {}
