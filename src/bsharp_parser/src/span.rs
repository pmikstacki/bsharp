use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ByteRange {
    pub start: usize,
    pub end: usize,
}

impl ByteRange {
    #[inline]
    pub fn len(&self) -> usize { self.end.saturating_sub(self.start) }
    #[inline]
    pub fn is_empty(&self) -> bool { self.start >= self.end }

    #[inline]
    pub fn slice<'a>(&self, src: &'a str) -> &'a str {
        &src[self.start..self.end]
    }
}

 
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineOffset {
    pub line: u32,
    pub offset: usize,
}

impl fmt::Display for LineOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.offset)
    }
}

 
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextRange {
    pub start: LineOffset,
    pub end: LineOffset,
}

 
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Spanned<T> {
    pub node: T,
    pub abs: ByteRange,
    pub rel: TextRange,
}

impl<T> Spanned<T> {
    #[inline]
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Spanned<U> {
        Spanned { node: f(self.node), abs: self.abs, rel: self.rel }
    }
    #[inline]
    pub fn into_inner(self) -> T { self.node }
}

pub trait HasSpan {
    fn byte_range(&self) -> Option<ByteRange> { None }
    fn text_range(&self) -> Option<TextRange> { None }
}

impl<T> HasSpan for Spanned<T> {
    #[inline]
    fn byte_range(&self) -> Option<ByteRange> { Some(self.abs) }
    #[inline]
    fn text_range(&self) -> Option<TextRange> { Some(self.rel) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::bytes::complete::tag;
    use nom::Parser as _;
    use crate::span_ext::ParserExt as _;
    use syntax::span::Span;

    #[test]
    fn spanned_basic_single_line() {
        let src = "let x = 42;";
        let input = Span::new(src);
        let (rest, s) = tag("let").spanned().parse(input).unwrap();

        assert_eq!(s.abs.start, 0);
        assert_eq!(s.abs.end, 3);
        assert_eq!(s.rel.start.line, 1);
        assert_eq!(s.rel.start.offset, 0);
        assert_eq!(s.rel.end.line, 1);
        assert_eq!(s.rel.end.offset, 3);
        assert_eq!(s.abs.slice(src), "let");

        assert_eq!(rest.location_offset(), 3);
        assert_eq!(rest.location_line(), 1);
    }

    #[test]
    fn spanned_across_lines_and_utf8() {
        let src = "αβ\n😀foo\nbar";
        let input = Span::new(src);

        let nl = src.find('\n').unwrap();
        let input2 = Span::new(&src[nl + 1..]);

        let (rest, s) = tag("😀").spanned().parse(input2).unwrap();

        assert_eq!(s.abs.start, 0);
        assert!(s.abs.end > 0); // multi-byte
        assert_eq!(s.rel.start.line, 1);
        assert_eq!(s.rel.start.offset, 0);
        assert!(s.rel.end.offset > 0);
        assert_eq!(s.abs.slice(&src[nl+1..]), "😀");

        assert_eq!(&rest.fragment()[..3.min(rest.fragment().len())], "foo");
    }
}
