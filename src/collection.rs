//! Types and traits for dealing with Collection attributes
//!
//! ```rust
//! # fn main() -> Result<(), anyhow::Error> {
//! use activitystreams::{
//!     collection::OrderedCollection,
//!     context,
//!     prelude::*,
//!     uri,
//! };
//!
//! let mut collection = OrderedCollection::new(vec![
//!     uri!("https://example.com/notes/1234").into(),
//! ]);
//!
//! collection
//!     .set_total_items(1u64)
//!     .set_current(uri!("https://example.com/notes/1234"))
//!     .set_first(uri!("https://example.com/notes/1234"))
//!     .set_last(uri!("https://example.com/notes/1234"))
//!     .set_id(uri!("https://example.com/collections/1234"))
//!     .set_context(context());
//! # Ok(())
//! # }
//! ```
use crate::{
    base::{AnyBase, AsBase, Base, Extends},
    markers,
    object::{ApObject, AsObject, Object},
    primitives::OneOrMany,
    unparsed::{Unparsed, UnparsedMut, UnparsedMutExt},
};
use std::convert::TryFrom;

pub mod kind {
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

use self::kind::*;

/// Implementation trait for deriving Collection methods for a type
///
/// Any type implementing AsCollection will automatically gain methods provided by CollectionExt
pub trait AsCollection<Kind>: markers::Collection {
    fn collection_ref(&self) -> &Collection<Kind>;
    fn collection_mut(&mut self) -> &mut Collection<Kind>;
}

/// Implementation trait for deriving Collection methods for a type
///
/// Any type implementing AsCollectionPage will automatically gain methods provided by CollectionPageExt
pub trait AsCollectionPage<Kind>: markers::CollectionPage {
    fn collection_page_ref(&self) -> &CollectionPage<Kind>;
    fn collection_page_mut(&mut self) -> &mut CollectionPage<Kind>;
}

/// Implementation trait for deriving Collection methods for a type
///
/// Any type implementing AsOrderedCollectionPage will automatically gain methods provided by
/// OrderedCollectionPageExt
pub trait AsOrderedCollectionPage: markers::CollectionPage {
    fn ordered_collection_page_ref(&self) -> &OrderedCollectionPage;
    fn ordered_collection_page_mut(&mut self) -> &mut OrderedCollectionPage;
}

/// Helper methods for interacting with Collection types
///
/// This trait represents methods valid for any ActivityStreams Collection
///
/// Documentation for the fields related to these methods can be found on the
/// `Collection` struct
pub trait CollectionExt<Kind>: AsCollection<Kind> {
    /// Fetch the items for the current activity
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::UnorderedCollection};
    /// # let mut collection = UnorderedCollection::new(vec![context().into()]);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// let items_ref = collection.items();
    /// println!("{:?}", items_ref);
    /// ```
    fn items<'a>(&'a self) -> &'a OneOrMany<AnyBase>
    where
        Kind: 'a,
    {
        &self.collection_ref().items
    }

    /// Set the items for the current activity
    ///
    /// This overwrites the contents of items
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{context, collection::UnorderedCollection, uri};
    /// # let mut collection = UnorderedCollection::new(vec![context().into()]);
    ///
    /// collection.set_items(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_items<T>(&mut self, item: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.collection_mut().items = item.into().into();
        self
    }

    /// Set many itemss for the current activity
    ///
    /// This overwrites the contents of items
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{context, collection::UnorderedCollection, uri};
    /// # let mut collection = UnorderedCollection::new(vec![context().into()]);
    ///
    /// collection.set_many_items(vec![
    ///     uri!("https://example.com/one"),
    ///     uri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_items<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.collection_mut().items = v.into();
        self
    }

    /// Add a items to the current activity
    ///
    /// This does not overwrite the contents of items, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{context, collection::UnorderedCollection, uri};
    /// # let mut collection = UnorderedCollection::new(vec![context().into()]);
    ///
    /// collection
    ///     .add_items(uri!("https://example.com/one"))
    ///     .add_items(uri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_items<T>(&mut self, item: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.collection_mut().items.add(item.into());
        self
    }

    /// Fetch the total_items of the current object
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::UnorderedCollection};
    /// # let mut collection = UnorderedCollection::new(vec![context().into()]);
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(total_items) = collection.total_items() {
    ///     println!("{:?}", total_items);
    /// }
    /// ```
    fn total_items<'a>(&'a self) -> Option<u64>
    where
        Kind: 'a,
    {
        self.collection_ref().total_items
    }

    /// Set the total_items for the current object
    ///
    /// This overwrites the contents of total_items
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::UnorderedCollection};
    /// # let mut collection = UnorderedCollection::new(vec![context().into()]);
    /// use activitystreams::prelude::*;
    ///
    /// collection.set_total_items(5u64);
    /// ```
    fn set_total_items<T>(&mut self, total_items: T) -> &mut Self
    where
        T: Into<u64>,
    {
        self.collection_mut().total_items = Some(total_items.into());
        self
    }

    /// Take the total_items of the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::UnorderedCollection};
    /// # let mut collection = UnorderedCollection::new(vec![context().into()]);
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(total_items) = collection.total_items() {
    ///     println!("{:?}", total_items);
    /// }
    /// ```
    fn take_total_items(&mut self) -> Option<u64> {
        self.collection_mut().total_items.take()
    }

    /// Delete the total_items from the current object
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::UnorderedCollection};
    /// # let mut collection = UnorderedCollection::new(vec![context().into()]);
    /// # collection.set_total_items(5u64);
    /// use activitystreams::prelude::*;
    ///
    /// assert!(collection.total_items().is_some());
    /// collection.delete_total_items();
    /// assert!(collection.total_items().is_none());
    /// ```
    fn delete_total_items(&mut self) -> &mut Self {
        self.collection_mut().total_items = None;
        self
    }

    /// Fetch the current field for the current object
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::UnorderedCollection};
    /// # let mut collection = UnorderedCollection::new(vec![context().into()]);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(current) = collection.current() {
    ///     println!("{:?}", current);
    /// }
    /// ```
    fn current<'a>(&'a self) -> Option<&'a AnyBase>
    where
        Kind: 'a,
    {
        self.collection_ref().current.as_ref()
    }

    /// Set the current field for the current object
    ///
    /// This overwrites the contents of current
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{context, collection::UnorderedCollection, uri};
    /// # let mut collection = UnorderedCollection::new(vec![context().into()]);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// collection.set_current(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_current<T>(&mut self, current: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.collection_mut().current = Some(current.into());
        self
    }

    /// Take the current field from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::UnorderedCollection};
    /// # let mut collection = UnorderedCollection::new(vec![context().into()]);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(current) = collection.take_current() {
    ///     println!("{:?}", current);
    /// }
    /// ```
    fn take_current(&mut self) -> Option<AnyBase> {
        self.collection_mut().current.take()
    }

    /// Delete the current field from the current object
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::UnorderedCollection};
    /// # let mut collection = UnorderedCollection::new(vec![context().into()]);
    /// # collection.set_current(context());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(collection.current().is_some());
    /// collection.delete_current();
    /// assert!(collection.current().is_none());
    /// ```
    fn delete_current(&mut self) -> &mut Self {
        self.collection_mut().current = None;
        self
    }

    /// Fetch the first field for the current object
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::UnorderedCollection};
    /// # let mut collection = UnorderedCollection::new(vec![context().into()]);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(first) = collection.first() {
    ///     println!("{:?}", first);
    /// }
    /// ```
    fn first<'a>(&'a self) -> Option<&'a AnyBase>
    where
        Kind: 'a,
    {
        self.collection_ref().first.as_ref()
    }

    /// Set the first field for the current object
    ///
    /// This overwrites the contents of first
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{context, collection::UnorderedCollection};
    /// # let mut collection = UnorderedCollection::new(vec![context().into()]);
    /// #
    /// use activitystreams::{prelude::*, uri};
    ///
    /// collection.set_first(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_first<T>(&mut self, first: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.collection_mut().first = Some(first.into());
        self
    }

    /// Take the first field from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::UnorderedCollection};
    /// # let mut collection = UnorderedCollection::new(vec![context().into()]);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(first) = collection.take_first() {
    ///     println!("{:?}", first);
    /// }
    /// ```
    fn take_first(&mut self) -> Option<AnyBase> {
        self.collection_mut().first.take()
    }

    /// Delete the first field from the current object
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::UnorderedCollection};
    /// # let mut collection = UnorderedCollection::new(vec![context().into()]);
    /// # collection.set_first(context());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(collection.first().is_some());
    /// collection.delete_first();
    /// assert!(collection.first().is_none());
    /// ```
    fn delete_first(&mut self) -> &mut Self {
        self.collection_mut().first = None;
        self
    }

    /// Fetch the last field for the current object
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::UnorderedCollection};
    /// # let mut collection = UnorderedCollection::new(vec![context().into()]);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(last) = collection.last() {
    ///     println!("{:?}", last);
    /// }
    /// ```
    fn last<'a>(&'a self) -> Option<&'a AnyBase>
    where
        Kind: 'a,
    {
        self.collection_ref().last.as_ref()
    }

    /// Set the last field for the current object
    ///
    /// This overwrites the contents of last
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{context, collection::UnorderedCollection};
    /// # let mut collection = UnorderedCollection::new(vec![context().into()]);
    /// #
    /// use activitystreams::{prelude::*, uri};
    ///
    /// collection.set_last(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_last<T>(&mut self, last: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.collection_mut().last = Some(last.into());
        self
    }

    /// Take the last field from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::UnorderedCollection};
    /// # let mut collection = UnorderedCollection::new(vec![context().into()]);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(last) = collection.take_last() {
    ///     println!("{:?}", last);
    /// }
    /// ```
    fn take_last(&mut self) -> Option<AnyBase> {
        self.collection_mut().last.take()
    }

    /// Delete the last field from the current object
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::UnorderedCollection};
    /// # let mut collection = UnorderedCollection::new(vec![context().into()]);
    /// # collection.set_last(context());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(collection.last().is_some());
    /// collection.delete_last();
    /// assert!(collection.last().is_none());
    /// ```
    fn delete_last(&mut self) -> &mut Self {
        self.collection_mut().last = None;
        self
    }
}

