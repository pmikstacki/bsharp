use anyhow::{Context, Result};
use log::{info, warn};
use std::fs;
use std::path::{Path, PathBuf};
use clap::{arg, Args, ValueEnum};
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::syntax::span::Span;
use bsharp_syntax::{FormatOptions, Formatter};
use serde::ser;

#[derive(Args, Debug, Clone)]
pub struct FormatArgs {
    /// The input C# file to analyze
    #[arg(required = true)]
    pub input: PathBuf,

    /// write to file
    #[arg(short, long)]
    pub write: Option<bool>,

    /// Newline mode: lf | crlf
    #[arg(long)]
    pub newline_mode: Option<NewlineMode>,

    ///Max consecutive blank lines
    #[arg(short, long)]
    pub max_consecutive_blank_lines: Option<u8>,

    ///Blank lines between members
    #[arg(short, long)]
    pub blank_line_between_members: Option<bool>,

    ///Trim trailing whitespace
    #[arg(short, long)]
    pub trim_trailing_whitespace: Option<bool>,
}

#[derive(ValueEnum, Clone, Default, Debug)]
#[clap(rename_all="lowercase")]
pub enum NewlineMode {
    #[default]
    LF,
    CRLF
}


pub fn execute(
    args: Box<FormatArgs>
) -> Result<()> {
    let mut files = Vec::new();
    collect_cs_files(&args.input, &mut files)?;

    if files.is_empty() {
        warn!("no .cs files found under {}", &args.input.display());
        return Ok(());
    }

    let is_single_file = args.input.is_file();
    let mut changed = 0usize;
    let mut processed = 0usize;
    let mut parse_failed = 0usize;

    for file in files {
        let src = fs::read_to_string(&file)
            .with_context(|| format!("Failed to read file: {}", file.display()))?;

        // Parse strictly; skip files that fail parsing
        let parsed = parse_csharp_source_strict(Span::new(src.as_str()));
        let Ok((_rest, cu)) = parsed else {
            warn!("skipping (parse failed): {}", file.display());
            parse_failed += 1;
            continue;
        };

        // Build options per-file (newline preservation)
        let mut opts = FormatOptions::default();
        opts.max_consecutive_blank_lines = args.max_consecutive_blank_lines.unwrap_or(1);
        opts.blank_line_between_members = args.blank_line_between_members.unwrap_or(true);
        opts.trim_trailing_whitespace = args.trim_trailing_whitespace.unwrap_or(true);
        let newline_mode =  &args.newline_mode.as_ref().unwrap_or(&NewlineMode::LF);
        opts.newline = match newline_mode {
            NewlineMode::LF => "\n",
            NewlineMode::CRLF => "\r\n",
            _ => "\n"
        };

        let fmt = Formatter::new(opts);
        // Format using syntax emitters + normalization
        let formatted = fmt
            .format_compilation_unit(&cu)
            .map_err(|_e| anyhow::anyhow!("Failed to format: {}", file.display()))?;

        if formatted != src {
            if args.write.unwrap_or(true) {
                fs::write(&file, formatted)
                    .with_context(|| format!("Failed to write file: {}", file.display()))?;
                changed += 1;
            } else {
                // If input is a single file and write=false and not --check, print to stdout once then return
                if is_single_file {
                    println!("{}", formatted);
                    return Ok(());
                }
            }
        }
        processed += 1;
    }

    if args.write.unwrap_or(true) {
        info!("formatted {} file(s)", changed);
    } else {
        eprintln!(
            "checked {} file(s), {} would change{}",
            processed,
            changed,
            if parse_failed > 0 { format!(", {} parse failed", parse_failed) } else { String::new() }
        );
        if changed > 0 { std::process::exit(2); }
    }

    Ok(())
}

fn collect_cs_files(path: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    let meta = fs::metadata(path)
        .with_context(|| format!("Failed to stat path: {}", path.display()))?;
    if meta.is_file() {
        if path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.eq_ignore_ascii_case("cs"))
            .unwrap_or(false)
        {
            out.push(path.to_path_buf());
        }
        return Ok(());
    }

    // Directory: walk recursively
    for entry in fs::read_dir(path)
        .with_context(|| format!("Failed to read dir: {}", path.display()))?
    {
        let entry = entry?;
        let p = entry.path();
        if entry.file_type()?.is_dir() {
            // Skip hidden directories like .git, bin, obj
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.starts_with('.') || name == "bin" || name == "obj" || name == "target" {
                continue;
            }
            collect_cs_files(&p, out)?;
        } else if p
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.eq_ignore_ascii_case("cs"))
            .unwrap_or(false)
        {
            out.push(p);
        }
    }
    Ok(())
}
