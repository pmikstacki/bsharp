use nom::error::{ContextError, ErrorKind, FromExternalError, ParseError};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum CustomErrorKind {
    Nom(ErrorKind),
    Expected(&'static str),
    // Add more specific kinds as needed
    TryStatementMissingCatchOrFinally, 
}

#[derive(Debug, Clone, PartialEq)]
pub struct BSharpParseError<I> {
    pub input: I,
    pub kind: CustomErrorKind,
    pub context: Vec<&'static str>,
}

impl<I: Clone + std::fmt::Display> BSharpParseError<I> {
    pub fn new(input: I, kind: CustomErrorKind) -> Self {
        BSharpParseError {
            input,
            kind,
            context: Vec::new(),
        }
    }

    pub fn from_nom_error(err: nom::error::Error<I>) -> Self {
        BSharpParseError {
            input: err.input,
            kind: CustomErrorKind::Nom(err.code),
            context: vec![],
        }
    }

    pub fn add_context_static(input: I, ctx: &'static str, mut other: Self) -> Self {
        other.input = input; // Update input position for context
        other.context.push(ctx);
        other
    }
}

impl<I: Clone + std::fmt::Display> ParseError<I> for BSharpParseError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        BSharpParseError::new(input, CustomErrorKind::Nom(kind))
    }

    fn append(input: I, kind: ErrorKind, _other: Self) -> Self {
        // Typically, you might prefer the existing error, but here we update
        // the input position and kind if the new error is more specific (or just overwrite).
        // For simplicity, let's just create a new error at the failure point.
        // A more sophisticated strategy could merge contexts or choose the 'deepest' error.
        Self::from_error_kind(input, kind)
        // Or potentially: other // if you want to keep the original error context
    }
}

impl<I: Clone + std::fmt::Display> ContextError<I> for BSharpParseError<I> {
    fn add_context(input: I, ctx: &'static str, other: Self) -> Self {
        BSharpParseError::add_context_static(input, ctx, other)
    }
}

// Allow converting external errors (though not strictly needed for context)
impl<I: Clone + std::fmt::Display, E> FromExternalError<I, E> for BSharpParseError<I> {
    fn from_external_error(input: I, kind: ErrorKind, _e: E) -> Self {
        // You might want to capture details from E if it's meaningful
        BSharpParseError::new(input, CustomErrorKind::Nom(kind))
    }
}

impl<I: fmt::Display> fmt::Display for BSharpParseError<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error {:?} at input starting near: '{}'. Context: [{}]",
            self.kind,
            self.input.to_string().chars().take(20).collect::<String>(),
            self.context.join(" -> ")
        )
    }
}

// Define a type alias for convenience
pub type BResult<I, O> = nom::IResult<I, O, BSharpParseError<I>>;