/// Helper methods for interacting with CollectionPage types
///
/// This trait represents methods valid for any ActivityStreams CollectionPage
///
/// Documentation for the fields related to these methods can be found on the
/// `CollectionPage` struct
pub trait CollectionPageExt<Kind>: AsCollectionPage<Kind> {
    /// Fetch the part_of field for the current object
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::UnorderedCollectionPage};
    /// # let mut collection = UnorderedCollectionPage::new(vec![context().into()]);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(part_of) = collection.part_of() {
    ///     println!("{:?}", part_of);
    /// }
    /// ```
    fn part_of<'a>(&'a self) -> Option<&'a AnyBase>
    where
        Kind: 'a,
    {
        self.collection_page_ref().part_of.as_ref()
    }

    /// Set the part_of field for the current object
    ///
    /// This overwrites the contents of part_of
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{context, collection::UnorderedCollectionPage};
    /// # let mut collection = UnorderedCollectionPage::new(vec![context().into()]);
    /// #
    /// use activitystreams::{prelude::*, uri};
    ///
    /// collection.set_part_of(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_part_of<T>(&mut self, part_of: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.collection_page_mut().part_of = Some(part_of.into());
        self
    }

    /// Take the part_of field from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::UnorderedCollectionPage};
    /// # let mut collection = UnorderedCollectionPage::new(vec![context().into()]);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(part_of) = collection.take_part_of() {
    ///     println!("{:?}", part_of);
    /// }
    /// ```
    fn take_part_of(&mut self) -> Option<AnyBase> {
        self.collection_page_mut().part_of.take()
    }

    /// Delete the part_of field from the current object
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::UnorderedCollectionPage};
    /// # let mut collection = UnorderedCollectionPage::new(vec![context().into()]);
    /// # collection.set_part_of(context());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(collection.part_of().is_some());
    /// collection.delete_part_of();
    /// assert!(collection.part_of().is_none());
    /// ```
    fn delete_part_of(&mut self) -> &mut Self {
        self.collection_page_mut().part_of = None;
        self
    }

    /// Fetch the next field for the current object
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::UnorderedCollectionPage};
    /// # let mut collection = UnorderedCollectionPage::new(vec![context().into()]);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(next) = collection.next() {
    ///     println!("{:?}", next);
    /// }
    /// ```
    fn next<'a>(&'a self) -> Option<&'a AnyBase>
    where
        Kind: 'a,
    {
        self.collection_page_ref().next.as_ref()
    }

    /// Set the next field for the current object
    ///
    /// This overwrites the contents of next
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{context, collection::UnorderedCollectionPage};
    /// # let mut collection = UnorderedCollectionPage::new(vec![context().into()]);
    /// #
    /// use activitystreams::{prelude::*, uri};
    ///
    /// collection.set_next(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_next<T>(&mut self, next: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.collection_page_mut().next = Some(next.into());
        self
    }

    /// Take the next field from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::UnorderedCollectionPage};
    /// # let mut collection = UnorderedCollectionPage::new(vec![context().into()]);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(next) = collection.take_next() {
    ///     println!("{:?}", next);
    /// }
    /// ```
    fn take_next(&mut self) -> Option<AnyBase> {
        self.collection_page_mut().next.take()
    }

    /// Delete the next field from the current object
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::UnorderedCollectionPage};
    /// # let mut collection = UnorderedCollectionPage::new(vec![context().into()]);
    /// # collection.set_next(context());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(collection.next().is_some());
    /// collection.delete_next();
    /// assert!(collection.next().is_none());
    /// ```
    fn delete_next(&mut self) -> &mut Self {
        self.collection_page_mut().next = None;
        self
    }

    /// Fetch the prev field for the current object
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::UnorderedCollectionPage};
    /// # let mut collection = UnorderedCollectionPage::new(vec![context().into()]);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(prev) = collection.prev() {
    ///     println!("{:?}", prev);
    /// }
    /// ```
    fn prev<'a>(&'a self) -> Option<&'a AnyBase>
    where
        Kind: 'a,
    {
        self.collection_page_ref().prev.as_ref()
    }

    /// Set the prev field for the current object
    ///
    /// This overwrites the contents of prev
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{context, collection::UnorderedCollectionPage};
    /// # let mut collection = UnorderedCollectionPage::new(vec![context().into()]);
    /// #
    /// use activitystreams::{prelude::*, uri};
    ///
    /// collection.set_prev(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_prev<T>(&mut self, prev: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.collection_page_mut().prev = Some(prev.into());
        self
    }

    /// Take the prev field from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::UnorderedCollectionPage};
    /// # let mut collection = UnorderedCollectionPage::new(vec![context().into()]);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(prev) = collection.take_prev() {
    ///     println!("{:?}", prev);
    /// }
    /// ```
    fn take_prev(&mut self) -> Option<AnyBase> {
        self.collection_page_mut().prev.take()
    }

    /// Delete the prev field from the current object
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::UnorderedCollectionPage};
    /// # let mut collection = UnorderedCollectionPage::new(vec![context().into()]);
    /// # collection.set_prev(context());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(collection.prev().is_some());
    /// collection.delete_prev();
    /// assert!(collection.prev().is_none());
    /// ```
    fn delete_prev(&mut self) -> &mut Self {
        self.collection_page_mut().prev = None;
        self
    }
}

