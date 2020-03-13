/*
 * This file is part of ActivityStreams Derive.
 *
 * Copyright © 2020 Riley Trautman
 *
 * ActivityStreams Derive is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * ActivityStreams Derive is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with ActivityStreams Derive.  If not, see <http://www.gnu.org/licenses/>.
 */

//! Derive macros for Activity Streams
//!
//! ## Examples
//!
//! First, add `serde` and `activitystreams-derive` to your Cargo.toml
//! ```toml
//! activitystreams-derive = "0.5.0-alpha.0"
//! # or activitystreams = "0.5.0-alpha.0"
//! serde = { version = "1.0", features = ["derive"] }
//! ```
//!
//! ```rust
//! use activitystreams_derive::{properties, UnitString};
//! // or activitystreams::{properties, UnitString};
//! use serde_json::Value;
//!
//! /// Using the UnitString derive macro
//! ///
//! /// This macro implements Serialize and Deserialize for the given type, making this type
//! /// represent the string "SomeKind" in JSON.
//! #[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, UnitString)]
//! #[unit_string(SomeKind)]
//! pub struct MyKind;
//!
//! /// Using the properties macro
//! ///
//! /// This macro generates getters and setters for the associated fields.
//! properties! {
//!     My {
//!         context {
//!             types [
//!                 String,
//!             ],
//!             rename("@context"),
//!         },
//!         kind {
//!             types [
//!                 MyKind,
//!             ],
//!             functional,
//!             required,
//!             rename("type"),
//!         },
//!         required_key {
//!             types [
//!                 Value,
//!             ],
//!             functional,
//!             required,
//!             alias [
//!                 "someKey",
//!                 "existingKey",
//!                 "woooKey",
//!             ],
//!         },
//!     }
//! }
//!
//! fn main () -> Result<(), Box<dyn std::error::Error>> {
//!     let s = r#"{
//!         "@context": "http://www.w3c.org/ns#activitystreams",
//!         "type": "SomeKind",
//!         "woooKey": {
//!             "key": "value"
//!         }
//!     }"#;
//!
//!     let m: MyProperties = serde_json::from_str(s)?;
//!     assert_eq!(&MyKind, m.get_kind());
//!     Ok(())
//! }
//! ```

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::{quote, ToTokens};
use syn::{
    braced, bracketed, parenthesized,
    parse::{Parse, ParseStream, Peek},
    parse_macro_input,
    punctuated::Punctuated,
    token, Attribute, Data, DeriveInput, Fields, Ident, LitStr, Result, Token, Type,
};

/// Derive implementations for activitystreams objects
///
/// ```ignore
/// #[derive(Clone, Debug, serde::Deserialize, serde::Serialize, PropRefs)]
/// #[prop_refs(Object)]
/// pub struct MyStruct {
///     /// Derive AsRef<MyProperties> and AsMut<MyProperties> delegating to `my_field`
///     #[prop_refs]
///     my_field: MyProperties,
///
///     /// Derive the above, plus Object (activitystreams)
///     #[prop_refs]
///     obj_field: ObjectProperties,
/// }
/// ```
#[proc_macro_derive(PropRefs, attributes(prop_refs))]
pub fn ref_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    let name = input.ident;

    let data = match input.data {
        Data::Struct(s) => s,
        _ => panic!("Can only derive for structs"),
    };

    let fields = match data.fields {
        Fields::Named(fields) => fields,
        _ => panic!("Can only derive for named fields"),
    };

    let name2 = name.clone();
    let base_impls: proc_macro2::TokenStream = input
        .attrs
        .iter()
        .filter_map(move |attr| {
            if attr
                .path
                .segments
                .last()
                .map(|segment| segment.ident == "prop_refs")
                .unwrap_or(false)
            {
                let object = from_value(attr.clone());
                let name = name2.clone();
                let box_name = Ident::new(&format!("{}Box", object), name.span());

                Some(quote! {
                    impl #object for #name {}

                    impl std::convert::TryFrom<#name> for #box_name {
                        type Error = serde_json::Error;

                        fn try_from(s: #name) -> Result<Self, Self::Error> {
                            #box_name::from_concrete(s)
                        }
                    }
                })
            } else {
                None
            }
        })
        .collect();

    let tokens: proc_macro2::TokenStream = fields
        .named
        .iter()
        .filter_map(|field| {
            let our_attr = field.attrs.iter().find(|attribute| {
                attribute
                    .path
                    .segments
                    .last()
                    .map(|segment| segment.ident == "prop_refs")
                    .unwrap_or(false)
            });

            our_attr.map(move |_| (field.ident.clone().unwrap(), field.ty.clone()))
        })
        .map(move |(ident, ty)| {
            let name = name.clone();
            quote! {
                impl AsRef<#ty> for #name {
                    fn as_ref(&self) -> &#ty {
                        &self.#ident
                    }
                }

                impl AsMut<#ty> for #name {
                    fn as_mut(&mut self) -> &mut #ty {
                        &mut self.#ident
                    }
                }
            }
        })
        .collect();

    let full = quote! {
        #base_impls
        #tokens
    };

    full.into()
}

