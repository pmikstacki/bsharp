use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{braced, parse::Parse, parse_macro_input, punctuated::Punctuated, token::Comma, Ident, LitStr, Token};

struct Entry {
    code: Ident,
    _arrow: Token![=>],
    _brace_token: syn::token::Brace,
    message: LitStr,
}

impl Parse for Entry {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let code: Ident = input.parse()?;
        let _arrow: Token![=>] = input.parse()?;
        let content;
        let _brace_token = braced!(content in input);

        // Minimal v1: require only message: "..."
        // Accept optional fields but ignore them for now to keep parser flexible.
        // Example:
        // BSE01001 => { message: "...", category: Semantic, severity: Error }
        let mut message: Option<LitStr> = None;
        while !content.is_empty() {
            let field: Ident = content.parse()?;
            let _: Token![:] = content.parse()?;
            match field.to_string().as_str() {
                "message" => {
                    let lit: LitStr = content.parse()?;
                    message = Some(lit);
                }
                _ => {
                    // Consume a token tree for unknown fields (ident, path, string, etc.)
                    // to allow forward-compatible syntax; ignore value.
                    let _ = content.parse::<syn::Expr>()?;
                }
            }
            let _ = content.parse::<Comma>();
        }
        let message = message.ok_or_else(|| syn::Error::new(code.span(), "missing `message` for diagnostic"))?;
        Ok(Entry { code, _arrow, _brace_token, message })
    }
}

struct Spec {
    entries: Punctuated<Entry, Comma>,
}

impl Parse for Spec {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let entries = Punctuated::<Entry, Comma>::parse_terminated(input)?;
        Ok(Spec { entries })
    }
}

#[proc_macro]
pub fn diagnostics(input: TokenStream) -> TokenStream {
    let Spec { entries } = parse_macro_input!(input as Spec);

    let mut enum_variants = Vec::new();
    let mut as_str_arms = Vec::new();
    let mut default_msg_arms = Vec::new();

    for e in entries.iter() {
        let code_ident = &e.code;
        let code_str = code_ident.to_string();
        let msg = &e.message;
        enum_variants.push(quote! { #code_ident });
        as_str_arms.push(quote! { DiagnosticCode::#code_ident => #code_str });
        default_msg_arms.push(quote! { DiagnosticCode::#code_ident => #msg });
    }

    let out = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub enum DiagnosticCode { #( #enum_variants, )* }

        impl DiagnosticCode {
            pub fn as_str(&self) -> &'static str {
                match self { #( #as_str_arms, )* }
            }
            pub fn default_message(&self) -> &'static str {
                match self { #( #default_msg_arms, )* }
            }
        }
    };

    out.into()
}
