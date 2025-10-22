use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, GenericArgument, PathArguments, Type, parse_macro_input};

#[proc_macro_derive(AstNode)]
pub fn derive_ast_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

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

    let expanded = quote! {
        #[allow(unused_variables)]
        impl crate::node::ast_node::AstNode for #name {
            fn as_any(&self) -> &dyn ::core::any::Any { self }
            fn children<'a>(&'a self, push: &mut dyn FnMut(crate::node::ast_node::NodeRef<'a>)) {
                #children_body
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
                // Handle Option<T>
                if ident == "Option" {
                    if let PathArguments::AngleBracketed(ab) = args {
                        if let Some(GenericArgument::Type(inner_ty)) = ab.args.first() {
                            let v = format_ident!("__v");
                            let inner = gen_push_for_type(inner_ty, quote! { #v });
                            return quote! {
                                if let ::core::option::Option::Some(#v) = (#access).as_ref() { #inner }
                            };
                        }
                    }
                }
                // Handle Vec<T>
                if ident == "Vec" {
                    if let PathArguments::AngleBracketed(ab) = args {
                        if let Some(GenericArgument::Type(inner_ty)) = ab.args.first() {
                            let v = format_ident!("__it");
                            let inner = gen_push_for_type(inner_ty, quote! { #v });
                            // Iterate explicitly via .iter() to ensure &T items
                            return quote! {
                                for #v in (#access).iter() { #inner }
                            };
                        }
                    }
                }
                // Handle Box<T>
                if ident == "Box" {
                    if let PathArguments::AngleBracketed(ab) = args {
                        if let Some(GenericArgument::Type(inner_ty)) = ab.args.first() {
                            // Borrow inner: &Box<T> -> &T
                            let inner = gen_push_for_type(inner_ty, quote! { (#access).as_ref() });
                            return quote! { #inner };
                        }
                    }
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