/// Derive a wrapper type based on serde_json::Value to contain any possible trait type
///
/// The following code
/// ```ignore
/// #[wrapper_type]
/// pub trait Object {}
/// ```
/// produces the following type
/// ```ignore
/// pub struct ObjectBox(pub serde_json::Value);
///
/// impl ObjectBox {
///     pub fn from_concrete<T>(t: T) -> Result<Self, serde_json::Error>
///     where
///         T: Object + serde::ser::Serialize,
///     {
///         Ok(ObjectBox(serde_json::to_value(t)?))
///     }
///
///     pub fn to_concrete<T>(self) -> Result<T, serde_json::Error>
///     where
///         T: Object + serde::de::DeserializeOwned,
///     {
///         serde_json::from_value(self.0)
///     }
///
///     pub fn is_type(&self, kind: impl std::fmt::Display) -> bool {
///         self.0["type"] == kind.to_string()
///     }
///
///     pub fn type(&self) -> Option<&str> {
///         match self.0["type"] {
///             serde_json::Value::String(ref s) -> Some(s),
///             _ => None,
///         }
///     }
/// }
#[proc_macro_attribute]
pub fn wrapper_type(_: TokenStream, input: TokenStream) -> TokenStream {
    let input: syn::ItemTrait = syn::parse(input).unwrap();
    let trait_name = input.ident.clone();
    let type_name = Ident::new(&format!("{}Box", trait_name), trait_name.span());

    let doc_line = to_doc(&format!("A wrapper type around a generic `{}`", trait_name));
    let tokens = quote! {
        #input

        #doc_line
        #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
        #[serde(transparent)]
        pub struct #type_name(pub serde_json::Value);

        impl #type_name {
            /// Coerce a concrete type into this wrapper type
            ///
            /// This is done automatically via TryFrom in proprties setter methods
            pub fn from_concrete<T>(t: T) -> Result<Self, serde_json::Error>
            where
                T: #trait_name + serde::ser::Serialize,
            {
                Ok(#type_name(serde_json::to_value(t)?))
            }

            /// Attempt to deserialize the wrapper type to a concrete type
            ///
            /// Before this method is called, the type should be verified via the `kind` or
            /// `is_kind` methods
            pub fn to_concrete<T>(self) -> Result<T, serde_json::Error>
            where
                T: #trait_name + serde::de::DeserializeOwned,
            {
                serde_json::from_value(self.0)
            }

            /// Return whether the given wrapper type is expected.
            ///
            /// For example
            /// ```ignore
            /// use activitystreams::object::{
            ///     kind::ImageType,
            ///     apub::Image,
            /// };
            /// if my_wrapper_type.is_kind(ImageType) {
            ///     let image = my_wrapper_type.to_concrete::<Image>()?;
            ///     ...
            /// }
            /// ```
            pub fn is_kind(&self, kind: impl std::fmt::Display) -> bool {
                self.0["type"] == kind.to_string()
            }

            /// Return the kind of wrapper type, if present
            ///
            /// Example
            /// ```ignore
            /// match my_wrapper_type.kind() {
            ///     Some("Image") => {
            ///         let image = my_wrapper_type.to_concrete::<Image>()?;
            ///         ...
            ///     }
            ///     _ => ...,
            /// }
            /// ```
            pub fn kind(&self) -> Option<&str> {
                match self.0["type"] {
                    serde_json::Value::String(ref s) => Some(s),
                    _ => None,
                }
            }
        }
    };

    tokens.into()
}

/// Derive implementations Serialize and Deserialize for a constant string Struct type
///
/// ```ignore
/// /// Derive Serialize and Deserialize such that MyStruct becomes the "MyType" string
/// #[derive(Clone, Debug, UnitString)]
/// #[unit_string(MyType)]
/// pub struct MyStruct;
///
/// // usage
/// let _: HashMap<String, MyStruct> = serde_json::from_str(r#"{"type":"MyType"}"#)?;
/// ```
#[proc_macro_derive(UnitString, attributes(unit_string))]
pub fn unit_string(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    let name = input.ident;

    let attr = input
        .attrs
        .iter()
        .find(|attribute| {
            attribute
                .path
                .segments
                .last()
                .map(|segment| segment.ident == "unit_string")
                .unwrap_or(false)
        })
        .unwrap()
        .clone();

    let visitor_name = from_value(attr);
    let value = format!("{}", visitor_name);

    let serialize = quote! {
        impl ::serde::ser::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(#value)
            }
        }
    };

    let expecting = quote! {
        fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(formatter, "The string '{}'", #value)
        }
    };

    let visit = quote! {
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: ::serde::de::Error,
        {
            if v == #value {
                Ok(#name)
            } else {
                Err(::serde::de::Error::custom("Invalid type"))
            }
        }
    };

    let visitor = quote! {
        struct #visitor_name;

        impl<'de> ::serde::de::Visitor<'de> for #visitor_name {
            type Value = #name;

            #expecting

            #visit
        }
    };

    let deserialize = quote! {
        impl<'de> ::serde::de::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<#name, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                deserializer.deserialize_str(#visitor_name)
            }
        }
    };

    let display = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", #value)
            }
        }
    };

    let c = quote! {
        #serialize
        #visitor
        #deserialize
        #display
    };

    c.into()
}

