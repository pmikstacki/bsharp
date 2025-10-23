//! Extensions to the nom [`Parser`][nom::Parser] trait which add postfix
//! versions of the common combinators. See [`ParserExt`] for details.

use core::{marker::PhantomData, str::FromStr};

use nom::{
    error::{ErrorKind as NomErrorKind, FromExternalError, ParseError},
    Emit, Err as NomErr, Input, Offset, OutputM, OutputMode, PResult, Parser,
};

use crate::context::ContextError;

/// No-op function that typechecks that its argument is a parser. Used to
/// ensure there are no accidentally missing type bounds on the `ParserExt`
/// methods
#[inline(always)]
fn must_be_a_parser<I, O, E: ParseError<I>, P: Parser<I, Output = O, Error = E>>(parser: P) -> P {
    parser
}

/// Additional postfix parser combinators, as a complement to [`Parser`].
/// Mostly these are postfix versions of the combinators in [`nom::combinator`]
/// and [`nom::sequence`], with some additional combinators original to
/// `nom-supreme`.
///
/// Compatibility note: it is expected that eventually many of these postfix
/// methods will eventually be added directly to the [`Parser`] trait. It will
/// therefore *not* be considered a compatibility break to remove those methods
/// from [`ParserExt`], *if* they have the same name and signature.
pub trait ParserExt<I, O, E>: Parser<I, Output = O, Error = E> + Sized
where
    E: ParseError<I>,
{
    /// Borrow a parser. This allows building parser combinators while still
    /// retaining ownership of the original parser. This is necessary because
    /// `impl<T: Parser> Parser for &mut T` is impossible due to conflicts
    /// with `impl<T: FnMut> Parser for T`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use nom::{Err, Parser};
    /// # use nom::error::{Error, ErrorKind};
    /// use nom_supreme::parser_ext::ParserExt;
    /// use nom_supreme::tag::complete::tag;
    ///
    /// let mut parser = tag("Hello");
    ///
    ///  {
    /// let mut subparser = parser.by_ref().terminated(tag(", World"));
    ///
    /// assert_eq!(subparser.parse("Hello, World!"), Ok(("!", "Hello")));
    /// assert_eq!(
    ///     subparser.parse("Hello"),
    ///     Err(Err::Error(Error{input: "", code: ErrorKind::Tag}))
    /// );
    /// }
    ///
    /// // We still have ownership of the original parser
    ///
    /// assert_eq!(parser.parse("Hello, World!"), Ok((", World!", "Hello")));
    /// assert_eq!(parser.parse("Hello"), Ok(("", "Hello")));
    /// ```
    #[inline]
    #[must_use = "Parsers do nothing unless used"]
    fn by_ref(&mut self) -> RefParser<'_, Self> {
        must_be_a_parser(RefParser { parser: self })
    }
    /// Create a parser that must consume all of the input, or else return an
    /// error.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use nom::{Err, Parser};
    /// # use nom::error::{Error, ErrorKind};
    /// use nom_supreme::parser_ext::ParserExt;
    /// use nom_supreme::tag::complete::tag;
    ///
    /// let mut parser = tag("Hello").all_consuming();
    ///
    /// assert_eq!(parser.parse("Hello"), Ok(("", "Hello")));
    /// assert_eq!(
    ///     parser.parse("World"),
    ///     Err(Err::Error(Error{input: "World", code: ErrorKind::Tag}))
    /// );
    /// assert_eq!(
    ///     parser.parse("Hello World"),
    ///     Err(Err::Error(Error{input: " World", code: ErrorKind::Eof}))
    /// );
    /// ```
    #[inline]
    #[must_use = "Parsers do nothing unless used"]
    fn all_consuming(self) -> impl Parser<I, Output = O, Error = E>
    where
        I: Input,
        E: ParseError<I>,
    {
        nom::combinator::all_consuming(self)
    }

    /// Create a parser that transforms `Incomplete` into `Error`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use nom::{Err, Parser};
    /// # use nom::error::{Error, ErrorKind};
    /// use nom_supreme::parser_ext::ParserExt;
    /// use nom_supreme::tag::streaming::tag;
    ///
    /// let mut parser = tag("Hello").complete();
    ///
    /// assert_eq!(parser.parse("Hello"), Ok(("", "Hello")));
    /// assert_eq!(
    ///     parser.parse("World"),
    ///     Err(Err::Error(Error{input: "World", code: ErrorKind::Tag}))
    /// );
    /// assert_eq!(
    ///     parser.parse("Hel"),
    ///     Err(Err::Error(Error{input: "Hel", code: ErrorKind::Complete}))
    /// );
    /// ```
    #[inline]
    #[must_use = "Parsers do nothing unless used"]
    fn complete(self) -> impl Parser<I, Output = O, Error = E>
    where
        I: Clone,
        E: ParseError<I>,
    {
        nom::combinator::complete(self)
    }

    /**
    Create a parser that transforms `Error` into `Failure`. This will
    end the parse immediately, even if there are other branches that
    could occur.
    # Example
    ```rust
    use cool_asserts::assert_matches;
    # use nom::{Err, Parser};
    # use nom::error::{Error, ErrorKind};
    use nom::branch::alt;
    use nom::character::complete::char;
    use nom_supreme::parser_ext::ParserExt;
    use nom_supreme::tag::complete::tag;
    use nom_supreme::error::{ErrorTree, BaseErrorKind, Expectation};
    let mut parser = alt((
        tag("Hello").terminated(char(']')).cut().preceded_by(char('[')),
        tag("World").terminated(char(')')).cut().preceded_by(char('(')),
    ));
    assert_matches!(parser.parse("[Hello]"), Ok(("", "Hello")));
    assert_matches!(parser.parse("(World)"), Ok(("", "World")));
    let branches = assert_matches!(
        parser.parse("ABC"),
        Err(Err::Error(ErrorTree::Alt(branches))) => branches
    );
    assert_matches!(
        branches.as_slice(),
        [
            ErrorTree::Base {
                kind: BaseErrorKind::Expected(Expectation::Char('[')),
                location: "ABC",
            },
            ErrorTree::Base {
                kind: BaseErrorKind::Expected(Expectation::Char('(')),
                location: "ABC",
            },
        ]
    );
    // Notice in this example that there's no error for [Hello]. The cut after
    // [ prevented the other branch from being attempted, and prevented earlier
    // errors from being retained
    assert_matches!(
        parser.parse("(Hello)"),
        Err(Err::Failure(ErrorTree::Base {
            kind: BaseErrorKind::Expected(Expectation::Tag("World")),
            location: "Hello)",
        }))
    );
    ```
    */
    #[inline]
    #[must_use = "Parsers do nothing unless used"]
    fn cut(self) -> impl Parser<I, Output = O, Error = E> {
        nom::combinator::cut(self)
    }

    /// Create a parser that applies a mapping function `func` to the output
    /// of the subparser. Any errors from `func` will be transformed into
    /// parse failures via [`FromExternalError`]. This will
    /// end the parse immediately, even if there are other branches that
    /// could occur.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use nom::{Err, Parser};
    /// # use nom::error::{Error, ErrorKind};
    /// use nom::character::complete::alphanumeric1;
    /// use nom_supreme::parser_ext::ParserExt;
    ///
    /// let mut parser = alphanumeric1.map_res_cut(|s: &str| s.parse());
    ///
    /// assert_eq!(parser.parse("10 abc"), Ok((" abc", 10)));
    /// assert_eq!(
    ///     parser.parse("<===>"),
    ///     Err(Err::Error(Error{input: "<===>", code: ErrorKind::AlphaNumeric})),
    /// );
    /// assert_eq!(
    ///     parser.parse("abc abc"),
    ///     Err(Err::Failure(Error{input: "abc abc", code: ErrorKind::MapRes})),
    /// );
    /// ```
    #[inline]
    #[must_use = "Parsers do nothing unless used"]
    fn map_res_cut<F, O2, E2>(mut self, mut func: F) -> impl FnMut(I) -> nom::IResult<I, O2, E>
    where
        F: FnMut(O) -> Result<O2, E2>,
        E: FromExternalError<I, E2>,
        I: Clone,
    {
        move |input: I| -> nom::IResult<I, O2, E> {
            let input_clone = input.clone();

            // Always parse in Emit mode to get a value we can pass to the function
            let (tail, value) = self.parse(input.clone())?;

            match (func)(value) {
                Ok(mapped_value) => Ok((tail, mapped_value)),
                Err(err) => {
                    // MapResCut always returns Failure
                    Err(nom::Err::Failure(E::from_external_error(
                        input_clone,
                        NomErrorKind::MapRes,
                        err,
                    )))
                }
            }
        }
    }

    /// Make this parser optional; if it fails to parse, instead it returns
    /// `None` with the input in the original position.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use nom::{Err, Parser, IResult};
    /// # use nom::error::{Error, ErrorKind};
    /// use nom::bytes::{tag};
    /// use nom::combinator::{cut};
    /// use nom_supreme::parser_ext::ParserExt;
    ///
    /// fn parser(input: &str) -> IResult<&str, Option<&str>> {
    ///     tag("Hello").opt().parse(input)
    /// }
    ///
    /// assert_eq!(parser.parse("Hello, World"), Ok((", World", Some("Hello"))));
    /// assert_eq!(parser.parse("World"), Ok(("World", None)));
    ///
    /// let mut parser = cut(tag("Hello")).opt();
    /// assert_eq!(
    ///     parser.parse("World"),
    ///     Err(Err::Failure(Error{input: "World", code: ErrorKind::Tag}))
    /// )
    /// ```
    #[inline]
    #[must_use = "Parsers do nothing unless used"]
    fn opt(self) -> impl Parser<I, Output = Option<O>, Error = E>
    where
        I: Clone,
    {
        nom::combinator::opt(self)
    }

    /// Replace this parser's output with the entire input that was consumed
    /// by the parser.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use nom::{Err, Parser};
    /// # use nom::error::{Error, ErrorKind};
    /// use nom::character::complete::space1;
    /// use nom_supreme::parser_ext::ParserExt;
    /// use nom_supreme::tag::complete::tag;
    ///
    /// let mut parser = tag("Hello").delimited_by(space1).recognize();
    ///
    /// assert_eq!(parser.parse("   Hello   World!"), Ok(("World!", "   Hello   ")));
    /// assert_eq!(
    ///     parser.parse("Hello"),
    ///     Err(Err::Error(Error{input: "Hello", code: ErrorKind::Space}))
    /// )
    /// ```
    #[inline]
    #[must_use = "Parsers do nothing unless used"]
    fn recognize(self) -> impl Parser<I, Output = I, Error = E>
    where
        I: Clone + Input + Offset,
    {
        nom::combinator::recognize(self)
    }

    /// Return the parsed value, but also return the entire input that was
    /// consumed by the parse
    ///
    /// # Example
    ///
    /// ```rust
    /// # use nom::{Err, Parser};
    /// # use nom::error::{Error, ErrorKind};
    /// use nom::character::complete::space1;
    /// use nom_supreme::parser_ext::ParserExt;
    /// use nom_supreme::tag::complete::tag;
    ///
    /// let mut parser = tag("Hello").delimited_by(space1).with_recognized();
    ///
    /// assert_eq!(parser.parse("   Hello   World!"), Ok(("World!", ("   Hello   ", "Hello"))));
    /// assert_eq!(
    ///     parser.parse("Hello"),
    ///     Err(Err::Error(Error{input: "Hello", code: ErrorKind::Space}))
    /// )
    /// ```
    #[inline]
    #[must_use = "Parsers do nothing unless used"]
    fn with_recognized(self) -> WithRecognized<Self>
    where
        I: Clone + Input + Offset,
    {
        must_be_a_parser(WithRecognized { parser: self })
    }

    /// Replace this parser's output with a clone of `value` every time it
    /// finishes successfully.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cool_asserts::assert_matches;
    /// # use nom::{Err, Parser};
    /// # use nom::error::{Error, ErrorKind};
    /// use nom::branch::alt;
    /// use nom_supreme::parser_ext::ParserExt;
    /// use nom_supreme::tag::complete::tag;
    /// use nom_supreme::error::{ErrorTree, BaseErrorKind, Expectation};
    ///
    ///
    /// let mut parser = alt((
    ///     tag("true").value(true),
    ///     tag("false").value(false),
    /// ));
    ///
    /// assert_eq!(parser.parse("true abc").unwrap(), (" abc", true));
    /// assert_eq!(parser.parse("false abc").unwrap(), (" abc", false));
    ///
    /// // ErrorTree gives much better error reports for alt and tag.
    /// let choices = assert_matches!(
    ///     parser.parse("null"),
    ///     Err(Err::Error(ErrorTree::Alt(choices))) => choices
    /// );
    ///
    /// assert_matches!(
    ///     choices.as_slice(),
    ///     [
    ///         ErrorTree::Base {
    ///             kind: BaseErrorKind::Expected(Expectation::Tag("true")),
    ///             location: "null",
    ///         },
    ///         ErrorTree::Base {
    ///             kind: BaseErrorKind::Expected(Expectation::Tag("false")),
    ///             location: "null",
    ///         },
    ///     ]
    /// )
    /// ```
    #[inline]
    #[must_use = "Parsers do nothing unless used"]
    fn value<T: Clone>(self, value: T) -> impl Parser<I, Output = T, Error = E> {
        nom::combinator::value(value, self)
    }

    /// Require the output of this parser to pass a verifier function, or
    /// else return a parse error.
    ///
    /// ```rust
    /// # use nom::{Err, Parser};
    /// # use nom::error::{Error, ErrorKind};
    /// use nom::character::complete::alpha1;
    /// use nom_supreme::parser_ext::ParserExt;
    ///
    /// let mut parser = alpha1.verify(|s: &&str| s.len() == 5);
    ///
    /// assert_eq!(parser.parse("Hello"), Ok(("", "Hello")));
    /// assert_eq!(parser.parse("Hello, World"), Ok((", World", "Hello")));
    /// assert_eq!(
    ///     parser.parse("abc"),
    ///     Err(Err::Error(Error{input: "abc", code: ErrorKind::Verify}))
    /// );
    /// assert_eq!(
    ///     parser.parse("abcabcabc"),
    ///     Err(Err::Error(Error{input: "abcabcabc", code: ErrorKind::Verify}))
    /// );
    /// assert_eq!(
    ///     parser.parse("123"),
    ///     Err(Err::Error(Error{input: "123", code: ErrorKind::Alpha}))
    /// );
    /// ```
    #[inline]
    #[must_use = "Parsers do nothing unless used"]
    fn verify<F>(self, verifier: F) -> impl Parser<I, Output = O, Error = E>
    where
        F: Fn(&O) -> bool,
        I: Clone,
        E: ParseError<I>,
    {
        nom::combinator::verify(self, verifier)
    }

    /// Add some context to the parser. This context will be added to any
    /// errors that are returned from the parser via [`ContextError`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use nom::{Err, Parser};
    /// # use nom::error::{ErrorKind};
    /// # use nom_language::error::{VerboseError, VerboseErrorKind};
    /// use nom::sequence::separated_pair;
    /// use nom::character::complete::space1;
    /// use nom_supreme::parser_ext::ParserExt;
    /// use nom_supreme::tag::complete::tag;
    ///
    /// let mut parser = separated_pair(
    ///     tag("Hello").context("hello"),
    ///     space1,
    ///     tag("World").context("world"),
    /// )
    /// .context("hello world");
    ///
    /// assert_eq!(parser.parse("Hello World"), Ok(("", ("Hello", "World"))));
    /// assert_eq!(
    ///     parser.parse("Hel"),
    ///     Err(Err::Error(VerboseError {errors: vec![
    ///         ("Hel", VerboseErrorKind::Nom(ErrorKind::Tag)),
    ///         ("Hel", VerboseErrorKind::Context("hello")),
    ///         ("Hel", VerboseErrorKind::Context("hello world")),
    ///     ]}))
    /// );
    /// assert_eq!(
    ///     parser.parse("Hello"),
    ///     Err(Err::Error(VerboseError {errors: vec![
    ///         ("", VerboseErrorKind::Nom(ErrorKind::Space)),
    ///         ("Hello", VerboseErrorKind::Context("hello world")),
    ///     ]}))
    /// );
    /// assert_eq!(
    ///     parser.parse("Hello Wor"),
    ///     Err(Err::Error(VerboseError {errors: vec![
    ///         ("Wor", VerboseErrorKind::Nom(ErrorKind::Tag)),
    ///         ("Wor", VerboseErrorKind::Context("world")),
    ///         ("Hello Wor", VerboseErrorKind::Context("hello world")),
    ///     ]}))
    /// );
    /// ```
    #[inline]
    #[must_use = "Parsers do nothing unless used"]
    fn context<C>(self, context: C) -> Context<Self, C>
    where
        E: ContextError<I, C>,
        I: Clone,
        C: Clone,
    {
        must_be_a_parser(Context {
            context,
            parser: self,
        })
    }

    /// Add a terminator parser. The terminator will run after this parser,
    /// returning any errors, but its output will otherwise be discarded.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use nom::{Err, Parser};
    /// # use nom::error::{Error, ErrorKind};
    /// use nom_supreme::parser_ext::ParserExt;
    /// use nom_supreme::tag::complete::tag;
    ///
    /// let mut parser = tag("Hello").terminated(tag(" World"));
    ///
    /// assert_eq!(parser.parse("Hello World!"), Ok(("!", "Hello")));
    /// assert_eq!(
    ///     parser.parse("Hello"),
    ///     Err(Err::Error(Error{input: "", code: ErrorKind::Tag}))
    /// );
    /// ```
    #[inline]
    #[must_use = "Parsers do nothing unless used"]
    fn terminated<F, O2>(self, terminator: F) -> impl Parser<I, Output = O, Error = E>
    where
        F: Parser<I, Output = O2, Error = E>,
    {
        nom::sequence::terminated(self, terminator)
    }

    /// Make this parser precede another one. The successor parser will run
    /// after this one succeeds, and the successor's output will be returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use nom::{Err, Parser};
    /// # use nom::error::{Error, ErrorKind};
    /// use nom::character::complete::digit1;
    /// use nom_supreme::parser_ext::ParserExt;
    /// use nom_supreme::tag::complete::tag;
    ///
    /// let mut parser = tag("Value: ").precedes(digit1);
    ///
    /// assert_eq!(parser.parse("Value: 25;"), Ok((";", "25")));
    /// assert_eq!(
    ///     parser.parse("Value: "),
    ///     Err(Err::Error(Error{input: "", code: ErrorKind::Digit}))
    /// );
    /// assert_eq!(
    ///     parser.parse("25"),
    ///     Err(Err::Error(Error{input: "25", code: ErrorKind::Tag}))
    /// );
    /// ```
    #[inline]
    #[must_use = "Parsers do nothing unless used"]
    fn precedes<F, O2>(self, successor: F) -> impl Parser<I, Output = O2, Error = E>
    where
        F: Parser<I, Output = O2, Error = E>,
    {
        successor.preceded_by(self)
    }

    /// Make this parser preceded by another one. The `prefix` will run first,
    /// and if it succeeds, its output will be discard and this parser will
    /// be run.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use nom::{Err, Parser};
    /// # use nom::error::{Error, ErrorKind};
    /// use nom::character::complete::digit1;
    /// use nom_supreme::parser_ext::ParserExt;
    /// use nom_supreme::tag::complete::tag;
    ///
    /// let mut parser = digit1.preceded_by(tag("Value: "));
    ///
    /// assert_eq!(parser.parse("Value: 25;"), Ok((";", "25")));
    /// assert_eq!(
    ///     parser.parse("Value: "),
    ///     Err(Err::Error(Error{input: "", code: ErrorKind::Digit}))
    /// );
    /// assert_eq!(
    ///     parser.parse("25"),
    ///     Err(Err::Error(Error{input: "25", code: ErrorKind::Tag}))
    /// );
    /// ```
    #[inline]
    #[must_use = "Parsers do nothing unless used"]
    fn preceded_by<F, O2>(self, prefix: F) -> impl Parser<I, Output = O, Error = E>
    where
        F: Parser<I, Output = O2, Error = E>,
    {
        nom::sequence::preceded(prefix, self)
    }

    /**
    Make this parser optionally precede by another one. `self` will
    run first, and then the `successor` will run even if `self` returns an
    error. Both outputs will be returned. This is functionally equivalent
    to `self.opt().and(successor)`, but it has the added benefit that if
    *both* parsers return an error, the error from the `prefix` will be
    retained, rather than discarded.

    ```rust
    use cool_asserts::assert_matches;
    # use nom::{Err, Parser, IResult};
    use nom::character::complete::{digit1, char};
    use nom_supreme::parser_ext::ParserExt;
    use nom_supreme::error::{ErrorTree, BaseErrorKind, Expectation};

    let mut parser = char('-').or(char('+')).opt_precedes(digit1);

    assert_matches!(parser.parse("123"), Ok(("", (None, "123"))));
    assert_matches!(parser.parse("-123"), Ok(("", (Some('-'), "123"))));

    let choices = assert_matches!(
        parser.parse("abc"),
        Err(Err::Error(ErrorTree::Alt(choices))) => choices,
    );

    assert_matches!(choices.as_slice(), [
        ErrorTree::Base {
            location: "abc",
            kind: BaseErrorKind::Expected(Expectation::Char('-'))
        },
        ErrorTree::Base {
            location: "abc",
            kind: BaseErrorKind::Expected(Expectation::Char('+'))
        },
        ErrorTree::Base {
            location: "abc",
            kind: BaseErrorKind::Expected(Expectation::Digit)
        },
    ]);
    ```
    */
    fn opt_precedes<F, O2>(self, successor: F) -> OptionalPreceded<Self, F>
    where
        E: ParseError<I>,
        I: Clone,
        F: Parser<I, Output = O2, Error = E>,
    {
        must_be_a_parser(OptionalPreceded {
            prefix: self,
            parser: successor,
        })
    }

    /**
    Make this parser optionally preceded by another one. The `prefix` will
    run first, and then this parser will run even if the `prefix` returned
    an error. Both outputs will be returned. This is functionally equivalent
    to `prefix.opt().and(self)`, but it has the added benefit that if *both*
    parsers return an error, the error from the `prefix` will be retained,
    rather than discarded.

    ```rust
    use cool_asserts::assert_matches;
    # use nom::{Err, Parser, IResult};
    use nom::character::complete::{digit1, char};
    use nom_supreme::parser_ext::ParserExt;
    use nom_supreme::error::{ErrorTree, BaseErrorKind, Expectation};

    let mut parser = digit1.opt_preceded_by(char('-'));

    assert_matches!(parser.parse("123"), Ok(("", (None, "123"))));
    assert_matches!(parser.parse("-123"), Ok(("", (Some('-'), "123"))));

    let choices = assert_matches!(
        parser.parse("abc"),
        Err(Err::Error(ErrorTree::Alt(choices))) => choices,
    );

    assert_matches!(choices.as_slice(), [
        ErrorTree::Base {
            location: "abc",
            kind: BaseErrorKind::Expected(Expectation::Char('-'))
        },
        ErrorTree::Base {
            location: "abc",
            kind: BaseErrorKind::Expected(Expectation::Digit)
        },
    ]);
    ```
    */
    #[inline]
    #[must_use = "Parsers do nothing unless used"]
    fn opt_preceded_by<F, O1>(self, prefix: F) -> OptionalPreceded<F, Self>
    where
        E: ParseError<I>,
        I: Clone,
        F: Parser<I, Output = O1, Error = E>,
    {
        must_be_a_parser(OptionalPreceded {
            parser: self,
            prefix,
        })
    }

    /// Make this parser delimited, requiring a `delimiter` as both a prefix and
    /// a suffix. The output of the delimiters is discarded.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use nom::{Err, Parser};
    /// # use nom::error::{Error, ErrorKind};
    /// use nom::character::complete::{char, digit1};
    /// use nom_supreme::parser_ext::ParserExt;
    ///
    /// let mut parser = digit1.delimited_by(char('\''));
    ///
    /// assert_eq!(parser.parse("'123' '456'"), Ok((" '456'", "123")));
    /// assert_eq!(
    ///     parser.parse("'' ''"),
    ///     Err(Err::Error(Error{input: "' ''", code: ErrorKind::Digit}))
    /// );
    /// assert_eq!(
    ///     parser.parse("'123 '"),
    ///     Err(Err::Error(Error{input: " '", code: ErrorKind::Char}))
    /// );
    /// ```
    #[inline]
    #[must_use = "Parsers do nothing unless used"]
    fn delimited_by<D, O2>(self, delimiter: D) -> impl Parser<I, Output = O, Error = E>
    where
        D: Parser<I, Output = O2, Error = E>,
    {
        must_be_a_parser(Delimited {
            parser: self,
            delimiter,
            phantom: PhantomData,
        })
    }

    /// Make this parser peeking: it runs normally but consumes no input.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use nom::{Err, Parser};
    /// # use nom::error::{Error, ErrorKind};
    /// use nom_supreme::parser_ext::ParserExt;
    /// use nom_supreme::tag::complete::tag;
    ///
    /// let mut parser = tag("Hello").peek();
    ///
    /// assert_eq!(parser.parse("Hello World"), Ok(("Hello World", "Hello")));
    /// assert_eq!(
    ///     parser.parse("World"),
    ///     Err(Err::Error(Error{input: "World", code: ErrorKind::Tag}))
    /// );
    /// ```
    #[inline]
    #[must_use = "Parsers do nothing unless used"]
    fn peek(self) -> impl Parser<I, Output = O, Error = E>
    where
        I: Clone,
    {
        nom::combinator::peek(self)
    }

    /// Make this parser a negative lookahead: it will succeed if the subparser
    /// fails, and fail if the subparser succeeds.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use nom::{Err, Parser};
    /// # use nom::error::{Error, ErrorKind};
    /// use nom_supreme::parser_ext::ParserExt;
    /// use nom_supreme::tag::complete::tag;
    ///
    /// let mut parser = tag("Hello").not();
    ///
    /// assert_eq!(parser.parse("World"), Ok(("World", ())));
    /// assert_eq!(
    ///     parser.parse("Hello World"),
    ///     Err(Err::Error(Error{input: "Hello World", code: ErrorKind::Not})),
    /// );
    /// ```
    #[inline]
    #[must_use = "Parsers do nothing unless used"]
    fn not(self) -> impl Parser<I, Output = (), Error = E>
    where
        I: Clone,
        E: ParseError<I>,
    {
        nom::combinator::not(self)
    }

    /// Create a parser that parses something via [`FromStr`], using this
    /// parser as a recognizer for the string to pass to
    /// [`from_str`][FromStr::from_str].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use nom::{Err, Parser, IResult};
    /// # use nom::error::{Error, ErrorKind};
    /// use nom::character::complete::digit1;
    /// use nom_supreme::parser_ext::ParserExt;
    ///
    /// let mut parser = digit1.parse_from_str();
    ///
    /// assert_eq!(parser.parse("123 abc"), Ok((" abc", 123)));
    /// assert_eq!(
    ///     parser.parse("abc"),
    ///     Err(Err::Error(Error{input: "abc", code: ErrorKind::Digit})),
    /// );
    /// ```
    ///
    /// # Parse error example
    ///
    /// If the [`FromStr`] parser fails, the error is recoverable from via
    /// [`FromExternalError`]. In general, though, it's better practice to
    /// ensure your recognizer won't allow invalid strings to be forwarded to
    /// the [`FromStr`] parser
    ///
    /// ```rust
    /// use std::num::ParseIntError;
    /// use cool_asserts::assert_matches;
    /// # use nom::{Err, Parser, IResult};
    /// # use nom::error::{ErrorKind};
    /// use nom::character::complete::alphanumeric1;
    /// use nom_supreme::parser_ext::ParserExt;
    /// use nom_supreme::error::{ErrorTree, BaseErrorKind};
    ///
    /// let mut parser = alphanumeric1.parse_from_str();
    ///
    /// assert_matches!(parser.parse("123 abc"), Ok((" abc", 123)));
    /// assert_matches!(
    ///     parser.parse("abc"),
    ///     Err(Err::Error(ErrorTree::Base{
    ///         location: "abc",
    ///         kind: BaseErrorKind::External(err),
    ///     })) => {
    ///         let _err: &ParseIntError = err.downcast_ref().unwrap();
    ///     },
    /// );
    /// ```
    #[inline]
    #[must_use = "Parsers do nothing unless used"]
    fn parse_from_str<T>(mut self) -> impl FnMut(I) -> nom::IResult<I, T, E>
    where
        O: AsRef<str>,
        I: Clone,
        T: FromStr,
        E: FromExternalError<I, T::Err>,
    {
        move |input: I| {
            let (tail, value_str) = self.parse(input.clone())?;
            match value_str.as_ref().parse() {
                Ok(value) => Ok((tail, value)),
                Err(parse_err) => Err(NomErr::Error(E::from_external_error(
                    input,
                    NomErrorKind::MapRes,
                    parse_err,
                ))),
            }
        }
    }

    /// Create a parser that parses something via [`FromStr`], using this
    /// parser as a recognizer for the string to pass to
    /// [`from_str`][FromStr::from_str]. This parser transforms any errors
    /// from [`FromStr`] into [`Err::Failure`][NomErr::Failure], which will
    /// end the overall parse immediately, even if there are other branches
    /// that could be tried.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use nom::{Err, Parser, IResult};
    /// # use nom::error::{Error, ErrorKind};
    /// use nom::character::complete::alphanumeric1;
    /// use nom_supreme::parser_ext::ParserExt;
    ///
    /// let mut parser = alphanumeric1.parse_from_str_cut();
    ///
    /// assert_eq!(parser.parse("123 abc"), Ok((" abc", 123)));
    /// assert_eq!(
    ///     parser.parse("<===>"),
    ///     Err(Err::Error(Error{input: "<===>", code: ErrorKind::AlphaNumeric})),
    /// );
    /// assert_eq!(
    ///     parser.parse("abc"),
    ///     Err(Err::Failure(Error{input: "abc", code: ErrorKind::MapRes})),
    /// );
    /// ```
    ///
    /// # Parse error example
    ///
    /// If the [`FromStr`] parser fails, the error is recoverable from via
    /// [`FromExternalError`]. In general, though, it's better practice to
    /// ensure your recognizer won't allow invalid strings to be forwarded to
    /// the [`FromStr`] parser
    ///
    /// ```rust
    /// use std::num::ParseIntError;
    /// use cool_asserts::assert_matches;
    /// # use nom::{Err, Parser, IResult};
    /// # use nom::error::{ErrorKind};
    /// use nom::character::complete::alphanumeric1;
    /// use nom_supreme::parser_ext::ParserExt;
    /// use nom_supreme::error::{ErrorTree, BaseErrorKind};
    ///
    /// let mut parser = alphanumeric1.parse_from_str_cut();
    ///
    /// assert_matches!(parser.parse("123 abc"), Ok((" abc", 123)));
    /// assert_matches!(
    ///     parser.parse("abc"),
    ///     Err(Err::Failure(ErrorTree::Base{
    ///         location: "abc",
    ///         kind: BaseErrorKind::External(err),
    ///     })) => {
    ///         let _err: &ParseIntError = err.downcast_ref().unwrap();
    ///     },
    /// );
    /// ```
    #[inline]
    #[must_use = "Parsers do nothing unless used"]
    fn parse_from_str_cut<T>(mut self) -> impl FnMut(I) -> nom::IResult<I, T, E>
    where
        O: AsRef<str>,
        I: Clone,
        T: FromStr,
        E: FromExternalError<I, T::Err> + ParseError<I>,
    {
        move |input: I| {
            let (tail, value_str) = self.parse(input.clone())?;
            match value_str.as_ref().parse() {
                Ok(value) => Ok((tail, value)),
                Err(parse_err) => Err(NomErr::Failure(E::from_external_error(
                    input,
                    NomErrorKind::MapRes,
                    parse_err,
                ))),
            }
        }
    }

    /// Create a parser that parses a fixed-size array by running this parser
    /// in a loop.
    ///
    /// The returned parser implements [`Parser`] generically over any
    /// `const N: usize`, which means it can be used to parse arrays of any
    /// length
    ///
    /// # Example
    ///
    /// ```rust
    /// use nom_supreme::ParserExt;
    /// use cool_asserts::assert_matches;
    /// use nom::bytes::complete::tag;
    /// use nom::character::complete::digit1;
    /// use nom::error::{Error, ErrorKind};
    /// use nom::{Err, Parser};
    ///
    /// assert_matches!(
    ///     digit1::<&str, Error<&str>>
    ///         .terminated(tag(", "))
    ///         .parse_from_str()
    ///         .array()
    ///         .parse("123, 456, 789, abc"),
    ///     Ok(("789, abc", [123, 456]))
    /// );
    ///
    /// assert_matches!(
    ///     digit1::<&str, Error<&str>>
    ///         .terminated(tag(", "))
    ///         .parse_from_str()
    ///         .array()
    ///         .parse("123, 456, 789, abc"),
    ///     Ok(("abc", [123, 456, 789]))
    /// );
    ///
    /// let res: Result<(&str, [u16; 4]), Err<Error<&str>>> = digit1::<&str, Error<&str>>
    ///     .terminated(tag(", "))
    ///     .parse_from_str()
    ///     .array()
    ///     .parse("123, 456, 789, abc");
    ///
    /// assert_matches!(
    ///     res,
    ///     Err(Err::Error(Error {
    ///         input: "abc",
    ///         code: ErrorKind::Digit
    ///     }))
    /// );
    /// ```
    ///
    /// Note that this parser does not attach any additional context to the
    /// error in the event of a parser; consider using [`context`][Self::context]
    /// on the item parser or array parser to add additional information about
    /// where in the input there was a parse failure.
    #[inline]
    #[must_use = "Parsers do nothing unless used"]
    fn array<const N: usize>(mut self) -> impl FnMut(I) -> nom::IResult<I, [O; N], E> {
        move |mut input: I| {
            let array: [O; N] = brownstone::build![{
                let (tail, value) = self.parse(input)?;
                input = tail;
                value
            }];

            Ok((input, array))
        }
    }

    /// Create a parser that parses a fixed-size array by running this parser
    /// in a loop, parsing a separator in between each element.
    ///
    /// The returned parser implements [`Parser`] generically over any
    /// `const N: usize`, which means it can be used to parse arrays of any
    /// length
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::net::{Ipv4Addr, SocketAddrV4};
    /// use cool_asserts::assert_matches;
    /// use nom::character::complete::{char, digit1};
    /// # use nom::{Parser, Err, IResult};
    /// # use nom::error::{ErrorKind, Error};
    /// use nom_supreme::ParserExt;
    /// use nom_supreme::tag::complete::tag;
    ///
    /// let mut parser = digit1
    ///     .parse_from_str()
    ///     .separated_array(char('.'))
    ///     .map(Ipv4Addr::from)
    ///     .terminated(char(':'))
    ///     .and(digit1.parse_from_str())
    ///     .map(|(ip, port)| SocketAddrV4::new(ip, port));
    ///
    /// let (_tail, socket_addr) = parser.parse("192.168.0.1:80").unwrap();
    /// assert_eq!(socket_addr.ip().octets(), [192, 168, 0, 1]);
    /// assert_eq!(socket_addr.port(), 80);
    ///
    /// assert_matches!(
    ///     parser.parse("192.168.0.abc:80"),
    ///     Err(Err::Error(Error{input: "abc:80", code: ErrorKind::Digit})),
    /// );
    ///
    /// assert_matches!(
    ///     parser.parse("192.168.0.1"),
    ///     Err(Err::Error(Error{input: "", code: ErrorKind::Char})),
    /// );
    ///
    /// assert_matches!(
    ///     parser.parse("192.168.0.1000:80"),
    ///     Err(Err::Error(Error{input: "1000:80", code: ErrorKind::MapRes})),
    /// );
    ///
    /// assert_matches!(
    ///     parser.parse("192.168.10abc"),
    ///     Err(Err::Error(Error{input: "abc", code: ErrorKind::Char})),
    /// );
    /// ```
    ///
    /// Note that this parser does not attach any additional context to the
    /// error in the event of a parser; consider using [`context`][Self::context]
    /// on the item, separator, or array parsers to add additional information
    /// about where in the input there was a parse failure.
    #[inline]
    #[must_use = "Parsers do nothing unless used"]
    fn separated_array<F, O2, const N: usize>(
        mut self,
        mut separator: F,
    ) -> impl FnMut(I) -> nom::IResult<I, [O; N], E>
    where
        F: Parser<I, Output = O2, Error = E>,
        I: Clone,
    {
        move |mut input: I| {
            let array: [O; N] = brownstone::build!(|index: usize| {
                let tail = match index {
                    0 => input,
                    _ => separator.parse(input)?.0,
                };
                let (new_tail, value) = self.parse(tail)?;
                input = new_tail;
                value
            });
            Ok((input, array))
        }
    }
}