pub trait OrderedCollectionPageExt: AsOrderedCollectionPage {
    /// Fetch the start_index of the current object
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::OrderedCollectionPage};
    /// # let mut collection = OrderedCollectionPage::new(vec![context().into()]);
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(start_index) = collection.start_index() {
    ///     println!("{:?}", start_index);
    /// }
    /// ```
    fn start_index(&self) -> Option<u64> {
        self.ordered_collection_page_ref().start_index
    }

    /// Set the start_index for the current object
    ///
    /// This overwrites the contents of start_index
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::OrderedCollectionPage};
    /// # let mut collection = OrderedCollectionPage::new(vec![context().into()]);
    /// use activitystreams::prelude::*;
    ///
    /// collection.set_start_index(5u64);
    /// ```
    fn set_start_index<T>(&mut self, start_index: T) -> &mut Self
    where
        T: Into<u64>,
    {
        self.ordered_collection_page_mut().start_index = Some(start_index.into());
        self
    }

    /// Take the start_index of the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::OrderedCollectionPage};
    /// # let mut collection = OrderedCollectionPage::new(vec![context().into()]);
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(start_index) = collection.start_index() {
    ///     println!("{:?}", start_index);
    /// }
    /// ```
    fn take_start_index(&mut self) -> Option<u64> {
        self.ordered_collection_page_mut().start_index.take()
    }

    /// Delete the start_index from the current object
    ///
    /// ```rust
    /// # use activitystreams::{context, collection::OrderedCollectionPage};
    /// # let mut collection = OrderedCollectionPage::new(vec![context().into()]);
    /// # collection.set_start_index(5u64);
    /// use activitystreams::prelude::*;
    ///
    /// assert!(collection.start_index().is_some());
    /// collection.delete_start_index();
    /// assert!(collection.start_index().is_none());
    /// ```
    fn delete_start_index(&mut self) -> &mut Self {
        self.ordered_collection_page_mut().start_index = None;
        self
    }
}

