use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{Data, Fields, Ident};

/// Derives the `MValueCompatible` trait for a struct to convert it to a `MValue`.
#[proc_macro_derive(MValueCompatible)]
pub fn mvalue_compatible_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    generate_to_mvalue(&ast)
}

fn generate_to_mvalue(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;

    let crate_name = match crate_name("s2json") {
        Ok(FoundCrate::Itself) => "s2json".to_string(),
        Ok(FoundCrate::Name(name)) => name,
        Err(_) => "s2json_core".to_string(), // Fallback if resolution fails (happens for testing)
    };
    let s2json_core = Ident::new(&crate_name, Span::call_site());

    let fields = match data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => panic!("Unsupported data type"),
    };

    let (from_mvalue, into_mvalue) = generate_conversions(fields);

    let gen = quote! {
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate #s2json_core as _s2json_core;
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate alloc;
            // This may be necessary if the struct is using MValue as a variable for instance
            use alloc::string::ToString;
            use _s2json_core::*;

            /// Starting from an MValue, convert to a struct
            #[automatically_derived]
            impl From<MValue> for #name {
                fn from(m: MValue) -> Self {
                    #from_mvalue
                }
            }
            /// Starting from a ref to an MValue, convert to a struct
            #[automatically_derived]
            impl From<&MValue> for #name {
                fn from(m: &MValue) -> Self {
                    #from_mvalue
                }
            }

            /// If this struct is nested into another struct, pull out the MValue and let
            /// From<MValue> handle
            #[automatically_derived]
            impl From<ValueType> for #name {
                fn from(value: ValueType) -> Self {
                    match value {
                        ValueType::Nested(v) => v.into(),
                        _ => #name::default(),
                    }
                }
            }
            /// If this struct ref is nested into another struct, pull out the MValue and let
            /// From<MValue> handle
            #[automatically_derived]
            impl From<&ValueType> for #name {
                fn from(value: &ValueType) -> Self {
                    match value {
                        ValueType::Nested(v) => v.into(),
                        _ => #name::default(),
                    }
                }
            }
            /// If this struct is nested into another struct, convert to a ValueType that's nested
            #[automatically_derived]
            impl From<#name> for ValueType {
                fn from(value: #name) -> ValueType {
                    ValueType::Nested(value.into())
                }
            }

            /// Starting from a struct, convert to an MValue
            #[automatically_derived]
            impl From<#name> for MValue {
                fn from(value: #name) -> MValue {
                    #into_mvalue
                }
            }

            /// Finally implement the MValueCompatible trait
            #[automatically_derived]
            impl MValueCompatible for #name {}
        };
    };

    gen.into()
}

fn generate_conversions(fields: &Fields) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut from_assignments = vec![];
    let mut into_insertions = vec![];

    for field in fields.iter() {
        let field_name = field.ident.as_ref().unwrap();
        let field_str = field_name.to_string();
        let field_ty = &field.ty;
        // Option needs to be handled manually for some from cases.
        // The rest can be handled with core provided into/from
        let is_option = if let syn::Type::Path(type_path) = field_ty {
            type_path.path.segments.last().unwrap().ident == "Option"
        } else {
            false
        };
        if is_option {
            from_assignments.push(quote! {
                #field_name: m.get(#field_str)
                    .map(|v| match v {
                        ValueType::Primitive(PrimitiveValue::Null) => None,
                        other => Some(other.into()),
                    })
                    .unwrap_or(None)
            });
        } else {
            from_assignments.push(quote! {
                #field_name: m.get(#field_str).map(Into::into).unwrap_or_default()
            });
        }

        // into insertions work for all cases
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
