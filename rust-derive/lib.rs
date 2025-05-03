#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! The `s2json-derive` Rust crate provides ... TODO

mod json;
mod mvalue;
mod prim_value;

use json::generate_to_json;
use mvalue::generate_to_mvalue;
use prim_value::generate_to_value_prim;
use proc_macro::TokenStream;

/// Derives the `MValueCompatible` trait for a struct to convert it to a `MValue`.
#[proc_macro_derive(MValueCompatible)]
pub fn mvalue_compatible_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    generate_to_mvalue(&ast)
}

/// Derives the `Properties` trait for a struct to convert it to a `Properties`.
#[proc_macro_derive(Properties)]
pub fn properties_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    generate_to_mvalue(&ast)
}

/// Derives the `MValue` trait for a struct to convert it to a `MValue`.
#[proc_macro_derive(MValue)]
pub fn mvalue_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    generate_to_mvalue(&ast)
}

/// Derives the `JSONProperties` trait for a struct to convert it to a `JSONProperties`.
#[proc_macro_derive(JSONProperties)]
pub fn json_properties_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    generate_to_json(&ast)
}

/// Derives the `ValuePrimitive` trait for a struct to convert it to a `ValuePrimitive`.
#[proc_macro_derive(ValuePrimitive)]
pub fn primitive_value_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    generate_to_value_prim(&ast)
}