/// A subtype of Collection in which members of the logical collection are assumed to always be
/// strictly ordered.
///
/// This is just an alias for `Collection<OrderedCollectionType>` because there's no fields
/// inherent to UnorderedCollection that aren't already present on a Collection.
pub type OrderedCollection = Collection<OrderedCollectionType>;

/// The default Collection type.
///
/// This is just an alias for `Collection<CollectionType>` because there's no fields
/// inherent to UnorderedCollection that aren't already present on a Collection.
pub type UnorderedCollection = Collection<CollectionType>;

/// Used to represent distinct subsets of items from a Collection.
///
/// This is just an alias for `CollectionPage<CollectionPageType>` because there's no fields
/// inherent to UnorderedCollection that aren't already present on a CollectionPage.
pub type UnorderedCollectionPage = CollectionPage<CollectionPageType>;

/// A Collection is a subtype of Object that represents ordered or unordered sets of Object or Link
/// instances.
///
/// The items within a Collection can be ordered or unordered. The OrderedCollection type MAY be
/// used to identify a Collection whose items are always ordered. In the JSON serialization, the
/// unordered items of a Collection are represented using the items property while ordered items
/// are represented using the orderedItems property.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection<Kind> {
    /// Identifies the items contained in a collection. The items might be ordered or unordered.
    ///
    /// - Range: Object | Link | Ordered List of [ Object | Link ]
    /// - Functional: false
    items: OneOrMany<AnyBase>,

    /// A non-negative integer specifying the total number of objects contained by the logical view
    /// of the collection.
    ///
    /// This number might not reflect the actual number of items serialized within the Collection
    /// object instance.
    ///
    /// - Range: xsd:nonNegativeInteger
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    total_items: Option<u64>,

    /// In a paged Collection, indicates the page that contains the most recently updated member
    /// items.
    ///
    /// - Range: CollectionPage | Link
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    current: Option<AnyBase>,

    /// In a paged Collection, indicates the furthest preceeding page of items in the collection.
    ///
    /// - Range: CollectionPage | Link
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    first: Option<AnyBase>,

    /// In a paged Collection, indicates the furthest proceeding page of the collection.
    ///
    /// - Range: CollectionPage | Link
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    last: Option<AnyBase>,

    /// Base fields and unparsed json ends up here
    #[serde(flatten)]
    inner: Object<Kind>,
}

