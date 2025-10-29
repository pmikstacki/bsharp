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
