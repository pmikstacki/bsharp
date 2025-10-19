#[macro_export]
macro_rules! define_token_pair_str {
    ($tok_fn:ident, $peek_fn:ident, $lit:literal) => {
        pub fn $tok_fn() -> impl FnMut($crate::syntax::span::Span) -> $crate::syntax::errors::BResult<&str> {
            use nom::Parser as _;
            (|i: $crate::syntax::span::Span| {
                nom::combinator::map(
                    nom_supreme::tag::complete::tag($lit),
                    |s: $crate::syntax::span::Span| *s.fragment(),
                )
                .parse(i)
            })
        }
        pub fn $peek_fn() -> impl FnMut($crate::syntax::span::Span) -> $crate::syntax::errors::BResult<&str> {
            use nom::Parser as _;
            (|i: $crate::syntax::span::Span| nom::combinator::peek($tok_fn()).parse(i))
        }
    };
}

#[macro_export]
macro_rules! define_token_pair_chr {
    ($tok_fn:ident, $peek_fn:ident, $ch:literal) => {
        pub fn $tok_fn() -> impl FnMut($crate::syntax::span::Span) -> $crate::syntax::errors::BResult<char> {
            use nom::Parser as _;
            (|i: $crate::syntax::span::Span| nom::character::complete::char($ch).parse(i))
        }
        pub fn $peek_fn() -> impl FnMut($crate::syntax::span::Span) -> $crate::syntax::errors::BResult<char> {
            use nom::Parser as _;
            (|i: $crate::syntax::span::Span| nom::combinator::peek(nom::character::complete::char($ch)).parse(i))
        }
    };
}

pub mod delimiters;
pub mod separators;
pub mod member;
pub mod lambda;
pub mod range;
pub mod nullish;
pub mod arithmetic;
pub mod bitwise;
pub mod equality;
pub mod relational;
pub mod shift;
pub mod assignment;
pub mod logical;
pub mod conditional;
pub mod qualifiers;
pub mod pointer;
pub mod sigils;
pub mod quotes;
pub mod string;