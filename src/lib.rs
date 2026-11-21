#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

use proc_macro::TokenStream;

use quote::{ToTokens, quote};
use syn::ItemStruct;

/// # Ensure uniform field types
///
/// A compile-time check to ensure that a struct uses the same type for all of its fields.
/// Works on structs with named fields and on tuple structs; unit and empty structs
/// are trivially uniform. See the [crate documentation](crate) for examples.
///
/// ## Note
///
/// The type check is currently name-based.
#[proc_macro_attribute]
pub fn ensure_uniform_type(_attr: TokenStream, item: TokenStream) -> TokenStream {
    expand(item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Checks that all fields of the struct in `item` share one type and returns the struct unchanged.
///
/// Uses `proc_macro2` so the logic can run outside the compiler, in unit and property tests.
fn expand(item: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let input: ItemStruct = syn::parse2(item)?;

    if let Some(error) = first_mismatch(&input) {
        return Err(error);
    }

    Ok(input.into_token_stream())
}

/// Returns an error for the first field whose type differs from the first field's type.
///
/// `None` means all fields share one type; structs without fields are trivially uniform.
fn first_mismatch(input: &ItemStruct) -> Option<syn::Error> {
    let mut fields = input.fields.iter().enumerate();
    let (_, first) = fields.next()?;
    let expected = type_name(&first.ty);

    fields.find_map(|(index, field)| {
        let found = type_name(&field.ty);
        (found != expected).then(|| mismatch_error(input, index, field, &expected, &found))
    })
}

/// Prints a type as tokens, which is what the uniformity check compares.
///
/// Name-based: `syn::Type` has no semantic equality, so `f32` and `std::primitive::f32`
/// print, and therefore compare, differently.
fn type_name(ty: &syn::Type) -> String {
    quote!(#ty).to_string()
}

/// Builds the compile error for `field`, spanned on its type so the compiler points at it.
///
/// `index` is the field's position, used as its name in tuple structs.
fn mismatch_error(
    input: &ItemStruct,
    index: usize,
    field: &syn::Field,
    expected: &str,
    found: &str,
) -> syn::Error {
    let field_name = field
        .ident
        .as_ref()
        .map_or_else(|| index.to_string(), ToString::to_string);
    let message = format!(
        "Struct {} has fields of different types. Expected uniform use of {expected}, found {found} in field {field_name}.",
        input.ident
    );

    syn::Error::new_spanned(&field.ty, message)
}

/// Property tests for [`expand`]; the compile-level behavior is covered by `tests/ui.rs`.
#[cfg(test)]
mod tests {
    use super::expand;
    use proptest::prelude::*;
    use quote::ToTokens;

    /// Field types the generated structs pick from; spelled the way `quote` prints them.
    const TYPES: &[&str] = &[
        "f32",
        "u8",
        "T",
        "Option < T >",
        "Vec < u8 >",
        "[f32 ; 3]",
        "& 'a str",
        "std :: primitive :: f32",
    ];

    /// Builds a named or tuple struct with the given field types.
    fn make_struct(types: &[&str], tuple: bool) -> String {
        if tuple {
            format!("pub struct S<'a, T>({});", types.join(", "))
        } else {
            let fields: Vec<String> = types
                .iter()
                .enumerate()
                .map(|(i, ty)| format!("f{i}: {ty}"))
                .collect();
            format!("pub struct S<'a, T> {{ {} }}", fields.join(", "))
        }
    }

    proptest! {
        /// The macro accepts a struct exactly when all its field types are spelled the same,
        /// and then returns the struct unchanged.
        #[test]
        fn accepts_iff_uniform(
            types in prop::collection::vec(prop::sample::select(TYPES), 0..6),
            tuple in any::<bool>(),
        ) {
            let source = make_struct(&types, tuple);
            let item: proc_macro2::TokenStream = source.parse().unwrap();
            let uniform = types.windows(2).all(|w| w[0] == w[1]);

            match expand(item.clone()) {
                Ok(output) => {
                    prop_assert!(uniform, "accepted non-uniform struct: {}", source);
                    // Compare after a parse round-trip: raw token printing differs in punctuation spacing.
                    let input = syn::parse2::<syn::ItemStruct>(item).unwrap().into_token_stream();
                    prop_assert_eq!(output.to_string(), input.to_string());
                }
                Err(error) => {
                    prop_assert!(!uniform, "rejected uniform struct: {}", source);
                    let first_mismatch = types.iter().position(|t| *t != types[0]).unwrap();
                    let field = if tuple { first_mismatch.to_string() } else { format!("f{first_mismatch}") };
                    prop_assert!(
                        error.to_string().ends_with(&format!("in field {field}.")),
                        "wrong field in error: {}", error
                    );
                }
            }
        }
    }

    /// Non-struct input is rejected with a parse error instead of a panic.
    #[test]
    fn rejects_enum() {
        let item: proc_macro2::TokenStream = "pub enum E { A }".parse().unwrap();
        assert!(expand(item).is_err());
    }
}
