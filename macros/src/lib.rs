#[allow(unused_extern_crates)]
extern crate proc_macro;
mod utils;

use crate::utils::IteratorExt as _;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    parse_quote, spanned::Spanned, Error, Field, Fields, ItemEnum, ItemStruct, Meta, NestedMeta,
};

/// Apply function on every field of structs or enums
fn apply_function_to_struct_and_enum_fields<F>(
    input: TokenStream,
    function: F,
) -> Result<TokenStream2, Error>
where
    F: Copy,
    F: Fn(&mut Field) -> Result<(), String>,
{
    /// Handle a single struct or a single enum variant
    fn apply_on_fields<F>(fields: &mut Fields, function: F) -> Result<(), Error>
    where
        F: Fn(&mut Field) -> Result<(), String>,
    {
        match fields {
            // simple, no fields, do nothing
            Fields::Unit => Ok(()),
            Fields::Named(ref mut fields) => fields
                .named
                .iter_mut()
                .map(|field| function(field).map_err(|err| Error::new(field.span(), err)))
                .collect_error(),
            Fields::Unnamed(ref mut fields) => fields
                .unnamed
                .iter_mut()
                .map(|field| function(field).map_err(|err| Error::new(field.span(), err)))
                .collect_error(),
        }
    }

    // For each field in the struct given by `input`, add the `skip_serializing_if` attribute,
    // if and only if, it is of type `Option`
    if let Ok(mut input) = syn::parse::<ItemStruct>(input.clone()) {
        apply_on_fields(&mut input.fields, function)?;
        Ok(quote!(#input))
    } else if let Ok(mut input) = syn::parse::<ItemEnum>(input) {
        input
            .variants
            .iter_mut()
            .map(|variant| apply_on_fields(&mut variant.fields, function))
            .collect_error()?;
        Ok(quote!(#input))
    } else {
        Err(Error::new(
            Span::call_site(),
            "The attribute can only be applied to struct or enum definitions.",
        ))
    }
}

/// Add the skip_serializing_if annotation to each field of the struct
fn skip_serializing_default_add_attr_to_field(field: &mut Field) -> Result<(), String> {
    let has_skip_serializing_if = field_has_attribute(field, "serde", "skip_serializing_if");

    // Remove the `serialize_always` attribute
    let mut has_always_attr = false;
    field.attrs.retain(|attr| {
        let has_attr = attr.path.is_ident("serialize_always");
        has_always_attr |= has_attr;
        !has_attr
    });

    // Error on conflicting attributes
    if has_always_attr && has_skip_serializing_if {
        let mut msg = r#"The attributes `serialize_always` and `serde(skip_serializing_if = "...")` cannot be used on the same field"#.to_string();
        if let Some(ident) = &field.ident {
            msg += ": `";
            msg += &ident.to_string();
            msg += "`";
        }
        msg += ".";
        return Err(msg);
    }

    // Do nothing if `skip_serializing_if` or `serialize_always` is already present
    if has_skip_serializing_if || has_always_attr {
        return Ok(());
    }

    // Add the `skip_serializing_if` attribute
    let attr = parse_quote!(
        #[serde(skip_serializing_if = "util::is_default")]
    );
    field.attrs.push(attr);
    Ok(())
}

fn field_has_attribute(field: &Field, namespace: &str, name: &str) -> bool {
    for attr in &field.attrs {
        if attr.path.is_ident(namespace) {
            // Ignore non parsable attributes, as these are not important for us
            if let Ok(Meta::List(expr)) = attr.parse_meta() {
                for expr in expr.nested {
                    if let NestedMeta::Meta(Meta::NameValue(expr)) = expr {
                        if let Some(ident) = expr.path.get_ident() {
                            if *ident == name {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

#[proc_macro_attribute]
pub fn skip_serializing_default(_args: TokenStream, input: TokenStream) -> TokenStream {
    let res = match apply_function_to_struct_and_enum_fields(
        input,
        skip_serializing_default_add_attr_to_field,
    ) {
        Ok(res) => res,
        Err(err) => err.to_compile_error(),
    };
    TokenStream::from(res)
}
