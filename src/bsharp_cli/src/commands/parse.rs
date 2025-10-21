use anyhow::{Context, Result};
use clap::arg;
use std::fs;
use std::path::{Path, PathBuf};

// Import the syntax from the containing crate
use bsharp_parser::bsharp::{parse_csharp_source, parse_csharp_source_strict};
use bsharp_parser::helpers::brace_tracker;
use bsharp_parser::parse_mode;
use bsharp_parser::syntax::errors as perr;
use bsharp_parser::syntax::node::render;
use bsharp_parser::syntax::span::Span;
use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};
use std::env;

// Select deepest location span from an ErrorTree<Span>
fn deepest_span<'a>(e: &'a ErrorTree<Span<'a>>) -> Span<'a> {
    match e {
        ErrorTree::Base { location, .. } => *location,
        ErrorTree::Stack { base, .. } => deepest_span(base),
        ErrorTree::Alt(list) => list
            .iter()
            .map(|child| deepest_span(child))
            .max_by_key(|s| s.location_offset())
            .unwrap_or_else(|| Span::new("")),
    }
}
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
                    let pretty = perr::format_error_tree(&source_code, &t);
                    if args.errors_json {
                        let payload = serde_json::json!({
                            "error": { "kind": "parse_error", "file": args.input.display().to_string(), "message": pretty }
                        });
                        println!(
                            "{}",
                            serde_json::to_string(&payload).unwrap_or_else(|_| {
                                "{\"error\":{\"message\":\"parse error\"}}".to_string()
                            })
                        );
                    } else {
                        print_pretty_error(&args.input, &pretty, args.no_color);
                    }
                    std::process::exit(1);
                }
            };
            // Pretty body for stderr or message field
            let pretty = perr::format_error_tree(&source_code, err_tree);
            if args.errors_json {
                let loc = deepest_span(err_tree);
                let line = loc.location_line() as usize;
                let column = loc.get_utf8_column();
                let line_text = source_code
                    .lines()
                    .nth(line.saturating_sub(1))
                    .unwrap_or("")
                    .to_string();
                // We don't compute expected/found here; leave empty strings
                let (expected, found) = (String::new(), String::new());
                let payload = serde_json::json!({
                    "error": {
                        "kind": "parse_error",
                        "file": args.input.display().to_string(),
                        "line": line,
                        "column": column,
                        "expected": expected,
                        "found": found,
                        "line_text": line_text,
                        "message": pretty
                    }
                });
                println!("{}", serde_json::to_string(&payload).unwrap_or_else(|_| "{\"error\":{\"message\":\"parse error\"}}".to_string()));
            } else {
                print_pretty_error(&args.input, &pretty, args.no_color);
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
                let pretty = perr::format_error_tree(&source_code, &tree);
                if args.errors_json {
                    let loc = deepest_span(&tree);
                    let line = loc.location_line() as usize;
                    let column = loc.get_utf8_column();
                    let line_text = source_code
                        .lines()
                        .nth(line.saturating_sub(1))
                        .unwrap_or("")
                        .to_string();
                    let (expected, found) = (String::new(), String::new());
                    let payload = serde_json::json!({
                        "error": {
                            "kind": "parse_error",
                            "file": args.input.display().to_string(),
                            "line": line,
                            "column": column,
                            "expected": expected,
                            "found": found,
                            "line_text": line_text,
                            "message": pretty
                        }
                    });
                    println!(
                        "{}",
                        serde_json::to_string(&payload).unwrap_or_else(|_| {
                            "{\"error\":{\"message\":\"parse error\"}}".to_string()
                        })
                    );
                } else {
                    print_pretty_error(&args.input, &pretty, args.no_color);
                }
            } else {
                // Build a synthetic EOF-expected error at the remaining location
                let tree = ErrorTree::Base {
                    location: remaining,
                    kind: BaseErrorKind::Expected(Expectation::Eof),
                };
                let pretty = perr::format_error_tree(&source_code, &tree);
                if args.errors_json {
                    let loc = deepest_span(&tree);
                    let line = loc.location_line() as usize;
                    let column = loc.get_utf8_column();
                    let line_text = source_code
                        .lines()
                        .nth(line.saturating_sub(1))
                        .unwrap_or("")
                        .to_string();
                    let (expected, found) = (String::new(), String::new());
                    let payload = serde_json::json!({
                        "error": {
                            "kind": "parse_error",
                            "file": args.input.display().to_string(),
                            "line": line,
                            "column": column,
                            "expected": expected,
                            "found": found,
                            "line_text": line_text,
                            "message": pretty
                        }
                    });
                    println!(
                        "{}",
                        serde_json::to_string(&payload).unwrap_or_else(|_| {
                            "{\"error\":{\"message\":\"parse error\"}}".to_string()
                        })
                    );
                } else {
                    print_pretty_error(&args.input, &pretty, args.no_color);
                }
            }
        } else {
            // Build a synthetic EOF-expected error at the remaining location
            let tree = ErrorTree::Base {
                location: remaining,
                kind: BaseErrorKind::Expected(Expectation::Eof),
            };
            let pretty = perr::format_error_tree(&source_code, &tree);
            if args.errors_json {
                let loc = deepest_span(&tree);
                let line = loc.location_line() as usize;
                let column = loc.get_utf8_column();
                let line_text = source_code
                    .lines()
                    .nth(line.saturating_sub(1))
                    .unwrap_or("")
                    .to_string();
                let (expected, found) = (String::new(), String::new());
                let payload = serde_json::json!({
                    "error": {
                        "kind": "parse_error",
                        "file": args.input.display().to_string(),
                        "line": line,
                        "column": column,
                        "expected": expected,
                        "found": found,
                        "line_text": line_text,
                        "message": pretty
                    }
                });
                println!("{}", serde_json::to_string(&payload).unwrap_or_else(|_| "{\"error\":{\"message\":\"parse error\"}}".to_string()));
            } else {
                print_pretty_error(&args.input, &pretty, args.no_color);
            }
        }
        std::process::exit(1);
    }

    // Produce a formatted textual tree representation and print to stdout
    let tree = render::to_text(&ast);
    println!("{}", tree);

    Ok(())
}

fn print_pretty_error(input: &Path, pretty_body: &str, no_color_flag: bool) {
    let no_color_env = env::var("NO_COLOR").is_ok();
    let colored = !(no_color_flag || no_color_env);

    let (err_hdr, arrow) = if colored {
        ("\x1b[1;31merror\x1b[0m:", "\x1b[31m^\x1b[0m")
    } else {
        ("error:", "^")
    };
    let file = if colored {
        format!("\x1b[36m{}\x1b[0m", input.display())
    } else {
        format!("{}", input.display())
    };

    // pretty_body already includes line/col and caret; optionally we can recolor caret by replacing '^'
    // For simplicity, prepend header and file path; leave body as-is.
    eprintln!(
        "{} failed to parse file\n  --> {}\n{}",
        err_hdr,
        file,
        pretty_body.replace('^', arrow)
    );
}

// (removed unused legacy helper)
