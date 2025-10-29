use anyhow::{Context, Result};
use clap::arg;
use std::fs;
use std::path::PathBuf;

// Import the syntax from the containing crate
use bsharp_parser::bsharp::{parse_csharp_source, parse_csharp_source_strict};
use bsharp_parser::helpers::brace_tracker;
use bsharp_parser::parse_mode;
use bsharp_parser::syntax::node::render;
use bsharp_syntax::span::Span;
use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};
use crate::errors::{emit_json_error, print_pretty_error};

#[derive(clap::Args)]
#[clap(author, version, about, long_about = None)]
pub struct ParseArgs {
    /// The input C# file to parse
    #[arg(short, long, required = true, value_name = "INPUT")]
    pub input: PathBuf,

    /// The output JSON file (defaults to <input>.json)
    #[arg(short, long, value_name = "OUTPUT")]
    pub output: Option<PathBuf>,

    /// Emit errors as JSON to stdout and exit with non-zero status (disables pretty errors)
    #[arg(long, default_value_t = false)]
    pub errors_json: bool,

    /// Disable ANSI colors in error output (pretty mode only)
    #[arg(long, default_value_t = false)]
    pub no_color: bool,

    /// Lenient mode: allow best-effort recovery (default: strict)
    #[arg(long, default_value_t = false)]
    pub lenient: bool,

    /// Include spans in JSON error output (no effect unless --errors-json is set)
    #[arg(long, default_value_t = false)]
    pub emit_spans: bool,
}

/// Execute the parse command: parse C# file and output JSON
/// On parse failure, pretty-print errors by default and exit non-zero.
/// If `errors_json` is true, emit a JSON error object to stdout instead.
/// If `no_color` is true (or NO_COLOR is set), ANSI colors are disabled in pretty output.
pub fn execute(args: ParseArgs) -> Result<()> {
    // Read the source code
    let source_code = fs::read_to_string(&args.input)
        .with_context(|| format!("Failed to read file: {}", &args.input.display()))?;

    // Select strict or lenient parser and set strict flag for deep parsers
    let parser = if args.lenient {
        parse_csharp_source
    } else {
        parse_csharp_source_strict
    };
    let prev_strict = parse_mode::is_strict();
    parse_mode::set_strict(!args.lenient);

    // Parse the source code (low-level) to preserve structured errors
    let parse_result = parser(Span::new(source_code.as_str()));
    // Restore previous strict mode to avoid leaking state
    parse_mode::set_strict(prev_strict);
    let (remaining, ast) = match parse_result {
        Ok(ok) => ok,
        Err(e) => {
            let err_tree: &ErrorTree<Span> = match &e {
                nom::Err::Error(t) | nom::Err::Failure(t) => t,
                nom::Err::Incomplete(_) => {
                    // Synthesize a generic EOF expectation at end of input
                    let t = ErrorTree::Base {
                        location: Span::new(""),
                        kind: BaseErrorKind::Expected(Expectation::Eof),
                    };
                    if args.errors_json {
                        emit_json_error(&args, &source_code, &t);
                    } else {
                        print_pretty_error(&args, &source_code, &t);
                    }
                    std::process::exit(1);
                }
            };
            if args.errors_json {
                emit_json_error(&args, &source_code, err_tree);
            } else {
                print_pretty_error(&args, &source_code, err_tree);
            }
            std::process::exit(1);
        }
    };

    // Treat significant trailing input as a pretty error with location and caret
    if !args.lenient && !remaining.fragment().trim().is_empty() {
        if let Some(status) = brace_tracker::take_status() {
            if let Some(offset) = status.unmatched_open {
                let tree = ErrorTree::Base {
                    location: Span::new(&source_code[offset..]),
                    kind: BaseErrorKind::Expected(Expectation::Char('}')),
                };
                if args.errors_json {
                    emit_json_error(&args, &source_code, &tree);
                } else {
                    print_pretty_error(&args, &source_code, &tree);
                }
            } else {
                // Build a synthetic EOF-expected error at the remaining location
                let tree = ErrorTree::Base {
                    location: remaining,
                    kind: BaseErrorKind::Expected(Expectation::Eof),
                };
                if args.errors_json {
                    emit_json_error(&args, &source_code, &tree);
                } else {
                    print_pretty_error(&args, &source_code, &tree);
                }
            }
        } else {
            // Build a synthetic EOF-expected error at the remaining location
            let tree = ErrorTree::Base {
                location: remaining,
                kind: BaseErrorKind::Expected(Expectation::Eof),
            };
            if args.errors_json {
                emit_json_error(&args, &source_code, &tree);
            } else {
                print_pretty_error(&args, &source_code, &tree);
            }
        }
        std::process::exit(1);
    }

    // Produce a formatted textual tree representation and print to stdout
    let tree = render::to_text(&ast);
    println!("{}", tree);

    Ok(())
}
