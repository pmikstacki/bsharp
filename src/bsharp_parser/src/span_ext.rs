use nom::{IResult, Parser};
use crate::span::{Spanned, ByteRange, LineOffset, TextRange};
use syntax::span::Span;

pub trait ParserExt<'a, O, E>: Parser<Span<'a>, Output = O, Error = E> + Sized {
    fn spanned(mut self) -> impl FnMut(Span<'a>) -> nom::IResult<Span<'a>, Spanned<O>, E> {
        move |input: Span<'a>| {
            let start_abs = input.location_offset();
            let start_lo = LineOffset {
                line: input.location_line(),
                offset: input.get_utf8_column().saturating_sub(1),
            };

            let (rest, node) = self.parse(input)?;

            let end_abs = rest.location_offset();
            let end_lo = LineOffset {
                line: rest.location_line(),
                offset: rest.get_utf8_column().saturating_sub(1),
            };

            let abs = ByteRange { start: start_abs, end: end_abs };
            let rel = TextRange { start: start_lo, end: end_lo };
            Ok((rest, Spanned { node, abs, rel }))
        }
    }
}

impl<'a, O, E, P> ParserExt<'a, O, E> for P where P: Parser<Span<'a>, Output = O, Error = E> {}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::bytes::complete::tag;
    use nom::Parser as _;

    #[test]
    fn column_and_line_offsets() {
        let src = "abc\nxyz";
        let input = Span::new(src);
        let (_rest, s) = nom::bytes::complete::tag::<&str, Span<'_>, nom_supreme::error::ErrorTree<Span<'_>>>("abc")
            .spanned()
            .parse(input)
            .unwrap();
        assert_eq!(s.rel.start.line, 1);
        assert_eq!(s.rel.start.offset, 0);
        assert_eq!(s.rel.end.line, 1);
        assert_eq!(s.rel.end.offset, 3);
    }
}
