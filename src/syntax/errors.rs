use nom::error::{convert_error, VerboseError};
use nom::IResult;
use nom_supreme::error::ErrorTree;

pub type BResult<I, O> = IResult<I, O, ErrorTree<I>>;

/// Helper function to format VerboseError with line/column information
pub fn format_parse_error(input: &str, error: VerboseError<&str>) -> String {
    convert_error(input, error)
}
