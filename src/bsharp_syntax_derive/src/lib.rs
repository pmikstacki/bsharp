use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(AstNode)]
pub fn derive_ast_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl crate::node::ast_node::AstNode for #name {
            fn as_any(&self) -> &dyn ::core::any::Any { self }
            fn children<'a>(&'a self, _push: &mut dyn FnMut(crate::node::ast_node::NodeRef<'a>)) {}
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(HasSpan)]
pub fn derive_has_span(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Very simple derive: requires a struct with a named field `span`
    // We do a basic check to help with clearer errors.
    let has_span_field = match &input.data {
        Data::Struct(ds) => match &ds.fields {
            Fields::Named(fields_named) => fields_named.named.iter().any(|f| {
                f.ident.as_ref().map(|id| id == "span").unwrap_or(false)
            }),
            _ => false,
        },
        _ => false,
    };

    let expanded = if has_span_field {
        quote! {
            impl crate::node::has_span::HasSpan for #name {
                fn span(&self) -> crate::node::has_span::TextRange { self.span }
            }
        }
    } else {
        // Fallback: generate a compile error explaining the expectation.
        quote! {
            compile_error!("#[derive(HasSpan)] requires a struct with a named field `span: TextRange`.");
        }
    };

    TokenStream::from(expanded)
}