/// Blanket implementation for all parsers.
impl<I, O, E, P> ParserExt<I, O, E> for P
where
    P: Parser<I, Output = O, Error = E>,
    E: ParseError<I>,
{
}

/// Parser wrapping a mutable reference to a subparser.
#[derive(Debug)]
pub struct RefParser<'a, P> {
    parser: &'a mut P,
}

impl<I, O, E, P> Parser<I> for RefParser<'_, P>
where
    P: Parser<I, Output = O, Error = E>,
    E: ParseError<I>,
{
    type Output = O;
    type Error = E;

    #[inline]
    fn process<OM: nom::OutputMode>(
        &mut self,
        input: I,
    ) -> nom::PResult<OM, I, Self::Output, Self::Error> {
        self.parser.process::<OM>(input)
    }
}

/// Parser which, when successful, returns the result of the inner parser and
/// also the consumed input
#[derive(Debug, Clone, Copy)]
pub struct WithRecognized<P> {
    parser: P,
}

impl<I, O, E, P> Parser<I> for WithRecognized<P>
where
    P: Parser<I, Output = O, Error = E>,
    I: Clone + Input + Offset,
    E: ParseError<I>,
{
    type Output = (I, O);
    type Error = E;

    #[inline]
    fn process<OM: nom::OutputMode>(
        &mut self,
        input: I,
    ) -> nom::PResult<OM, I, Self::Output, Self::Error> {
        let input_clone = input.clone();

        self.parser
            .process::<OM>(input.clone())
            .map(|(tail, output)| {
                let index = input_clone.offset(&tail);
                let recognized = input.take(index);

                (
                    tail,
                    OM::Output::combine(
                        OM::Output::bind(|| recognized),
                        output,
                        |recognized, output| (recognized, output),
                    ),
                )
            })
    }
}

