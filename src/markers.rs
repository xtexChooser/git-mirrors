//! Marker traits for bounding methods
//!
//! ```rust
//! use activitystreams::{base::BaseExt, markers::Activity};
//!
//! /// Applies the name "hi" to any given activity
//! fn manipulator<T, Kind>(mut some_type: T) -> T
//! where
//!     T: Activity + BaseExt<Kind>,
//! {
//!     some_type.set_name("hi");
//!
//!     some_type
//! }
//! ```

/// The lowermost trait of the trait structure
///
/// Base exists solely so Object and Link can have impls that don't potentially conflict
pub trait Base {}

/// Describes an object of any kind.
///
/// The Object type serves as the base type for most of the other kinds of objects defined in the
/// Activity Vocabulary, including other Core types such as `Activity`, `IntransitiveActivity`,
/// `Collection` and `OrderedCollection`.
pub trait Object: Base {}

/// A Link is an indirect, qualified reference to a resource identified by a URL.
///
/// The fundamental model for links is established by
/// [[RFC5988](https://tools.ietf.org/html/rfc5988)]. Many of the properties defined by the
/// Activity Vocabulary allow values that are either instances of Object or Link. When a Link is
/// used, it establishes a qualified relation connecting the subject (the containing object) to the
/// resource identified by the href. Properties of the Link are properties of the reference as
/// opposed to properties of the resource.
pub trait Link: Base {}

/// A Collection is a subtype of `Object` that represents ordered or unordered sets of `Object` or
/// `Link` instances.
///
/// The items within a Collection can be ordered or unordered. The OrderedCollection type MAY be
/// used to identify a Collection whose items are always ordered. In the JSON serialization, the
/// unordered items of a Collection are represented using the items property while ordered items
/// are represented using the orderedItems property.
pub trait Collection: Object {}

/// Used to represent distinct subsets of items from a Collection.
///
/// A `Collection` can contain a large number of items. Often, it becomes impractical for an
/// implementation to serialize every item contained by a `Collection` using the items (or
/// `ordered_items`) property alone. In such cases, the items within a `Collection` can be divided
/// into distinct subsets or "pages". A page is identified using the `CollectionPage` type.
pub trait CollectionPage: Collection {}

/// `Actor` types are `Object` types that are capable of performing activities.
///
/// This specification intentionally defines `Actors` in only the most generalized way, stopping
/// short of defining semantically specific properties for each. All Actor objects are
/// specializations of `Object` and inherit all of the core properties common to all Objects.
/// External vocabularies can be used to express additional detail not covered by the Activity
/// Vocabulary. VCard [[vcard-rdf](https://www.w3.org/TR/vcard-rdf/) SHOULD be used to provide
/// additional metadata for `Person`, `Group`, and `Organization` instances.
///
/// While implementations are free to introduce new types of Actors beyond those defined by the
/// Activity Vocabulary, interoperability issues can arise when applications rely too much on
/// extension types that are not recognized by other implementations. Care should be taken to not
/// unduly overlap with or duplicate the existing `Actor` types.
///
/// When an implementation uses an extension type that overlaps with a core vocabulary type, the
/// implementation MUST also specify the core vocabulary type. For instance, some vocabularies
/// (e.g. VCard) define their own types for describing people. An implementation that wishes, for
/// example, to use a `vcard:Individual` as an `Actor` MUST also identify that `Actor` as a
/// `Person`.
pub trait Actor: Object {}

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