/// Used to represent distinct subsets of items from a Collection.
///
/// A Collection can contain a large number of items. Often, it becomes impractical for an
/// implementation to serialize every item contained by a Collection using the items (or
/// ordered_items) property alone. In such cases, the items within a Collection can be divided into
/// distinct subsets or "pages". A page is identified using the CollectionPage type.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionPage<Kind> {
    /// Identifies the Collection to which a CollectionPage objects items belong.
    ///
    /// - Range: Collection | Link
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    part_of: Option<AnyBase>,

    /// In a paged Collection, indicates the next page of items.
    ///
    /// - Range: CollectionPage | Link
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    next: Option<AnyBase>,

    /// In a paged Collection, identifies the previous page of items.
    ///
    /// - Range: CollectionPage | Link
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    prev: Option<AnyBase>,

    /// Base fields and unparsed json ends up here
    #[serde(flatten)]
    inner: Collection<Kind>,
}

/// Used to represent ordered subsets of items from an OrderedCollection.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollectionPage {
    /// A non-negative integer value identifying the relative position within the logical view of a strictly ordered collection.
    ///
    /// - Range: xsd:nonNegativeInteger
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    start_index: Option<u64>,

    /// Base fields and unparsed json ends up here
    #[serde(flatten)]
    inner: CollectionPage<OrderedCollectionPageType>,
}

