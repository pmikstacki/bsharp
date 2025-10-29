use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, GenericArgument, LitStr, PathArguments, Type,
};

#[proc_macro_derive(AstNode)]
pub fn derive_ast_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let attrs = input.attrs;

    let children_body = match &input.data {
        Data::Struct(ds) => {
            let mut stmts = Vec::new();
            match &ds.fields {
                Fields::Named(named) => {
                    for f in &named.named {
                        let fname = f.ident.as_ref().unwrap();
                        // Borrow field for traversal
                        let access = quote! { &self.#fname };
                        stmts.push(gen_push_for_type(&f.ty, access));
                    }
                }
                Fields::Unnamed(unnamed) => {
                    for (i, f) in unnamed.unnamed.iter().enumerate() {
                        let idx = syn::Index::from(i);
                        let access = quote! { &self.#idx };
                        stmts.push(gen_push_for_type(&f.ty, access));
                    }
                }
                Fields::Unit => {}
            }
            quote! { #(#stmts)* }
        }
        Data::Enum(en) => {
            let mut arms = Vec::new();
            for v in &en.variants {
                let vident = &v.ident;
                match &v.fields {
                    Fields::Unit => {
                        arms.push(quote! { Self::#vident => {} });
                    }
                    Fields::Unnamed(unnamed) => {
                        let mut binds = Vec::new();
                        let mut stmts = Vec::new();
                        for (i, f) in unnamed.unnamed.iter().enumerate() {
                            let b = format_ident!("f{}", i);
                            // Bind names (borrowed via match ergonomics)
                            binds.push(quote! { #b });
                            stmts.push(gen_push_for_type(&f.ty, quote! { #b }));
                        }
                        arms.push(quote! { Self::#vident( #(#binds),* ) => { #(#stmts)* } });
                    }
                    Fields::Named(named) => {
                        let mut binds = Vec::new();
                        let mut stmts = Vec::new();
                        for f in &named.named {
                            let fname = f.ident.as_ref().unwrap();
                            // Bind names (borrowed via match ergonomics)
                            binds.push(quote! { #fname });
                            stmts.push(gen_push_for_type(&f.ty, quote! { #fname }));
                        }
                        arms.push(quote! { Self::#vident { #(#binds),* } => { #(#stmts)* } });
                    }
                }
            }
            quote! { match self { #( #arms ),* } }
        }
        Data::Union(_) => quote! {},
    };

    // Determine node_label_value() body based on struct/enum shape and optional attribute
    let label_value_body = match &input.data {
        Data::Struct(ds) => {
            // Try attribute: #[ast_label(field = "foo")]
            let mut preferred_field: Option<syn::Ident> = None;
            for attr in &attrs {
                if attr.path().is_ident("ast_label") {
                    // Best-effort parsing; ignore errors to keep derive resilient
                    let _ = attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("field") {
                            let lit: LitStr = meta.value()?.parse()?;
                            preferred_field = Some(format_ident!("{}", lit.value()));
                        }
                        Ok(())
                    });
                }
            }
            // If no attribute, fall back to a field literally named `name`
            let mut fallback_name_field: Option<syn::Ident> = None;
            if let Fields::Named(named) = &ds.fields {
                for f in &named.named {
                    if let Some(fid) = &f.ident {
                        if fid == "name" { fallback_name_field = Some(fid.clone()); break; }
                    }
                }
            }
            let field_ident = preferred_field.or(fallback_name_field);
            if let Some(fid) = field_ident {
                // Find the field type for smarter formatting (e.g., Option<Identifier>)
                let mut is_option = false;
                if let Fields::Named(named) = &ds.fields {
                    for f in &named.named {
                        if let Some(ffid) = &f.ident {
                            if ffid == &fid {
                                if let Type::Path(tp) = &f.ty {
                                    if let Some(seg) = tp.path.segments.last() {
                                        if seg.ident == "Option" { is_option = true; }
                                    }
                                }
                                break;
                            }
                        }
                    }
                }
                if is_option {
                    quote! {
                        match (&self.#fid) {
                            ::core::option::Option::Some(__v) => ::core::option::Option::Some(format!("{}", __v)),
                            ::core::option::Option::None => ::core::option::Option::None,
                        }
                    }
                } else {
                    quote! { ::core::option::Option::Some(format!("{}", self.#fid)) }
                }
            } else {
                quote! { ::core::option::Option::None }
            }
        }
        Data::Enum(en) => {
            let mut arms = Vec::new();
            for v in &en.variants {
                let vident = &v.ident;
                match &v.fields {
                    Fields::Unit => {
                        arms.push(quote! { Self::#vident => ::core::option::Option::Some(stringify!(#vident).to_string()) });
                    }
                    Fields::Unnamed(unnamed) => {
                        // Only include payload when it is a single String; otherwise omit payload to avoid verbose labels
                        if unnamed.unnamed.len() == 1 {
                            let b0 = format_ident!("_f0");
                            let is_string = match &unnamed.unnamed.first().unwrap().ty {
                                Type::Path(tp) => tp.path.segments.last().map(|s| s.ident == "String").unwrap_or(false),
                                _ => false,
                            };
                            if is_string {
                                arms.push(quote! { Self::#vident( #b0 ) => ::core::option::Option::Some(format!("{} ({})", stringify!(#vident), #b0)) });
                            } else {
                                arms.push(quote! { Self::#vident( #b0 ) => ::core::option::Option::Some(stringify!(#vident).to_string()) });
                            }
                        } else {
                            arms.push(quote! { Self::#vident( .. ) => ::core::option::Option::Some(stringify!(#vident).to_string()) });
                        }
                    }
                    Fields::Named(_named) => {
                        // Omit named field values to keep labels concise
                        arms.push(quote! { Self::#vident { .. } => ::core::option::Option::Some(stringify!(#vident).to_string()) });
                    }
                }
            }
            quote! { match self { #( #arms ),* } }
        }
        Data::Union(_) => quote! { ::core::option::Option::None },
    };

    let expanded = quote! {
        #[allow(unused_variables)]
        impl crate::node::ast_node::AstNode for #name {
            fn as_any(&self) -> &dyn ::core::any::Any { self }
            fn children<'a>(&'a self, push: &mut dyn FnMut(crate::node::ast_node::NodeRef<'a>)) {
                #children_body
            }
            fn node_label_value(&self) -> ::core::option::Option<::std::string::String> {
                #label_value_body
            }
        }
    };
    TokenStream::from(expanded)
}

fn gen_push_for_type(ty: &Type, access: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match ty {
        Type::Path(tp) => {
            if let Some(seg) = tp.path.segments.last() {
                let ident = &seg.ident;
                let args = &seg.arguments;
                let ident_str = ident.to_string();
                match (ident_str.as_str(), args) {
                    // Handle Option<T>
                    ("Option", PathArguments::AngleBracketed(ab)) => {
                        if let Some(GenericArgument::Type(inner_ty)) = ab.args.first() {
                            let v = format_ident!("__v");
                            let inner = gen_push_for_type(inner_ty, quote! { #v });
                            return quote! {
                                if let ::core::option::Option::Some(#v) = (#access).as_ref() { #inner }
                            };
                        }
                    }
                    // Handle Vec<T>
                    ("Vec", PathArguments::AngleBracketed(ab)) => {
                        if let Some(GenericArgument::Type(inner_ty)) = ab.args.first() {
                            let v = format_ident!("__it");
                            let inner = gen_push_for_type(inner_ty, quote! { #v });
                            // Iterate explicitly via .iter() to ensure &T items
                            return quote! {
                                for #v in (#access).iter() { #inner }
                            };
                        }
                    }
                    // Handle Box<T>
                    ("Box", PathArguments::AngleBracketed(ab)) => {
                        if let Some(GenericArgument::Type(inner_ty)) = ab.args.first() {
                            // Borrow inner: &Box<T> -> &T
                            let inner = gen_push_for_type(inner_ty, quote! { (#access).as_ref() });
                            return quote! { #inner };
                        }
                    }
                    _ => {}
                }
                // Skip primitives and String and internal primitive enums
                if is_primitive_like(ident) {
                    return quote! {};
                }
            }
            // Default: treat as AST node; `#access` is a &T already.
            quote! {
                let __n: &'a dyn crate::node::ast_node::AstNode = #access;
                push(crate::node::dyn_node_ref::DynNodeRef(__n));
            }
        }
        _ => quote! {},
    }
}

fn is_primitive_like(ident: &syn::Ident) -> bool {
    let s = ident.to_string();
    matches!(
        s.as_str(),
        "bool"
            | "char"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "isize"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "usize"
            | "f32"
            | "f64"
            | "String"
            | "PrimitiveType"
    )
}
