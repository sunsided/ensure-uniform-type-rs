//! # `#[ensure_uniform_type]`: Ensure uniform struct field types at compile-time
//!
//! A compile-time check to ensure that a type uses uniform types across its fields.
//!
//! An example use for this macro is to ensure that a struct `#[repr(C)]` layout can
//! be correctly mapped onto a slice of the (uniform) field type.
//!
//! ## Note
//!
//! The type check is currently name-based.
//!
//! ## Examples
//!
//! Assume the following type:
//!
//! ```compile_fail
//! #[ensure_uniform_type::ensure_uniform_type]
//! pub struct Example<T>
//! {
//!     /// First field
//!     x: T,
//!
//!     // Different type
//!     offending: u32,
//! }
//! ```
//!
//! The above would fail to compile, instead giving the error:
//!
//! ```plain
//! error: Struct DifferentialDriveState has fields of different types. Expected uniform use of T, found u32 in field lol.
//! --> src/differential_drive.rs:16:1
//! |
//! 16 | / /// A state of a differential drive robot, or differential wheeled robot.
//! 18 | | #[ensure_uniform_type]
//! 19 | | pub struct Example<T>
//! ...  |
//! 37 | |     offending: u32,
//! 38 | | }
//! | |_^
//! ```
//!
//! By contrast, the following would compile without an error:
//!
//! ```
//! #[ensure_uniform_type::ensure_uniform_type]
//! pub struct Example<T>
//! {
//!     x: T,
//!     not_offending: T,
//! }
//! ```

#![deny(unsafe_code)]

extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, ItemStruct};

/// # Ensure uniform field types
///
/// A compile-time check to ensure that a type uses uniform types across its fields.
///
/// ## Note
///
/// The type check is currently name-based.
///
/// ## Examples
///
/// The above would fail to compile, instead giving the error:
///
/// ```compile_fail
/// #[ensure_uniform_type::ensure_uniform_type]
/// pub struct Example<T>
/// {
///     /// First field
///     x: T,
///
/// // Different type
///     offending: u32,
/// }
/// ```
///
/// The above would fail with the error:
///
/// ```plain
/// error: Struct DifferentialDriveState has fields of different types. Expected uniform use of T, found u32 in field lol.
/// --> src/differential_drive.rs:16:1
/// |
/// 16 | / /// A state of a differential drive robot, or differential wheeled robot.
/// 18 | | #[ensure_uniform_type]
/// 19 | | pub struct Example<T>
/// ...  |
/// 37 | |     offending: u32,
/// 38 | | }
/// | |_^
/// ```
///
/// By contrast, the following would compile without an error:
///
/// ```
/// #[ensure_uniform_type::ensure_uniform_type]
/// pub struct Example<T>
/// {
///     x: T,
///     not_offending: T,
/// }
/// ```
#[proc_macro_attribute]
pub fn ensure_uniform_type(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let struct_name = &input.ident;
    let fields = if let syn::Fields::Named(fields) = &input.fields {
        &fields.named
    } else {
        unimplemented!("Only named fields are supported")
    };

    // Assume the first field type is the required uniform size type
    let first_field_type = &fields.first().unwrap().ty;

    // HACK: We cannot compare syn::Type instances directly, so we instead compare them by name.
    let first_field_type = quote!(#first_field_type).to_string();

    for field in fields {
        let field_name = field.ident.as_ref().expect("expected named field");
        let field_type = &field.ty;
        let field_type = quote!(#field_type).to_string();

        if first_field_type != field_type {
            let error_message = format!(
                "Struct {} has fields of different types. Expected uniform use of {}, found {} in field {}.",
                struct_name,
                first_field_type,
                field_type,
                field_name
            );
            return syn::Error::new_spanned(input, error_message)
                .to_compile_error()
                .into();
        }
    }
    TokenStream::from(quote! {
        #input
    })
}