impl<Kind> Collection<Kind> {
    /// Create a new Collection
    ///
    /// ```rust
    /// use activitystreams::collection::Collection;
    ///
    /// let collection = Collection::<String>::new(vec![]);
    /// ```
    pub fn new<T>(items: T) -> Self
    where
        T: Into<OneOrMany<AnyBase>>,
        Kind: Default,
    {
        Collection {
            items: items.into(),
            total_items: None,
            current: None,
            first: None,
            last: None,
            inner: Object::new(),
        }
    }

    fn extending(mut inner: Object<Kind>) -> Result<Self, serde_json::Error> {
        let items = inner.remove("items")?;
        let total_items = inner.remove("totalItems")?;
        let current = inner.remove("current")?;
        let first = inner.remove("first")?;
        let last = inner.remove("last")?;

        Ok(Collection {
            items,
            total_items,
            current,
            first,
            last,
            inner,
        })
    }

    fn retracting(self) -> Result<Object<Kind>, serde_json::Error> {
        let Collection {
            items,
            total_items,
            current,
            first,
            last,
            mut inner,
        } = self;

        inner
            .insert("last", last)?
            .insert("first", first)?
            .insert("current", current)?
            .insert("totalItems", total_items)?
            .insert("items", items)?;

        Ok(inner)
    }
}

impl<Kind> CollectionPage<Kind> {
    /// Create a new CollectionPage
    ///
    /// ```rust
    /// use activitystreams::collection::CollectionPage;
    ///
    /// let collection = CollectionPage::<String>::new(vec![]);
    /// ```
    pub fn new<T>(items: T) -> Self
    where
        T: Into<OneOrMany<AnyBase>>,
        Kind: Default,
    {
        CollectionPage {
            part_of: None,
            next: None,
            prev: None,
            inner: Collection::new(items),
        }
    }

    fn extending(object: Object<Kind>) -> Result<Self, serde_json::Error> {
        let mut inner = Collection::extending(object)?;

        let part_of = inner.remove("partOf")?;
        let next = inner.remove("next")?;
        let prev = inner.remove("prev")?;

        Ok(CollectionPage {
            part_of,
            next,
            prev,
            inner,
        })
    }

    fn retracting(self) -> Result<Object<Kind>, serde_json::Error> {
        let CollectionPage {
            part_of,
            next,
            prev,
            mut inner,
        } = self;

        inner
            .insert("prev", prev)?
            .insert("next", next)?
            .insert("partOf", part_of)?;

        inner.retracting()
    }
}

impl OrderedCollectionPage {
    /// Create a new CollectionPage
    ///
    /// ```rust
    /// use activitystreams::collection::OrderedCollectionPage;
    ///
    /// let collection = OrderedCollectionPage::new(vec![]);
    /// ```
    pub fn new<T>(items: T) -> Self
    where
        T: Into<OneOrMany<AnyBase>>,
    {
        OrderedCollectionPage {
            start_index: None,
            inner: CollectionPage::new(items),
        }
    }

    fn extending(object: Object<OrderedCollectionPageType>) -> Result<Self, serde_json::Error> {
        let mut inner = CollectionPage::extending(object)?;

        let start_index = inner.remove("startIndex")?;

        Ok(OrderedCollectionPage { start_index, inner })
    }

    fn retracting(self) -> Result<Object<OrderedCollectionPageType>, serde_json::Error> {
        let OrderedCollectionPage {
            start_index,
            mut inner,
        } = self;

        inner.insert("startIndex", start_index)?;

        inner.retracting()
    }
}

impl<Kind> markers::Base for Collection<Kind> {}
impl<Kind> markers::Object for Collection<Kind> {}
impl<Kind> markers::Collection for Collection<Kind> {}

