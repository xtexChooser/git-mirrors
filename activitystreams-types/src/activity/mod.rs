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

mod accept;
mod add;
mod amove;
mod announce;
mod arrive;
mod block;
mod create;
mod delete;
mod dislike;
mod flag;
mod follow;
mod ignore;
mod invite;
mod join;
pub mod kind;
mod leave;
mod like;
mod listen;
mod offer;
pub mod properties;
mod question;
mod read;
mod reject;
mod remove;
mod tentative_accept;
mod tentative_reject;
mod travel;
mod undo;
mod update;
mod view;

pub use self::{
    accept::*, add::*, amove::*, announce::*, arrive::*, block::*, create::*, delete::*,
    dislike::*, flag::*, follow::*, ignore::*, invite::*, join::*, leave::*, like::*, listen::*,
    offer::*, question::*, read::*, reject::*, remove::*, tentative_accept::*, tentative_reject::*,
    travel::*, undo::*, update::*, view::*,
};

use activitystreams_traits::Activity;

use self::properties::ActivityProperties;

/// The Activity Extension Trait
///
/// This trait provides generic access to an activity's properties
pub trait ActivityExt: Activity {
    fn props(&self) -> &ActivityProperties;
    fn props_mut(&mut self) -> &mut ActivityProperties;
}