fn from_value(attr: Attribute) -> Ident {
    let group = attr
        .tokens
        .clone()
        .into_iter()
        .filter_map(|token_tree| match token_tree {
            TokenTree::Group(group) => Some(group),
            _ => None,
        })
        .next()
        .unwrap();

    group
        .stream()
        .clone()
        .into_iter()
        .filter_map(|token_tree| match token_tree {
            TokenTree::Ident(ident) => Some(ident),
            _ => None,
        })
        .next()
        .unwrap()
}

fn to_doc(s: &String) -> proc_macro2::TokenStream {
    format!("/// {}", s).parse().unwrap()
}

fn many_docs(v: &Vec<String>) -> proc_macro2::TokenStream {
    v.iter()
        .map(|d| {
            let d = to_doc(d);
            quote! {
                #d
            }
        })
        .collect()
}

/// Generate structs and enums for activitystreams objects
///
/// ```rust
/// use activitystreams_derive::properties;
///
/// #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
/// pub struct MyStruct;
///
/// properties! {
///     Hello {
///         docs [ "Defining the HelloProperties struct" ],
///
///         field {
///             types [ String ],
///         },
///
///         other_field {
///             docs [
///                 "field documentation",
///                 "is cool",
///             ],
///             types [
///                 String,
///                 MyStruct,
///             ],
///             functional,
///             required,
///             rename("@other_field"),
///             alias [
///                 "@second_field",
///                 "another_field",
///             ],
///         },
///     }
/// }
///
/// fn main() {
///     let _ = HelloProperties::default();
/// }
/// ```
#[proc_macro]
pub fn properties(tokens: TokenStream) -> TokenStream {
    let Properties { name, docs, fields } = parse_macro_input!(tokens as Properties);

    let docs: proc_macro2::TokenStream = many_docs(&docs);

    let name = Ident::new(&format!("{}Properties", name), name.span());

    let (fields, deps): (Vec<_>, Vec<_>) = fields.iter().filter_map(|field| {
        if field.description.types.is_empty() {
            return None;
        }

        let fname = field.name.clone();
        let fdocs: proc_macro2::TokenStream = many_docs(&field.description.docs);

        let (ty, deps) = if field.description.types.len() == 1 {
            let ty = Ident::new(&field.description.types.first().unwrap().to_token_stream().to_string(), fname.span());
            if field.description.functional {
                (ty, None)
            } else {
                let enum_ty = Ident::new(&camelize(&format!("{}_{}_enum", name, fname)), fname.span());
                let doc_lines = many_docs(&vec![
                    format!("Variations for the `{}` field from `{}", fname, name),
                    String::new(),
                    format!("`{}` isn't functional, meaning it can be represented as either a single `{}` or a vector of `{}`.", fname, ty, ty),
                ]);
                let deps = quote! {
                    #doc_lines
                    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
                    #[serde(rename_all = "camelCase")]
                    #[serde(untagged)]
                    pub enum #enum_ty {
                        Term(#ty),
                        Array(Vec<#ty>),
                    }

                    impl Default for #enum_ty {
                        fn default() -> Self {
                            #enum_ty::Array(Vec::new())
                        }
                    }

                    impl From<#ty> for #enum_ty {
                        fn from(t: #ty) -> Self {
                            #enum_ty::Term(t)
                        }
                    }

                    impl From<Vec<#ty>> for #enum_ty {
                        fn from(v: Vec<#ty>) -> Self {
                            #enum_ty::Array(v)
                        }
                    }
                };

                (enum_ty, Some(deps))
            }
        } else {
            let ty = Ident::new(&camelize(&format!("{}_{}_enum", name, fname)), fname.span());

            let v_tokens: proc_macro2::TokenStream = field
                .description
                .types
                .iter()
                .map(|v_ty| {
                    quote! {
                        #v_ty(#v_ty),
                    }
                })
                .collect();

            let first_type = field.description.types.iter().next().unwrap().clone();

            let deps = if !field.description.functional {
                let term_ty = Ident::new(&camelize(&format!("{}_{}_term_enum", name, fname)), fname.span());

                let from_tokens: proc_macro2::TokenStream = field
                    .description
                    .types
                    .iter()
                    .map(|v_ty| {
                        quote! {
                            impl From<#v_ty> for #term_ty {
                                fn from(item: #v_ty) -> #term_ty {
                                    #term_ty::#v_ty(item)
                                }
                            }
                        }
                    })
                    .collect();

                let term_doc_lines = many_docs(&vec![
                    format!("Terminating variations for the `{}` field from `{}`", fname, name),
                    String::new(),
                    format!("Since {} can be one of multiple types, this enum represents all possibilities of {}", fname, fname),
                ]);
                let doc_lines = many_docs(&vec![
                    format!("Non-Terminating variations for the `{}` field from `{}`", fname, name),
                    String::new(),
                    format!("`{}` isn't functional, meaning it can be represented as either a single `{}` or a vector of `{}`", fname, term_ty, term_ty),
                ]);
                quote! {
                    #term_doc_lines
                    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
                    #[serde(rename_all = "camelCase")]
                    #[serde(untagged)]
                    pub enum #term_ty {
                        #v_tokens
                    }

                    #doc_lines
                    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
                    #[serde(rename_all = "camelCase")]
                    #[serde(untagged)]
                    pub enum #ty {
                        Term(#term_ty),
                        Array(Vec<#term_ty>),
                    }

                    impl Default for #ty {
                        fn default() -> Self {
                            #ty::Array(Vec::new())
                        }
                    }

                    impl From<#term_ty> for #ty {
                        fn from(term: #term_ty) -> Self {
                            #ty::Term(term)
                        }
                    }

                    impl From<Vec<#term_ty>> for #ty {
                        fn from(v: Vec<#term_ty>) -> Self {
                            #ty::Array(v)
                        }
                    }

                    #from_tokens
                }
            } else {
                let from_tokens: proc_macro2::TokenStream = field
                    .description
                    .types
                    .iter()
                    .map(|v_ty| {
                        quote! {
                            impl From<#v_ty> for #ty {
                                fn from(item: #v_ty) -> #ty {
                                    #ty::#v_ty(item)
                                }
                            }
                        }
                    })
                    .collect();

                let doc_lines = many_docs(&vec![
                    format!("Variations for the `{}` field from `{}`", fname, name),
                    String::new(),
                    format!("`{}` isn't functional, meaning it can only be represented as a single `{}`", fname, ty),
                    String::new(),
                    format!("This enum's variants represent all valid types to construct a `{}`", fname),
                ]);
                quote! {
                    #doc_lines
                    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
                    #[serde(rename_all = "camelCase")]
                    #[serde(untagged)]
                    pub enum #ty {
                        #v_tokens
                    }

                    impl Default for #ty {
                        fn default() -> Self {
                            #ty::#first_type(Default::default())
                        }
                    }

                    #from_tokens
                }
            };

            (ty, Some(deps))
        };

        let alias_tokens: proc_macro2::TokenStream = field.description.aliases.iter().map(|alias| quote!{
            #[serde(alias = #alias)]
        }).collect();
        let rename_tokens: proc_macro2::TokenStream = field.description.rename.iter().map(|rename| quote!{
            #[serde(rename = #rename)]
        }).collect();

        let field_tokens = if field.description.required {
            quote! {
                pub #fname: #ty,
            }
        } else {
            quote! {
                #[serde(skip_serializing_if = "Option::is_none")]
                pub #fname: Option<#ty>,
            }
        };

        let field_tokens = quote!{
            #fdocs
            #rename_tokens
            #alias_tokens
            #field_tokens
        };

        let fns = if field.description.types.len() == 1 {
            let v_ty = field.description.types.first().unwrap().clone();

            let set_ident =
                Ident::new(&format!("set_{}", fname), fname.span());
            let get_ident =
                Ident::new(&format!("get_{}", fname), fname.span());

            let enum_ty = Ident::new(&camelize(&format!("{}_{}_enum", name, fname)), fname.span());

            let set_many_ident =
                Ident::new(&format!("set_many_{}s", fname), fname.span());
            let get_many_ident =
                Ident::new(&format!("get_many_{}s", fname), fname.span());

            if field.description.required {
                if field.description.functional {
                    let doc_line = to_doc(&format!("Set `{}` with a type that can be cnoverted into a `{}`", fname, v_ty.to_token_stream()));
                    let set = quote! {
                        #doc_line
                        pub fn #set_ident<T>(&mut self, item: T) -> Result<&mut Self, <T as std::convert::TryInto<#v_ty>>::Error>
                        where
                            T: std::convert::TryInto<#v_ty>,
                        {
                            use std::convert::TryInto;
                            self.#fname = item.try_into()?;
                            Ok(self)
                        }
                    };

                    let doc_line = to_doc(&format!("Get the `{}` as `{}`", fname, v_ty.to_token_stream()));
                    let get = quote! {
                        #doc_line
                        pub fn #get_ident(&self) -> &#v_ty {
                            &self.#fname
                        }
                    };

                    quote!{
                        #get
                        #set
                    }
                } else {
                    let doc_line = to_doc(&format!("Set `{}` with a type that can be converted into a `{}`", fname, v_ty.to_token_stream()));
                    let set = quote! {
                        #doc_line
                        pub fn #set_ident<T>(&mut self, item: T) -> Result<&mut Self, <T as std::convert::TryInto<#v_ty>>::Error>
                        where
                            T: std::convert::TryInto<#v_ty>,
                        {
                            use std::convert::TryInto;
                            self.#fname = #enum_ty::Term(item.try_into()?);
                            Ok(self)
                        }
                    };

                    let doc_line = to_doc(&format!("Get the `{}` as `{}`", fname, v_ty.to_token_stream()));
                    let get = quote! {
                        #doc_line
                        ///
                        /// This returns `None` when there is more than one item
                        pub fn #get_ident(&self) -> Option<&#v_ty> {
                            match self.#fname {
                                #enum_ty::Term(ref term) => Some(term),
                                _ => None,
                            }
                        }
                    };

                    let doc_line = to_doc(&format!("Set the `{}` with a vector of types that can be converted into `{}`s", fname, v_ty.to_token_stream()));
                    let set_many = quote! {
                        #doc_line
                        pub fn #set_many_ident<T>(&mut self, item: Vec<T>) -> Result<&mut Self, <T as std::convert::TryInto<#v_ty>>::Error>
                        where
                            T: std::convert::TryInto<#v_ty>,
                        {
                            let item: Vec<#v_ty> = item.into_iter().map(std::convert::TryInto::try_into).collect::<Result<Vec<_>, _>>()?;
                            self.#fname = #enum_ty::Array(item);
                            Ok(self)
                        }
                    };

                    let doc_line = to_doc(&format!("Get the `{}` as a slice of `{}`", fname, v_ty.to_token_stream()));
                    let get_many = quote! {
                        #doc_line
                        ///
                        /// This returns `None` if
                        /// - There is only one element
                        pub fn #get_many_ident(&self) -> Option<&[#v_ty]> {
                            match self.#fname {
                                #enum_ty::Array(ref array) => Some(array),
                                _ => None,
                            }
                        }
                    };

                    quote! {
                        #get
                        #set
                        #get_many
                        #set_many
                    }
                }
            } else {
                if field.description.functional {
                    let doc_line = to_doc(&format!("Set the `{}` with a type that can be converted into `{}`", fname, v_ty.to_token_stream()));
                    let set = quote! {
                        #doc_line
                        pub fn #set_ident<T>(&mut self, item: T) -> Result<&mut Self, <T as std::convert::TryInto<#v_ty>>::Error>
                        where
                            T: std::convert::TryInto<#v_ty>,
                        {
                            use std::convert::TryInto;
                            self.#fname = Some(item.try_into()?);
                            Ok(self)
                        }
                    };

                    let doc_line = to_doc(&format!("Get `{}` as a `{}`", fname, v_ty.to_token_stream()));
                    let get = quote! {
                        #doc_line
                        ///
                        /// This returns `None` if there is no value present
                        pub fn #get_ident(&self) -> Option<&#v_ty> {
                            self.#fname.as_ref()
                        }
                    };

                    quote!{
                        #get
                        #set
                    }
                } else {
                    let doc_line = to_doc(&format!("Set the `{}` with a type that can be converted into `{}`", fname, v_ty.to_token_stream()));
                    let set = quote! {
                        #doc_line
                        pub fn #set_ident<T>(&mut self, item: T) -> Result<&mut Self, <T as std::convert::TryInto<#v_ty>>::Error>
                        where
                            T: std::convert::TryInto<#v_ty>,
                        {
                            use std::convert::TryInto;
                            self.#fname = Some(#enum_ty::Term(item.try_into()?));
                            Ok(self)
                        }
                    };

                    let doc_line = to_doc(&format!("Get `{}` as a `{}`", fname, v_ty.to_token_stream()));
                    let get = quote! {
                        #doc_line
                        ///
                        /// This returns `None` if
                        /// - There is no value present
                        /// - There is more than one value present
                        pub fn #get_ident(&self) -> Option<&#v_ty> {
                            match self.#fname {
                                Some(#enum_ty::Term(ref term)) => Some(term),
                                _ => None,
                            }
                        }
                    };

                    let doc_line = to_doc(&format!("Set the `{}` with a vector of types that can be converted into `{}`s", fname, v_ty.to_token_stream()));
                    let set_many = quote! {
                        #doc_line
                        pub fn #set_many_ident<T>(&mut self, item: Vec<T>) -> Result<&mut Self, <T as std::convert::TryInto<#v_ty>>::Error>
                        where
                            T: std::convert::TryInto<#v_ty>,
                        {
                            let item: Vec<#v_ty> = item.into_iter().map(std::convert::TryInto::try_into).collect::<Result<Vec<_>, _>>()?;
                            self.#fname = Some(#enum_ty::Array(item));
                            Ok(self)
                        }
                    };

                    let doc_line = to_doc(&format!("Get `{}` as a slice of `{}`s", fname, v_ty.to_token_stream()));
                    let get_many = quote! {
                        #doc_line
                        ///
                        /// This returns `None` if
                        /// - There is no value present
                        /// - There is only one value present
                        pub fn #get_many_ident(&self) -> Option<&[#v_ty]> {
                            match self.#fname {
                                Some(#enum_ty::Array(ref a)) => Some(a),
                                _ => None,
                            }
                        }
                    };

                    quote! {
                        #get
                        #set
                        #get_many
                        #set_many
                    }
                }
            }
        } else if field.description.functional {
            let tokens: proc_macro2::TokenStream = field
                .description
                .types
                .iter()
                .map(|v_ty| {
                    let set_ident =
                        Ident::new(&format!("set_{}_{}", fname, snakize(&v_ty.to_token_stream().to_string())), fname.span());
                    let get_ident =
                        Ident::new(&format!("get_{}_{}", fname, snakize(&v_ty.to_token_stream().to_string())), fname.span());

                    if field.description.required {
                        let doc_line = to_doc(&format!("Set the `{}` with a type that can be converted into `{}`", fname, v_ty.to_token_stream()));
                        let set = quote! {
                            #doc_line
                            pub fn #set_ident<T>(&mut self, item: T) -> Result<&mut Self, <T as std::convert::TryInto<#v_ty>>::Error>
                            where
                                T: std::convert::TryInto<#v_ty>,
                            {
                                use std::convert::TryInto;
                                let item: #v_ty = item.try_into()?;
                                self.#fname = item.into();
                                Ok(self)
                            }
                        };

                        let doc_line = to_doc(&format!("Get `{}` as a slice of `{}`s", fname, v_ty.to_token_stream()));
                        let get = quote! {
                            #doc_line
                            ///
                            /// This returns `None` if
                            /// - The requested type is not the stored type
                            pub fn #get_ident(&self) -> Option<&#v_ty> {
                                match self.#fname {
                                    #ty::#v_ty(ref term) => Some(term),
                                    _ => None,
                                }
                            }
                        };

                        quote! {
                            #get
                            #set
                        }
                    } else {
                        let doc_line = to_doc(&format!("Set `{}` with a value that can be converted into `{}`", fname, v_ty.to_token_stream()));
                        let set = quote! {
                            #doc_line
                            pub fn #set_ident<T>(&mut self, item: T) -> Result<&mut Self, <T as std::convert::TryInto<#v_ty>>::Error>
                            where
                                T: std::convert::TryInto<#v_ty>,
                            {
                                use std::convert::TryInto;
                                let item: #v_ty = item.try_into()?;
                                self.#fname = Some(item.into());
                                Ok(self)
                            }
                        };

                        let doc_line = to_doc(&format!("Get `{}` as a `{}`", fname, v_ty.to_token_stream()));
                        let get = quote! {
                            #doc_line
                            ///
                            /// This returns `None` if
                            /// - There is no value present
                            /// - The requested type is not the stored type
                            pub fn #get_ident(&self) -> Option<&#v_ty> {
                                match self.#fname {
                                    Some(#ty::#v_ty(ref term)) => Some(term),
                                    _ => None,
                                }
                            }
                        };

                        quote! {
                            #get
                            #set
                        }
                    }
                })
                .collect();

            quote! {
                #tokens
            }
        } else {
            let term_ty = Ident::new(&camelize(&format!("{}_{}_term_enum", name, fname)), fname.span());
            let tokens: proc_macro2::TokenStream = field
                .description
                .types
                .iter()
                .map(|v_ty| {
                    let set_ident =
                        Ident::new(&format!("set_{}_{}", fname, snakize(&v_ty.to_token_stream().to_string())), fname.span());
                    let get_ident =
                        Ident::new(&format!("get_{}_{}", fname, snakize(&v_ty.to_token_stream().to_string())), fname.span());

                    let set_many_ident =
                        Ident::new(&format!("set_many_{}_{}s", fname, snakize(&v_ty.to_token_stream().to_string())), fname.span());
                    let get_many_ident =
                        Ident::new(&format!("get_many_{}_{}s", fname, snakize(&v_ty.to_token_stream().to_string())), fname.span());

                    if field.description.required {
                        let doc_line = to_doc(&format!("Set `{}` with a value that can be converted into `{}`", fname, v_ty.to_token_stream()));
                        let set = quote! {
                            #doc_line
                            pub fn #set_ident<T>(&mut self, item: T) -> Result<&mut Self, <T as std::convert::TryInto<#v_ty>>::Error>
                            where
                                T: std::convert::TryInto<#v_ty>,
                            {
                                use std::convert::TryInto;
                                let item: #v_ty = item.try_into()?;
                                let item: #term_ty = item.into();
                                self.#fname = item.into();
                                Ok(self)
                            }
                        };

                        let doc_line = to_doc(&format!("Get the `{}` as a `{}`", fname, v_ty.to_token_stream()));
                        let get = quote! {
                            #doc_line
                            ///
                            /// This returns `None` if
                            /// - There is more than one value present
                            /// - The requested type is not the stored type
                            pub fn #get_ident(&self) -> Option<&#v_ty> {
                                match self.#fname {
                                    #ty::Term(#term_ty::#v_ty(ref term)) => Some(term),
                                    _ => None,
                                }
                            }
                        };

                        let doc_line = to_doc(&format!("Set `{}` from a vec of items that can be converted into `{}`s", fname, v_ty.to_token_stream()));
                        let set_many = quote! {
                            #doc_line
                            pub fn #set_many_ident<T>(&mut self, item: Vec<T>) -> Result<&mut Self, <T as std::convert::TryInto<#v_ty>>::Error>
                            where
                                T: std::convert::TryInto<#v_ty>,
                            {
                                let item: Vec<#v_ty> = item.into_iter().map(std::convert::TryInto::try_into).collect::<Result<Vec<_>, _>>()?;
                                let item: Vec<#term_ty> = item.into_iter().map(Into::into).collect();
                                self.#fname = item.into();
                                Ok(self)
                            }
                        };

                        let doc_line = to_doc(&format!("Get `{}` as a vec of `&{}`s", fname, v_ty.to_token_stream()));
                        let get_many = quote! {
                            #doc_line
                            ///
                            /// This returns `None` if
                            /// - There is only one value present
                            ///
                            /// The returned vec will be empty if no values match the requested
                            /// type, but values are present.
                            pub fn #get_many_ident(&self) -> Option<Vec<&#v_ty>> {
                                match self.#fname {
                                    #ty::Array(ref array) => Some(array.iter().filter_map(|i| match i {
                                        #term_ty::#v_ty(item) => Some(item),
                                        _ => None,
                                    }).collect()),
                                    _ => None,
                                }
                            }
                        };

                        quote! {
                            #get
                            #set
                            #get_many
                            #set_many
                        }
                    } else {
                        let doc_line = to_doc(&format!("Set `{}` from a value that can be converted into `{}`", fname, v_ty.to_token_stream()));
                        let set = quote! {
                            #doc_line
                            pub fn #set_ident<T>(&mut self, item: T) -> Result<&mut Self, <T as std::convert::TryInto<#v_ty>>::Error>
                            where
                                T: std::convert::TryInto<#v_ty>,
                            {
                                use std::convert::TryInto;
                                let item: #v_ty = item.try_into()?;
                                let item: #term_ty = item.into();
                                self.#fname = Some(item.into());
                                Ok(self)
                            }
                        };

                        let doc_line = to_doc(&format!("Get `{}` as a `{}`", fname, v_ty.to_token_stream()));
                        let get = quote! {
                            #doc_line
                            ///
                            /// This returns `None` if
                            /// - There is no value present
                            /// - There is more than one value present
                            /// - The requested type is not stored type
                            pub fn #get_ident(&self) -> Option<&#v_ty> {
                                match self.#fname {
                                    Some(#ty::Term(#term_ty::#v_ty(ref term))) => Some(term),
                                    _ => None,
                                }
                            }
                        };

                        let doc_line = to_doc(&format!("Set `{}` from a vec of items that can be converted into `{}`s", fname, v_ty.to_token_stream()));
                        let set_many = quote! {
                            #doc_line
                            pub fn #set_many_ident<T>(&mut self, item: Vec<T>) -> Result<&mut Self, <T as std::convert::TryInto<#v_ty>>::Error>
                            where
                                T: std::convert::TryInto<#v_ty>,
                            {
                                let item: Vec<#v_ty> = item.into_iter().map(std::convert::TryInto::try_into).collect::<Result<Vec<_>, _>>()?;
                                let item: Vec<#term_ty> = item.into_iter().map(Into::into).collect();
                                self.#fname = Some(item.into());
                                Ok(self)
                            }
                        };

                        let doc_line = to_doc(&format!("Get `{}` as a slice of `{}`s", fname, term_ty.to_token_stream()));
                        let get_many = quote! {
                            #doc_line
                            ///
                            /// This returns `None` if
                            /// - There is no value present
                            /// - There is only one value present
                            pub fn #get_many_ident(&self) -> Option<Vec<&#v_ty>> {
                                match self.#fname {
                                    Some(#ty::Array(ref array)) => Some(array.iter().filter_map(|i| match i {
                                        #term_ty::#v_ty(item) => Some(item),
                                        _ => None,
                                    }).collect()),
                                    _ => None,
                                }
                            }
                        };

                        quote! {
                            #get
                            #set
                            #get_many
                            #set_many
                        }
                    }
                })
                .collect();

            let delete = if !field.description.required {
                let delete_ident =
                    Ident::new(&format!("delete_{}", fname), fname.span());

                let doc_line = to_doc(&format!("Set the `{}` field to `None`", fname));
                quote! {
                    #doc_line
                    pub fn #delete_ident(&mut self) -> &mut Self {
                        self.#fname = None;
                        self
                    }
                }
            } else {
                quote! {}
            };

            quote! {
                #tokens

                #delete
            }
        };

        Some(((field_tokens, fns), deps))
    }).unzip();

    let (field_tokens, fn_tokens): (proc_macro2::TokenStream, proc_macro2::TokenStream) =
        fields.into_iter().unzip();
    let deps_tokens: proc_macro2::TokenStream = deps.into_iter().filter_map(|d| d).collect();

    let q = quote! {
        #docs
        #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct #name {
            #field_tokens
        }

        impl #name {
            #fn_tokens
        }

        #deps_tokens
    };
    q.into()
}

