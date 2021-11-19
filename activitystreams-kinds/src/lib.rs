//! # activitystreams-kinds
//!
//! Enums representing typed versions of activitypub 'type' fields.

use url::Url;

/// Returns the `https://www.w3.org/ns/activitystreams` Url
pub fn context() -> Url {
    "https://www.w3.org/ns/activitystreams".parse().unwrap()
}

/// Returns the `https://www.w3.org/ns/activitystreams#Public` Url
pub fn public() -> Url {
    "https://www.w3.org/ns/activitystreams#Public"
        .parse()
        .unwrap()
}

/// Returns the `https://w3id.org/security/v1` Url
pub fn security() -> Url {
    "https://w3id.org/security/v1".parse().unwrap()
}

/// Generate an enum implementing serde's Serialize and Deserialize with a single variant
///
/// This is useful for describing constants
///
/// ```rust
/// # fn main() -> Result<(), anyhow::Error> {
/// use activitystreams_kinds::kind;
///
/// kind!(CustomType, Custom);
///
/// #[derive(serde::Deserialize)]
/// struct MyStruct {
///     #[serde(rename = "type")]
///     kind: CustomType,
/// }
///
/// let s: MyStruct = serde_json::from_str(r#"{"type":"Custom"}"#)?;
///
/// assert_eq!(s.kind, CustomType::Custom);
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! kind {
    ($x:ident, $y:ident) => {
        #[derive(
            Clone,
            Debug,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            serde::Deserialize,
            serde::Serialize,
        )]
        /// A type stand-in for the constant $y, deriving serde traits
        pub enum $x {
            $y,
        }

        impl std::fmt::Display for $x {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, stringify!($y))
            }
        }

        impl Default for $x {
            fn default() -> Self {
                $x::$y
            }
        }
    };
}

pub mod activity {
    //! Kinds of activities defined by the spec
    //!
    //! These types exist only to be statically-typed versions of the associated string. e.g.
    //! `CreateType` -> `"Create"`

    use crate::kind;

    kind!(AcceptType, Accept);
    kind!(AddType, Add);
    kind!(AnnounceType, Announce);
    kind!(ArriveType, Arrive);
    kind!(BlockType, Block);
    kind!(CreateType, Create);
    kind!(DeleteType, Delete);
    kind!(DislikeType, Dislike);
    kind!(FlagType, Flag);
    kind!(FollowType, Follow);
    kind!(IgnoreType, Ignore);
    kind!(InviteType, Invite);
    kind!(JoinType, Join);
    kind!(LeaveType, Leave);
    kind!(LikeType, Like);
    kind!(ListenType, Listen);
    kind!(MoveType, Move);
    kind!(OfferType, Offer);
    kind!(QuestionType, Question);
    kind!(ReadType, Read);
    kind!(RejectType, Reject);
    kind!(RemoveType, Remove);
    kind!(TentativeAcceptType, TentativeAccept);
    kind!(TentativeRejectType, TentativeReject);
    kind!(TravelType, Travel);
    kind!(UndoType, Undo);
    kind!(UpdateType, Update);
    kind!(ViewType, View);
}

pub mod actor {
    //! Kinds of actors defined by the spec
    //!
    //! These types exist only to be statically-typed versions of the associated string. e.g.
    //! `PersonType` -> `"Person"`

    use crate::kind;

    kind!(ApplicationType, Application);
    kind!(GroupType, Group);
    kind!(OrganizationType, Organization);
    kind!(PersonType, Person);
    kind!(ServiceType, Service);
}

pub mod collection {
    //! Kinds of collections defined by the spec
    //!
    //! These types exist only to be statically-typed versions of the associated string. e.g.
    //! `CollectionType` -> `"Collection"`

    use crate::kind;

    kind!(CollectionType, Collection);
    kind!(OrderedCollectionType, OrderedCollection);
    kind!(CollectionPageType, CollectionPage);
    kind!(OrderedCollectionPageType, OrderedCollectionPage);
}

pub mod link {
    //! Kinds of links defined by the spec
    //!
    //! These types exist only to be statically-typed versions of the associated string. e.g.
    //! `MentionType` -> `"Mention"`

    use crate::kind;

    kind!(MentionType, Mention);
}

pub mod object {
    //! Kinds of objects defined by the spec
    //!
    //! These types exist only to be statically-typed versions of the associated string. e.g.
    //! `PlaceType` -> `"Place"`

    use crate::kind;

    kind!(ArticleType, Article);
    kind!(AudioType, Audio);
    kind!(DocumentType, Document);
    kind!(EventType, Event);
    kind!(ImageType, Image);
    kind!(NoteType, Note);
    kind!(PageType, Page);
    kind!(PlaceType, Place);
    kind!(ProfileType, Profile);
    kind!(RelationshipType, Relationship);
    kind!(TombstoneType, Tombstone);
    kind!(VideoType, Video);
}

#[cfg(test)]
mod tests {
    use super::kind;

    #[test]
    fn to_string_works() {
        kind!(MyType, My);

        assert_eq!(MyType::My.to_string(), "My")
    }
}