/// Parser which attaches additional context to any errors returned by the
/// subparser.
#[derive(Debug, Clone, Copy)]
pub struct Context<P, C> {
    context: C,
    parser: P,
}

impl<I, O, E, P, C> Parser<I> for Context<P, C>
where
    P: Parser<I, Output = O, Error = E>,
    E: ContextError<I, C> + ParseError<I>,
    I: Clone,
    C: Clone,
{
    type Output = O;
    type Error = E;

    #[inline]
    fn process<OM: OutputMode>(
        &mut self,
        input: I,
    ) -> nom::PResult<OM, I, Self::Output, Self::Error> {
        let input_clone = input.clone();
        let context_clone = self.context.clone();

        self.parser
            .process::<OM>(input.clone())
            .map_err(move |err| match err {
                nom::Err::Incomplete(needed) => nom::Err::Incomplete(needed),
                nom::Err::Error(e) => {
                    // For Error, we need to map the error inside the Mode wrapper
                    nom::Err::Error(OM::Error::map(e, |e| {
                        E::add_context(input_clone.clone(), context_clone.clone(), e)
                    }))
                }
                nom::Err::Failure(e) => {
                    nom::Err::Failure(E::add_context(input, self.context.clone(), e))
                }
            })
    }
}

use nom::Mode;