mod kw {
    syn::custom_keyword!(types);
    syn::custom_keyword!(functional);
    syn::custom_keyword!(required);
    syn::custom_keyword!(rename);
    syn::custom_keyword!(alias);
    syn::custom_keyword!(docs);
}

struct Properties {
    name: Ident,
    docs: Vec<String>,
    fields: Punctuated<Field, Token![,]>,
}

struct Field {
    name: Ident,
    description: Description,
}

struct Description {
    docs: Vec<String>,
    types: Punctuated<Type, Token![,]>,
    functional: bool,
    required: bool,
    rename: Option<String>,
    aliases: Vec<String>,
}

impl Parse for Properties {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;

        let content;
        let _: token::Brace = braced!(content in input);

        let docs = parse_string_array::<_, kw::docs>(&&content, kw::docs)?;

        let fields = Punctuated::<Field, Token![,]>::parse_terminated(&content)?;

        Ok(Properties { name, docs, fields })
    }
}

impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;

        let content;
        let _: token::Brace = braced!(content in input);

        let description = content.parse()?;

        Ok(Field { name, description })
    }
}

impl Parse for Description {
    fn parse(input: ParseStream) -> Result<Self> {
        let docs = parse_string_array::<_, kw::docs>(&input, kw::docs)?;

        let lookahead = input.lookahead1();
        if !lookahead.peek(kw::types) {
            return Err(lookahead.error());
        }
        input.parse::<kw::types>()?;

        let content;
        let _: token::Bracket = bracketed!(content in input);
        let types = Punctuated::<Type, Token![,]>::parse_terminated(&content)?;
        optional_comma(&input)?;

        let functional = parse_kw::<_, kw::functional>(&input, kw::functional)?;
        let required = parse_kw::<_, kw::required>(&input, kw::required)?;
        let rename = parse_string_group::<_, kw::rename>(&input, kw::rename)?;
        let aliases = parse_string_array::<_, kw::alias>(&input, kw::alias)?;

        Ok(Description {
            docs,
            types,
            functional,
            required,
            rename,
            aliases,
        })
    }
}

