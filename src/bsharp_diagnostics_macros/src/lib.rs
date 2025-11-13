use proc_macro::TokenStream;
use quote::quote;
use syn::{braced, parse::Parse, parse_macro_input, punctuated::Punctuated, token::Comma, Ident, LitStr, Token};

struct Entry {
    code: Ident,
    _arrow: Token![=>],
    _brace_token: syn::token::Brace,
    message: LitStr,
}

// enum_as_str!(TypeName { VariantA => "a", VariantB => "b" })
#[derive(Debug)]
struct AsStrItem {
    variant: Ident,
    _arrow: Token![=>],
    text: LitStr,
}

#[derive(Debug)]
struct AsStrSpec {
    ty: Ident,
    _brace: syn::token::Brace,
    items: Punctuated<AsStrItem, Comma>,
}

impl Parse for AsStrItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(AsStrItem {
            variant: input.parse()?,
            _arrow: input.parse()?,
            text: input.parse()?,
        })
    }
}

impl Parse for AsStrSpec {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ty: Ident = input.parse()?;
        let content;
        let _brace = braced!(content in input);
        let items = Punctuated::<AsStrItem, Comma>::parse_terminated(&content)?;
        Ok(AsStrSpec { ty, _brace, items })
    }
}

#[proc_macro]
pub fn enum_as_str(input: TokenStream) -> TokenStream {
    let AsStrSpec { ty, items, .. } = parse_macro_input!(input as AsStrSpec);
    let mut arms = Vec::new();
    for it in items.iter() {
        let v = &it.variant;
        let s = &it.text;
        arms.push(quote! { #ty::#v => #s });
    }
    let out = quote! {
        impl #ty {
            pub fn as_str(&self) -> &'static str {
                match self { #( #arms, )* }
            }
        }
    };
    out.into()
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
    let mut severity_arms = Vec::new();
    let mut category_arms = Vec::new();

    for e in entries.iter() {
        let code_ident = &e.code;
        let code_str = code_ident.to_string();
        let msg = &e.message;
        enum_variants.push(quote! { #code_ident });
        as_str_arms.push(quote! { DiagnosticCode::#code_ident => #code_str });
        default_msg_arms.push(quote! { DiagnosticCode::#code_ident => #msg });

        // Severity derived from prefix: BSE* => Error, BSW* => Warning
        if code_str.starts_with("BSE") {
            severity_arms.push(quote! { DiagnosticCode::#code_ident => DiagnosticSeverity::Error });
        } else {
            severity_arms.push(quote! { DiagnosticCode::#code_ident => DiagnosticSeverity::Warning });
        }

        // Category derived from code family
        // Errors: BSE03*** => Type; other BSE => Semantic
        // Warnings: BSW01*** => Maintainability; BSW02*** => Style; BSW03*** => Performance; BSW04*** => Security
        if code_str.starts_with("BSE03") {
            category_arms.push(quote! { DiagnosticCode::#code_ident => DiagnosticCategory::Type });
        } else if code_str.starts_with("BSE") {
            category_arms.push(quote! { DiagnosticCode::#code_ident => DiagnosticCategory::Semantic });
        } else if code_str.starts_with("BSW01") {
            category_arms.push(quote! { DiagnosticCode::#code_ident => DiagnosticCategory::Maintainability });
        } else if code_str.starts_with("BSW02") {
            category_arms.push(quote! { DiagnosticCode::#code_ident => DiagnosticCategory::Style });
        } else if code_str.starts_with("BSW03") {
            category_arms.push(quote! { DiagnosticCode::#code_ident => DiagnosticCategory::Performance });
        } else if code_str.starts_with("BSW04") {
            category_arms.push(quote! { DiagnosticCode::#code_ident => DiagnosticCategory::Security });
        }
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
            pub fn severity(&self) -> DiagnosticSeverity {
                match self { #( #severity_arms, )* }
            }
            pub fn category(&self) -> DiagnosticCategory {
                match self { #( #category_arms, )* }
            }
        }
    };

    out.into()
}

struct EnumList {
    idents: Punctuated<Ident, Comma>,
}

impl Parse for EnumList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let idents = Punctuated::<Ident, Comma>::parse_terminated(input)?;
        Ok(EnumList { idents })
    }
}

#[proc_macro]
pub fn diagnostic_enum(input: TokenStream) -> TokenStream {
    let EnumList { idents } = parse_macro_input!(input as EnumList);
    let variants: Vec<Ident> = idents.into_iter().collect();
    let as_str_arms = variants.iter().map(|v| {
        let s = v.to_string();
        quote! { DiagnosticCode::#v => #s }
    });
    let out = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub enum DiagnosticCode { #( #variants ),* }

        impl DiagnosticCode {
            pub fn as_str(&self) -> &'static str {
                match self { #( #as_str_arms, )* }
            }
        }
    };
    out.into()
}

#[derive(Debug)]
struct RuleItem {
    name: Ident,
    id_str: LitStr,
    category_str: LitStr,
    visit_block: syn::Block,
}

impl Parse for RuleItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        let id_str: LitStr = input.parse()?;
        let _: Token![,] = input.parse()?;
        let category_str: LitStr = input.parse()?;
        let _: Token![,] = input.parse()?;
        let visit_block: syn::Block = input.parse()?;
        Ok(RuleItem { name, id_str, category_str, visit_block })
    }
}

struct RuleList {
    items: Punctuated<RuleItem, Comma>,
}

impl Parse for RuleList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let items = Punctuated::<RuleItem, Comma>::parse_terminated(input)?;
        Ok(RuleList { items })
    }
}

#[proc_macro]
pub fn rule(input: TokenStream) -> TokenStream {
    let RuleList { items } = parse_macro_input!(input as RuleList);
    let mut struct_defs = Vec::new();
    let mut impls = Vec::new();

    for item in items {
        let name = item.name;
        let id_str = item.id_str;
        let category_str = item.category_str;
        let visit_block = item.visit_block;
        struct_defs.push(quote! {
            struct #name;
        });
        impls.push(quote! {
            impl Rule for #name {
                fn id(&self) -> &'static str { #id_str }
                fn category(&self) -> &'static str { #category_str }
                fn visit(&self, node: &NodeRef, session: &mut AnalysisSession) {
                    #visit_block
                }
            }
        });
    }

    let out = quote! {
        #( #struct_defs )*
        #( #impls )*
    };
    out.into()
}

#[derive(Debug)]
struct RulesetSpec {
    name: Ident,
    rules: Punctuated<Ident, Comma>,
}

impl Parse for RulesetSpec {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        let rules = Punctuated::<Ident, Comma>::parse_terminated(input)?;
        Ok(RulesetSpec { name, rules })
    }
}

#[proc_macro]
pub fn ruleset(input: TokenStream) -> TokenStream {
    let RulesetSpec { name, rules } = parse_macro_input!(input as RulesetSpec);
    let fn_name = Ident::new(&format!("{}_ruleset", name), name.span());
    let name_str = name.to_string();
    let mut with_rule_arms = Vec::new();
    for rule_ident in rules {
        with_rule_arms.push(quote! { rs = rs.with_rule(#rule_ident); });
    }
    let out = quote! {
        pub fn #fn_name() -> RuleSet {
            let mut rs = RuleSet::new(#name_str);
            #( #with_rule_arms )*
            rs
        }
    };
    out.into()
}