/// Parser which replaces errors coming from the inner parser.
#[derive(Debug, Clone, Copy)]
pub struct ReplaceError<P, F> {
    new_error: F,
    parser: P,
}

impl<I, P, F, E> Parser<I> for ReplaceError<P, F>
where
    P: Parser<I>,
    F: FnMut() -> E,
    E: ParseError<I>,
{
    type Output = P::Output;
    type Error = E;

    #[inline]
    fn process<OM: OutputMode>(&mut self, input: I) -> PResult<OM, I, Self::Output, Self::Error> {
        self.parser.process::<OM>(input).map_err(|err| {
            // Handle each variant of Err separately
            match err {
                nom::Err::Incomplete(needed) => nom::Err::Incomplete(needed),
                nom::Err::Error(_) => nom::Err::Error(OM::Error::bind(|| (self.new_error)())),
                nom::Err::Failure(_) => nom::Err::Failure((self.new_error)()),
            }
        })
    }
}

/// Parser which gets an optional output from a prefix subparser before running
/// the main subparser. Returns the output even if the prefix subparser returns
/// error.
#[derive(Debug, Clone, Copy)]
pub struct OptionalPreceded<P1, P2> {
    parser: P2,
    prefix: P1,
}

impl<I, O1, O2, E, P1, P2> Parser<I> for OptionalPreceded<P1, P2>
where
    P1: Parser<I, Output = O1, Error = E>,
    P2: Parser<I, Output = O2, Error = E>,
    I: Clone,
    E: ParseError<I>,
{
    type Output = (Option<O1>, O2);
    type Error = E;

    #[inline]
    fn process<OM: OutputMode>(&mut self, input: I) -> PResult<OM, I, Self::Output, Self::Error> {
        match self.prefix.process::<OM>(input.clone()) {
            Ok((input, o1)) => self
                .parser
                .process::<OM>(input)
                .map(|(tail, o2)| (tail, OM::Output::combine(o1, o2, |o1, o2| (Some(o1), o2)))),
            Err(NomErr::Error(err1)) => self
                .parser
                .process::<OM>(input)
                .map(|(tail, o2)| (tail, OM::Output::map(o2, |o2| (None, o2))))
                .map_err(|err2| match err2 {
                    nom::Err::Error(err2) => {
                        nom::Err::Error(OM::Error::combine(err1, err2, |err1, err2| err1.or(err2)))
                    }
                    nom::Err::Incomplete(needed) => nom::Err::Incomplete(needed),
                    nom::Err::Failure(err2) => nom::Err::Failure(err2),
                }),
            Err(err) => Err(err),
        }
    }
}