impl<Kind> markers::Base for CollectionPage<Kind> {}
impl<Kind> markers::Object for CollectionPage<Kind> {}
impl<Kind> markers::Collection for CollectionPage<Kind> {}
impl<Kind> markers::CollectionPage for CollectionPage<Kind> {}

impl markers::Base for OrderedCollectionPage {}
impl markers::Object for OrderedCollectionPage {}
impl markers::Collection for OrderedCollectionPage {}
impl markers::CollectionPage for OrderedCollectionPage {}

impl<Inner> markers::Collection for ApObject<Inner> where Inner: markers::Collection {}
impl<Inner> markers::CollectionPage for ApObject<Inner> where Inner: markers::CollectionPage {}

impl<Kind> Extends<Kind> for Collection<Kind> {
    type Error = serde_json::Error;

    fn extends(base: Base<Kind>) -> Result<Self, Self::Error> {
        let inner = Object::extends(base)?;
        Self::extending(inner)
    }

    fn retracts(self) -> Result<Base<Kind>, Self::Error> {
        let inner = self.retracting()?;
        inner.retracts()
    }
}

impl<Kind> TryFrom<Collection<Kind>> for Object<Kind> {
    type Error = serde_json::Error;

    fn try_from(collection: Collection<Kind>) -> Result<Self, Self::Error> {
        collection.retracting()
    }
}

impl<Kind> TryFrom<Object<Kind>> for Collection<Kind> {
    type Error = serde_json::Error;

    fn try_from(object: Object<Kind>) -> Result<Self, Self::Error> {
        Self::extending(object)
    }
}

impl<Kind> Extends<Kind> for CollectionPage<Kind> {
    type Error = serde_json::Error;

    fn extends(base: Base<Kind>) -> Result<Self, Self::Error> {
        let inner = Object::extends(base)?;
        Self::extending(inner)
    }

    fn retracts(self) -> Result<Base<Kind>, Self::Error> {
        let inner = self.retracting()?;
        inner.retracts()
    }
}

impl<Kind> TryFrom<Object<Kind>> for CollectionPage<Kind> {
    type Error = serde_json::Error;

    fn try_from(object: Object<Kind>) -> Result<Self, Self::Error> {
        Self::extending(object)
    }
}

impl<Kind> TryFrom<CollectionPage<Kind>> for Object<Kind> {
    type Error = serde_json::Error;

    fn try_from(collection_page: CollectionPage<Kind>) -> Result<Self, Self::Error> {
        collection_page.retracting()
    }
}

impl Extends<OrderedCollectionPageType> for OrderedCollectionPage {
    type Error = serde_json::Error;

    fn extends(base: Base<OrderedCollectionPageType>) -> Result<Self, Self::Error> {
        let inner = Object::extends(base)?;
        Self::extending(inner)
    }

    fn retracts(self) -> Result<Base<OrderedCollectionPageType>, Self::Error> {
        let inner = self.retracting()?;
        inner.retracts()
    }
}

impl TryFrom<Object<OrderedCollectionPageType>> for OrderedCollectionPage {
    type Error = serde_json::Error;

    fn try_from(object: Object<OrderedCollectionPageType>) -> Result<Self, Self::Error> {
        Self::extending(object)
    }
}

impl TryFrom<OrderedCollectionPage> for Object<OrderedCollectionPageType> {
    type Error = serde_json::Error;

    fn try_from(collection_page: OrderedCollectionPage) -> Result<Self, Self::Error> {
        collection_page.retracting()
    }
}

