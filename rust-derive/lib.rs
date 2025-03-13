extern crate s2json_core;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, Fields};

/// Derives the `MValueCompatible` trait for a struct to convert it to a `MValue`.
#[proc_macro_derive(MValueCompatible)]
pub fn mvalue_compatible_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree that we can manipulate
    let ast = syn::parse(input).unwrap();
    // Build the trait implementation
    generate_to_mvalue(&ast)
}

fn generate_to_mvalue(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;

    let fields = match data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => panic!("Unsupported data type"),
    };

    let (from_mvalue, into_mvalue) = generate_conversions(fields);

    let gen = quote! {
        /// Starting from an MValue, convert to a struct
        impl From<MValue> for #name {
            fn from(mut m: MValue) -> Self {
                #from_mvalue
            }
        }
        /// If this struct is nested into another struct, pull out the MValue and let From<MValue> handle
        impl From<ValueType> for #name {
            fn from(value: ValueType) -> Self {
                match value {
                    ValueType::Nested(v) => v.into(),
                    _ => #name::default(),
                }
            }
        }
        /// Starting from a struct, convert to an MValue
        impl From<#name> for MValue {
            fn from(value: #name) -> MValue {
                #into_mvalue
            }
        }
        /// If this struct is nested into another struct, convert to a ValueType that's nested
        impl From<#name> for ValueType {
            fn from(value: #name) -> ValueType {
                ValueType::Nested(value.into())
            }
        }
    };

    gen.into()
}

fn generate_conversions(fields: &Fields) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut from_assignments = vec![];
    let mut into_insertions = vec![];

    for field in fields.iter() {
        let field_name = field.ident.as_ref().unwrap();
        let field_str = field_name.to_string();

        from_assignments.push(quote! {
            #field_name: m.remove(#field_str).map(Into::into).unwrap_or_default()
        });

        into_insertions.push(quote! {
            map.insert(#field_str.to_string(), value.#field_name.into());
        });
    }

    let from_mvalue = quote! {
        Self {
            #(#from_assignments),*
        }
    };

    let into_mvalue = quote! {
        let mut map = MValue::new();
        #(#into_insertions)*
        map
    };

    (from_mvalue, into_mvalue)
}
