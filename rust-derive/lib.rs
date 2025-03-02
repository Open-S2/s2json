use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, Fields, GenericArgument, Ident, PathArguments, Type, TypePath};

/// Derives the `Deserialize` trait for a struct to convert it to a `MValue`.
#[proc_macro_derive(Deserialize)]
pub fn mvalue_compatible_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
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
    let generated_mappings = generate_value_mappings(fields);

    let gen = quote! {
        impl s2json::MValueDeserialize for #name {
            fn to_mvalue(&self) -> s2json::MValue {
                #generated_mappings
            }
        }
    };

    gen.into()
}

// Function to generate all the field mappings from the struct fields
fn generate_value_mappings(fields: &Fields) -> proc_macro2::TokenStream {
    let field_mappings: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().expect("Unnamed fields not supported");
            let field_type = &field.ty;
            let field_conversion = generate_value_conversion(field_name, field_type, false); // Call the helper function for each field
            quote! {
                mvalue.insert(stringify!(#field_name).to_string(), #field_conversion);
            }
        })
        .collect();

    quote! {
        let mut mvalue = s2json::MValue::new();
        #(#field_mappings)*
        mvalue
    }
}

// Helper function to generate field-to-MValue conversion
fn generate_value_conversion(
    field_name: &Ident,
    field_type: &Type,
    is_option_input: bool,
) -> proc_macro2::TokenStream {
    // Check Primtives
    if is_primitive(field_type) {
        if is_option_input {
            generate_option_primitive_conversion(field_name, field_type)
        } else {
            generate_primitive_conversion(field_name, field_type)
        }
    } else if let Some(_inner_type) = is_option(field_type) {
        let is_some = generate_value_conversion(field_name, _inner_type, true);
        quote! {
            match self.#field_name {
                Some(value) => #is_some,
                None => s2json::ValueType::Primitive(s2json::PrimitiveValue::Null)
            }
        }
    } else if let Some(_inner_type) = is_vec(field_type) {
        // TODO: If inner type is primitive -> build a prim vector
        // TODO: If inner type is an option -> build an option vector
        // TODO: If inner type is a struct -> just run to_mvalue
        todo!("Vector not supported");
    } else {
        quote! { s2json::ValueType::Nested(self.#field_name.to_mvalue()) }
    }
}

fn generate_primitive_conversion(
    field_name: &Ident,
    field_type: &Type,
) -> proc_macro2::TokenStream {
    if is_type(field_type, "String") {
        quote! { s2json::ValueType::Primitive(s2json::PrimitiveValue::String(self.#field_name.clone())) }
    } else if is_type(field_type, "u8")
        || is_type(field_type, "u16")
        || is_type(field_type, "u32")
        || is_type(field_type, "u64")
    {
        quote! { s2json::ValueType::Primitive(s2json::PrimitiveValue::U64(self.#field_name.into())) }
    } else if is_type(field_type, "i8")
        || is_type(field_type, "i16")
        || is_type(field_type, "i32")
        || is_type(field_type, "i64")
    {
        quote! { s2json::ValueType::Primitive(s2json::PrimitiveValue::I64(self.#field_name.into())) }
    } else if is_type(field_type, "f32") || is_type(field_type, "f64") {
        quote! { s2json::ValueType::Primitive(s2json::PrimitiveValue::F64(self.#field_name.into())) }
    } else if is_type(field_type, "bool") {
        quote! { s2json::ValueType::Primitive(s2json::PrimitiveValue::Bool(self.#field_name)) }
    } else {
        quote! { s2json::ValueType::Primitive(s2json::PrimitiveValue::Null) }
    }
}

fn generate_option_primitive_conversion(
    field_name: &Ident,
    field_type: &Type,
) -> proc_macro2::TokenStream {
    if is_type(field_type, "String") {
        quote! { s2json::ValueType::Primitive(s2json::PrimitiveValue::String(self.#field_name.unwrap_or_default().clone())) }
    } else if is_type(field_type, "u8")
        || is_type(field_type, "u16")
        || is_type(field_type, "u32")
        || is_type(field_type, "u64")
    {
        quote! { s2json::ValueType::Primitive(s2json::PrimitiveValue::U64(self.#field_name.unwrap_or_default().into())) }
    } else if is_type(field_type, "i8")
        || is_type(field_type, "i16")
        || is_type(field_type, "i32")
        || is_type(field_type, "i64")
    {
        quote! { s2json::ValueType::Primitive(s2json::PrimitiveValue::I64(self.#field_name.unwrap_or_default().into())) }
    } else if is_type(field_type, "f32") || is_type(field_type, "f64") {
        quote! { s2json::ValueType::Primitive(s2json::PrimitiveValue::F64(self.#field_name.unwrap_or_default().into())) }
    } else if is_type(field_type, "bool") {
        quote! { s2json::ValueType::Primitive(s2json::PrimitiveValue::Bool(self.#field_name.unwrap_or_default())) }
    } else {
        quote! { s2json::ValueType::Primitive(s2json::PrimitiveValue::Null) }
    }
}

// Function to check if a type matches a given identifier (e.g., "String", "u32")
fn is_type(field_type: &Type, expected: &str) -> bool {
    if let Type::Path(TypePath { path, .. }) = field_type {
        path.segments.last().map_or(false, |seg| seg.ident == expected)
    } else {
        false
    }
}

fn is_primitive(field_type: &Type) -> bool {
    is_type(field_type, "String")
        || is_type(field_type, "u8")
        || is_type(field_type, "u16")
        || is_type(field_type, "u32")
        || is_type(field_type, "u64")
        || is_type(field_type, "i8")
        || is_type(field_type, "i16")
        || is_type(field_type, "i32")
        || is_type(field_type, "i64")
        || is_type(field_type, "f16")
        || is_type(field_type, "f32")
        || is_type(field_type, "f64")
        || is_type(field_type, "bool")
}

// // Function to check if a type is a Vec
// fn is_vec(field_type: &Type) -> bool {
//     if let Type::Path(TypePath { path, .. }) = field_type {
//         path.segments.last().map_or(false, |seg| seg.ident == "Vec")
//     } else {
//         false
//     }
// }

fn is_vec(field_type: &Type) -> Option<&Type> {
    if let Type::Path(TypePath { path, .. }) = field_type {
        if path.segments.len() == 1 && path.segments[0].ident == "Vec" {
            if let PathArguments::AngleBracketed(args) = &path.segments[0].arguments {
                if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                    return Some(inner_type); // Return inner type of Vec<T>
                }
            }
        }
    }
    None
}

// Function to check if a type is an Option<T>
fn is_option(field_type: &Type) -> Option<&Type> {
    if let Type::Path(TypePath { path, .. }) = field_type {
        if path.segments.len() == 1 && path.segments[0].ident == "Option" {
            // Get the inner type of Option<T>
            if let PathArguments::AngleBracketed(args) = &path.segments[0].arguments {
                if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                    return Some(inner_type);
                }
            }
        }
    }
    None
}

// fn generate_array_conversion(fields: &Fields) -> proc_macro2::TokenStream {
//     let field_mappings: Vec<_> = fields
//         .iter()
//         .map(|field| {
//             let field_conversion = generate_value_primitive_conversion(field); // Call the helper function for each field
//             quote! {
//                 mvalue.insert(#field_conversion);
//             }
//         })
//         .collect();

//     quote! {
//         let mut arr = Vec::<ValuePrimitiveType>::new();
//         #(#field_mappings)*
//         s2json::ValueType::Array(arr)
//     }
// }

// fn generate_value_primitive_conversion(field: &Field) -> proc_macro2::TokenStream {
//     let field_name = field.ident.as_ref().expect("Unnamed fields not supported");
//     let field_type = &field.ty;

//     // TODO: Nested Primitive
//     if is_primitive(field_type) {
//         generate_primitive_conversion(field_name, field_type)
//     } else if let Some(inner_type) = is_option(field_type) {
//         quote! { s2json::ValueType::Primitive(s2json::PrimitiveValue::Null) }
//     } else {
//         panic!("Unsupported Vector field type");
//     }
// }