impl<Kind> UnparsedMut for Collection<Kind> {
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl<Kind> UnparsedMut for CollectionPage<Kind> {
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl UnparsedMut for OrderedCollectionPage {
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl<Kind> AsBase<Kind> for Collection<Kind> {
    fn base_ref(&self) -> &Base<Kind> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<Kind> {
        self.inner.base_mut()
    }
}

impl<Kind> AsObject<Kind> for Collection<Kind> {
    fn object_ref(&self) -> &Object<Kind> {
        &self.inner
    }

    fn object_mut(&mut self) -> &mut Object<Kind> {
        &mut self.inner
    }
}

impl<Kind> AsCollection<Kind> for Collection<Kind> {
    fn collection_ref(&self) -> &Collection<Kind> {
        self
    }

    fn collection_mut(&mut self) -> &mut Collection<Kind> {
        self
    }
}

impl<Kind> AsBase<Kind> for CollectionPage<Kind> {
    fn base_ref(&self) -> &Base<Kind> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<Kind> {
        self.inner.base_mut()
    }
}

impl<Kind> AsObject<Kind> for CollectionPage<Kind> {
    fn object_ref(&self) -> &Object<Kind> {
        self.inner.object_ref()
    }

    fn object_mut(&mut self) -> &mut Object<Kind> {
        self.inner.object_mut()
    }
}

impl<Kind> AsCollection<Kind> for CollectionPage<Kind> {
    fn collection_ref(&self) -> &Collection<Kind> {
        &self.inner
    }

    fn collection_mut(&mut self) -> &mut Collection<Kind> {
        &mut self.inner
    }
}

impl<Kind> AsCollectionPage<Kind> for CollectionPage<Kind> {
    fn collection_page_ref(&self) -> &CollectionPage<Kind> {
        self
    }

    fn collection_page_mut(&mut self) -> &mut CollectionPage<Kind> {
        self
    }
}

impl AsBase<OrderedCollectionPageType> for OrderedCollectionPage {
    fn base_ref(&self) -> &Base<OrderedCollectionPageType> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<OrderedCollectionPageType> {
        self.inner.base_mut()
    }
}

impl AsObject<OrderedCollectionPageType> for OrderedCollectionPage {
    fn object_ref(&self) -> &Object<OrderedCollectionPageType> {
        self.inner.object_ref()
    }

    fn object_mut(&mut self) -> &mut Object<OrderedCollectionPageType> {
        self.inner.object_mut()
    }
}

impl AsCollection<OrderedCollectionPageType> for OrderedCollectionPage {
    fn collection_ref(&self) -> &Collection<OrderedCollectionPageType> {
        self.inner.collection_ref()
    }

    fn collection_mut(&mut self) -> &mut Collection<OrderedCollectionPageType> {
        self.inner.collection_mut()
    }
}

impl AsCollectionPage<OrderedCollectionPageType> for OrderedCollectionPage {
    fn collection_page_ref(&self) -> &CollectionPage<OrderedCollectionPageType> {
        &self.inner
    }

    fn collection_page_mut(&mut self) -> &mut CollectionPage<OrderedCollectionPageType> {
        &mut self.inner
    }
}

impl AsOrderedCollectionPage for OrderedCollectionPage {
    fn ordered_collection_page_ref(&self) -> &OrderedCollectionPage {
        self
    }

    fn ordered_collection_page_mut(&mut self) -> &mut OrderedCollectionPage {
        self
    }
}

impl<Inner, Kind> AsCollection<Kind> for ApObject<Inner>
where
    Inner: AsCollection<Kind>,
{
    fn collection_ref(&self) -> &Collection<Kind> {
        self.inner().collection_ref()
    }

    fn collection_mut(&mut self) -> &mut Collection<Kind> {
        self.inner_mut().collection_mut()
    }
}

impl<Inner, Kind> AsCollectionPage<Kind> for ApObject<Inner>
where
    Inner: AsCollectionPage<Kind>,
{
    fn collection_page_ref(&self) -> &CollectionPage<Kind> {
        self.inner().collection_page_ref()
    }

    fn collection_page_mut(&mut self) -> &mut CollectionPage<Kind> {
        self.inner_mut().collection_page_mut()
    }
}

impl<Inner> AsOrderedCollectionPage for ApObject<Inner>
where
    Inner: AsOrderedCollectionPage,
{
    fn ordered_collection_page_ref(&self) -> &OrderedCollectionPage {
        self.inner().ordered_collection_page_ref()
    }

    fn ordered_collection_page_mut(&mut self) -> &mut OrderedCollectionPage {
        self.inner_mut().ordered_collection_page_mut()
    }
}

impl<T, Kind> CollectionExt<Kind> for T where T: AsCollection<Kind> {}
impl<T, Kind> CollectionPageExt<Kind> for T where T: AsCollectionPage<Kind> {}
impl<T> OrderedCollectionPageExt for T where T: AsOrderedCollectionPage {}