/// Parser which gets and discards a delimiting value both before and after the
/// main subparser. Returns the output from the main subparser if all were
/// successful.
#[derive(Debug, Clone, Copy)]
pub struct Delimited<P, D, O2> {
    parser: P,
    delimiter: D,
    phantom: PhantomData<O2>,
}

impl<P, D, I, O, E, O2> Parser<I> for Delimited<P, D, O2>
where
    P: Parser<I, Output = O, Error = E>,
    D: Parser<I, Output = O2, Error = E>,
    E: ParseError<I>,
{
    type Output = O;
    type Error = E;

    #[inline]
    fn process<OM: OutputMode>(&mut self, input: I) -> PResult<OM, I, Self::Output, Self::Error> {
        let (input, _) = self.delimiter.process::<OM>(input)?;
        let (input, value) = self.parser.process::<OM>(input)?;
        let (input, _) = self.delimiter.process::<OM>(input)?;

        Ok((input, value))
    }
}

/// Parser which runs a fallible mapping function on the output of the
/// subparser. Any errors returned by the mapping function are transformed
/// into a parse failure.
///
#[derive(Debug, Clone, Copy)]
pub struct MapResCut<P, F, O, E2> {
    parser: P,
    func: F,
    phantom: PhantomData<(O, E2)>,
}

