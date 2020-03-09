/*
 * This file is part of ActivityStreams Derive.
 *
 * Copyright © 2018 Riley Trautman
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
//! activitystreams-derive = "3.0"
//! serde = { version = "1.0", features = ["derive"] }
//! ```
//!
//! ```rust
//! use activitystreams_derive::{properties, UnitString};
//! use serde_json::Value;
//!
//! /// Using the UnitString derive macro
//! ///
//! /// This macro implements Serialize and Deserialize for the given type, making this type
//! /// represent the string "SomeKind" in JSON.
//! #[derive(Clone, Debug, Default, UnitString)]
//! #[activitystreams(SomeKind)]
//! pub struct MyKind;
//!
//! /// Using the properties macro
//! ///
//! /// This macro generates getters and setters for the associated fields.
//! properties! {
//!     My {
//!         context {
//!             types [
//!                 Value,
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
//!         },
//!     }
//! }
//! #
//! # fn main () -> Result<(), Box<dyn std::error::Error> {
//! let s = r#"{
//!     "@context": "http://www.w3c.org/ns#activitystreams",
//!     "type": "SomeKind",
//!     "required_key": {
//!         "key": "value"
//!     }
//! }"#
//!
//! let m: MyProperties = serde_json::from_str(s)?;
//! println!("{:?}", m.get_kind());
//! # Ok(())
//! # }
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

use std::iter::FromIterator;

#[proc_macro_derive(PropRefs, attributes(activitystreams))]
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

    let impls = fields
        .named
        .iter()
        .filter_map(|field| {
            let our_attr = field.attrs.iter().find(|attribute| {
                attribute
                    .path
                    .segments
                    .last()
                    .map(|segment| {
                        segment.ident == Ident::new("activitystreams", segment.ident.span())
                    })
                    .unwrap_or(false)
            });

            our_attr.map(move |our_attr| {
                (
                    field.ident.clone().unwrap(),
                    field.ty.clone(),
                    our_attr.clone(),
                )
            })
        })
        .flat_map(move |(ident, ty, attr)| {
            let object = from_value(attr);
            let name = name.clone();
            let ext_trait = Ident::new(&format!("{}Ext", object), name.span());

            let activity_impls = quote! {
                #[typetag::serde]
                impl #object for #name {
                    fn as_any(&self) -> &std::any::Any {
                        self
                    }

                    fn as_any_mut(&self) -> &mut std::any::Any {
                        self
                    }
                }

                impl #ext_trait for #name {
                    fn props(&self) -> &#ty {
                        self.as_ref()
                    }

                    fn props_mut(&mut self) -> &mut #ty {
                        self.as_mut()
                    }
                }
            };

            let ref_impls = quote! {
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
            };

            if object == "None" {
                ref_impls
            } else {
                quote! {
                    #ref_impls
                    #activity_impls
                }
            }
        });

    let tokens = proc_macro2::TokenStream::from_iter(impls);

    let full = quote! {
        #tokens
    };

    full.into()
}

#[proc_macro_derive(UnitString, attributes(activitystreams))]
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
                .map(|segment| segment.ident == Ident::new("activitystreams", segment.ident.span()))
                .unwrap_or(false)
        })
        .unwrap()
        .clone();

    let visitor_name = from_value(attr);
    let value = format!("\"{}\"", visitor_name);

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

    let c = quote! {
        #serialize
        #visitor
        #deserialize
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

#[proc_macro]
pub fn properties(tokens: TokenStream) -> TokenStream {
    let Properties { name, fields } = parse_macro_input!(tokens as Properties);

    let name = Ident::new(&format!("{}Properties", name), name.span());

    let (fields, deps): (Vec<_>, Vec<_>) = fields.iter().filter_map(|field| {
        if field.description.types.is_empty() {
            return None;
        }

        let fname = field.name.clone();
        let (ty, deps) = if field.description.types.len() == 1 {
            let ty = Ident::new(&field.description.types.first().unwrap().to_token_stream().to_string(), fname.span());
            if field.description.functional {
                (ty, None)
            } else {
                let enum_ty = Ident::new(&camelize(&format!("{}_{}_enum", name, fname)), fname.span());
                let deps = quote! {
                    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
                    #[serde(rename_all = "camelCase")]
                    #[serde(untagged)]
                    pub enum #enum_ty {
                        Term(#ty),
                        Array(Vec<#ty>),
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

            let v_tys: Vec<_> = field
                .description
                .types
                .iter()
                .map(|v_ty| {
                    quote! {
                        #v_ty(#v_ty),
                    }
                })
                .collect();

            let v_tokens = proc_macro2::TokenStream::from_iter(v_tys);

            let deps = if !field.description.functional {
                let term_ty = Ident::new(&camelize(&format!("{}_{}_term_enum", name, fname)), fname.span());

                let froms: Vec<_> = field
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

                let from_tokens = proc_macro2::TokenStream::from_iter(froms);

                quote! {
                    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
                    #[serde(rename_all = "camelCase")]
                    #[serde(untagged)]
                    pub enum #term_ty {
                        #v_tokens
                    }

                    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
                    #[serde(rename_all = "camelCase")]
                    #[serde(untagged)]
                    pub enum #ty {
                        Term(#term_ty),
                        Array(Vec<#term_ty>),
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
                let froms: Vec<_> = field
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

                let from_tokens = proc_macro2::TokenStream::from_iter(froms);

                quote! {
                    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
                    #[serde(rename_all = "camelCase")]
                    #[serde(untagged)]
                    pub enum #ty {
                        #v_tokens
                    }

                    #from_tokens
                }
            };

            (ty, Some(deps))
        };

        let field_tokens = if field.description.required {
            if let Some(ref rename) = field.description.rename {
                quote! {
                    #[serde(rename = #rename)]
                    pub #fname: #ty,
                }
            } else {
                quote! {
                    pub #fname: #ty,
                }
            }
        } else {
            if let Some(ref rename) = field.description.rename {
                quote! {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    #[serde(rename = #rename)]
                    pub #fname: Option<#ty>,
                }
            } else {
                quote! {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub #fname: Option<#ty>,
                }
            }
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
                    let set = quote! {
                        pub fn #set_ident<T>(&mut self, item: T) -> &mut Self
                        where
                            T: Into<#v_ty>
                        {
                            self.#fname = item.into();
                            self
                        }
                    };

                    let get = quote! {
                        pub fn #get_ident(&self) -> &#v_ty {
                            &self.#fname
                        }
                    };

                    quote!{
                        #get
                        #set
                    }
                } else {
                    let set = quote! {
                        pub fn #set_ident<T>(&mut self, item: T) -> &mut Self
                        where
                            T: Into<#v_ty>
                        {
                            self.#fname = #enum_ty::Term(item.into());
                            self
                        }
                    };

                    let get = quote! {
                        pub fn #get_ident(&self) -> Option<&#v_ty> {
                            match self.#fname {
                                #enum_ty::Term(ref term) => Some(term),
                                _ => None,
                            }
                        }
                    };

                    let set_many = quote! {
                        pub fn #set_many_ident<T>(&mut self, item: Vec<T>) -> &mut Self
                        where
                            T: Into<#v_ty>,
                        {
                            let item: Vec<#v_ty> = item.into_iter().map(Into::into).collect();
                            self.#fname = #enum_ty::Array(item);
                            self
                        }
                    };

                    let get_many = quote! {
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
                    let set = quote! {
                        pub fn #set_ident<T>(&mut self, item: T) -> &mut Self
                        where
                            T: Into<#v_ty>
                        {
                            self.#fname = Some(item.into());
                            self
                        }
                    };

                    let get = quote! {
                        pub fn #get_ident(&self) -> Option<&#v_ty> {
                            self.#fname.as_ref()
                        }
                    };

                    quote!{
                        #get
                        #set
                    }
                } else {
                    let set = quote! {
                        pub fn #set_ident<T>(&mut self, item: T) -> &mut Self
                        where
                            T: Into<#v_ty>
                        {
                            self.#fname = Some(#enum_ty::Term(item.into()));
                            self
                        }
                    };

                    let get = quote! {
                        pub fn #get_ident(&self) -> Option<&#v_ty> {
                            match self.#fname {
                                Some(#enum_ty::Term(ref term)) => Some(term),
                                _ => None,
                            }
                        }
                    };

                    let set_many = quote! {
                        pub fn #set_many_ident<T>(&mut self, item: Vec<T>) -> &mut Self
                        where
                            T: Into<#v_ty>,
                        {
                            let item: Vec<#v_ty> = item.into_iter().map(Into::into).collect();
                            self.#fname = Some(#enum_ty::Array(item));
                            self
                        }
                    };

                    let get_many = quote! {
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
            let impls: Vec<_> = field
                .description
                .types
                .iter()
                .map(|v_ty| {
                    let set_ident =
                        Ident::new(&format!("set_{}_{}", fname, snakize(&v_ty.to_token_stream().to_string())), fname.span());
                    let get_ident =
                        Ident::new(&format!("get_{}_{}", fname, snakize(&v_ty.to_token_stream().to_string())), fname.span());

                    if field.description.required {
                        let set = quote! {
                            pub fn #set_ident<T>(&mut self, item: T) -> &mut Self
                            where
                                T: Into<#v_ty>,
                            {
                                let item: #v_ty = item.into();
                                self.#fname = item.into();
                                self
                            }
                        };

                        let get = quote! {
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
                        let set = quote! {
                            pub fn #set_ident<T>(&mut self, item: T) -> &mut Self
                            where
                                T: Into<#v_ty>,
                            {
                                let item: #v_ty = item.into();
                                self.#fname = Some(item.into());
                                self
                            }
                        };

                        let get = quote! {
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

            let tokens = proc_macro2::TokenStream::from_iter(impls);

            quote! {
                #tokens
            }
        } else {
            let term_ty = Ident::new(&camelize(&format!("{}_{}_term_enum", name, fname)), fname.span());
            let impls: Vec<_> = field
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
                        let set = quote! {
                            pub fn #set_ident<T>(&mut self, item: T) -> &mut Self
                            where
                                T: Into<#v_ty>,
                            {
                                let item: #v_ty = item.into();
                                let item: #term_ty = item.into();
                                self.#fname = item.into();
                                self
                            }
                        };

                        let get = quote! {
                            pub fn #get_ident(&self) -> Option<&#v_ty> {
                                match self.#fname {
                                    #ty::Term(#term_ty::#v_ty(ref term)) => Some(term),
                                    _ => None,
                                }
                            }
                        };

                        let set_many = quote! {
                            pub fn #set_many_ident<T>(&mut self, item: Vec<T>) -> &mut Self
                            where
                                T: Into<#v_ty>,
                            {
                                let item: Vec<#v_ty> = item.into_iter().map(Into::into).collect();
                                let item: Vec<#term_ty> = item.into_iter().map(Into::into).collect();
                                self.#fname = item.into();
                                self
                            }
                        };

                        let get_many = quote! {
                            pub fn #get_many_ident(&self) -> Option<&[#term_ty]> {
                                match self.#fname {
                                    #ty::Array(ref array) => Some(array),
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
                        let set = quote! {
                            pub fn #set_ident<T>(&mut self, item: T) -> &mut Self
                            where
                                T: Into<#v_ty>,
                            {
                                let item: #v_ty = item.into();
                                let item: #term_ty = item.into();
                                self.#fname = Some(item.into());
                                self
                            }
                        };

                        let get = quote! {
                            pub fn #get_ident(&self) -> Option<&#v_ty> {
                                match self.#fname {
                                    Some(#ty::Term(#term_ty::#v_ty(ref term))) => Some(term),
                                    _ => None,
                                }
                            }
                        };

                        let set_many = quote! {
                            pub fn #set_many_ident<T>(&mut self, item: Vec<T>) -> &mut Self
                            where
                                T: Into<#v_ty>,
                            {
                                let item: Vec<#v_ty> = item.into_iter().map(Into::into).collect();
                                let item: Vec<#term_ty> = item.into_iter().map(Into::into).collect();
                                self.#fname = Some(item.into());
                                self
                            }
                        };

                        let get_many = quote! {
                            pub fn #get_many_ident(&self) -> Option<&[#term_ty]> {
                                match self.#fname {
                                    Some(#ty::Array(ref array)) => Some(array),
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

            let tokens = proc_macro2::TokenStream::from_iter(impls);

            let delete = if !field.description.required {
                let delete_ident =
                    Ident::new(&format!("delete_{}", fname), fname.span());

                quote! {
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

    let (fields, fns): (Vec<_>, Vec<_>) = fields.into_iter().unzip();
    let deps: Vec<_> = deps.into_iter().filter_map(|d| d).collect();

    let field_tokens = proc_macro2::TokenStream::from_iter(fields);
    let fn_tokens = proc_macro2::TokenStream::from_iter(fns);
    let deps_tokens = proc_macro2::TokenStream::from_iter(deps);

    let q = quote! {
        #deps_tokens

        #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct #name {
            #field_tokens
        }

        impl #name {
            #fn_tokens
        }
    };
    q.into()
}

mod kw {
    syn::custom_keyword!(types);
    syn::custom_keyword!(functional);
    syn::custom_keyword!(required);
    syn::custom_keyword!(rename);
}

struct Properties {
    name: Ident,
    fields: Punctuated<Field, Token![,]>,
}

struct Field {
    name: Ident,
    description: Description,
}

struct Description {
    types: Punctuated<Type, Token![,]>,
    functional: bool,
    required: bool,
    rename: Option<String>,
}

impl Parse for Properties {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;

        let content;
        let _: token::Brace = braced!(content in input);
        let fields = Punctuated::<Field, Token![,]>::parse_terminated(&content)?;

        Ok(Properties { name, fields })
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
        let rename = parse_rename::<_, kw::rename>(&input, kw::rename)?;

        Ok(Description {
            types,
            functional,
            required,
            rename,
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

fn parse_rename<T: Peek + Copy, U: Parse>(input: &ParseStream, t: T) -> Result<Option<String>> {
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
