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

//! Namespace for Unit Structs that serialize to strings

use crate::UnitString;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Accept)]
pub struct AcceptType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Add)]
pub struct AddType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Move)]
pub struct MoveType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Announce)]
pub struct AnnounceType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Arrive)]
pub struct ArriveType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Block)]
pub struct BlockType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Create)]
pub struct CreateType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Delete)]
pub struct DeleteType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Dislike)]
pub struct DislikeType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Flag)]
pub struct FlagType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Follow)]
pub struct FollowType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Ignore)]
pub struct IgnoreType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Invite)]
pub struct InviteType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Join)]
pub struct JoinType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Leave)]
pub struct LeaveType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Like)]
pub struct LikeType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Listen)]
pub struct ListenType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Offer)]
pub struct OfferType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Question)]
pub struct QuestionType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Real)]
pub struct ReadType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Reject)]
pub struct RejectType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Remove)]
pub struct RemoveType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(TentativeAccept)]
pub struct TentativeAcceptType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(TentativeReject)]
pub struct TentativeRejectType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Travel)]
pub struct TravelType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Undo)]
pub struct UndoType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Update)]
pub struct UpdateType;

#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(View)]
pub struct ViewType;