impl<P, F, I, O, E, O2, E2> Parser<I> for MapResCut<P, F, O, E2>
where
    P: Parser<I, Output = O, Error = E>,
    F: FnMut(O) -> Result<O2, E2>,
    E: FromExternalError<I, E2> + ParseError<I>,
    I: Clone,
{
    type Output = O2;
    type Error = E;

    #[inline]
    fn process<OM: OutputMode>(&mut self, input: I) -> PResult<OM, I, Self::Output, Self::Error> {
        let input_clone = input.clone();

        // Always parse in Emit mode to get a value we can pass to the function
        let (tail, value) = self
            .parser
            .process::<OutputM<Emit, OM::Error, OM::Incomplete>>(input)?;

        match (self.func)(value) {
            Ok(mapped_value) => {
                // Wrap the result according to the requested output mode
                Ok((tail, OM::Output::bind(|| mapped_value)))
            }
            Err(err) => {
                // MapResCut always returns Failure
                Err(nom::Err::Failure(E::from_external_error(
                    input_clone,
                    NomErrorKind::MapRes,
                    err,
                )))
            }
        }
    }
}

#[cfg(feature = "error")]
#[test]
fn from_str_parser_non_str_input() {
    use core::str::from_utf8;

    use cool_asserts::assert_matches;
    use nom::{
        character::complete::{char, digit1},
        Err as NomErr,
    };

    use crate::error::{BaseErrorKind, ErrorTree, Expectation};

    let mut parser = digit1
        .opt_preceded_by(char('-'))
        .recognize()
        .map_res(from_utf8)
        .parse_from_str();

    assert_matches!(parser.parse(b"-123"), Ok((b"", -123)));

    let branches = assert_matches!(parser.parse(b"abc"), Err(NomErr::Error(ErrorTree::Alt(branches))) => branches);

    assert_matches!(
        branches.as_slice(),
        [
            ErrorTree::Base {
                location: b"abc",
                kind: BaseErrorKind::Expected(Expectation::Char('-'))
            },
            ErrorTree::Base {
                location: b"abc",
                kind: BaseErrorKind::Expected(Expectation::Digit)
            }
        ]
    )
}