fn parse_kw<T: Peek + Copy, U: Parse>(input: &ParseStream, t: T) -> Result<bool> {
    let lookahead = input.lookahead1();
    if lookahead.peek(t) {
        input.parse::<U>()?;
        optional_comma(&input)?;

        return Ok(true);
    }

    Ok(false)
}

fn parse_string_array<T: Peek + Copy, U: Parse>(input: &ParseStream, t: T) -> Result<Vec<String>> {
    let lookahead = input.lookahead1();
    if lookahead.peek(t) {
        input.parse::<U>()?;
        let content;
        bracketed!(content in input);

        let docs = Punctuated::<LitStr, Token![,]>::parse_terminated(&content)?;
        optional_comma(&input)?;
        Ok(docs.into_iter().map(|d| d.value()).collect())
    } else {
        Ok(vec![])
    }
}

fn parse_string_group<T: Peek + Copy, U: Parse>(
    input: &ParseStream,
    t: T,
) -> Result<Option<String>> {
    let lookahead = input.lookahead1();
    if lookahead.peek(t) {
        input.parse::<U>()?;
        let content;
        parenthesized!(content in input);
        let s: LitStr = content.parse()?;
        optional_comma(&input)?;

        return Ok(Some(s.value()));
    }

    Ok(None)
}

fn optional_comma(input: &ParseStream) -> Result<()> {
    let lookahead = input.lookahead1();
    if lookahead.peek(Token![,]) {
        input.parse::<Token![,]>()?;
    }
    Ok(())
}

fn camelize(s: &str) -> String {
    let (s, _) = s
        .chars()
        .fold((String::new(), true), |(mut acc, should_upper), c| {
            if c == '_' {
                (acc, true)
            } else {
                if should_upper {
                    acc += &c.to_uppercase().to_string();
                } else {
                    acc += &c.to_string();
                }

                (acc, false)
            }
        });

    s
}

fn snakize(s: &str) -> String {
    s.chars().fold(String::new(), |mut acc, c| {
        if c.is_uppercase() && !acc.is_empty() {
            acc += "_";
            acc += &c.to_lowercase().to_string();
        } else if c.is_uppercase() {
            acc += &c.to_lowercase().to_string();
        } else {
            acc += &c.to_string();
        }
        acc
    })
}
