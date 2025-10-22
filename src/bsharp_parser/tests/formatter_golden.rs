use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::syntax::span::Span;
use syntax::formatter::formatter::{FormatOptions, Formatter};
fn format_src(src: &str, opts: FormatOptions) -> String {
    let (_rest, cu) = parse_csharp_source_strict(Span::new(src)).expect("parse ok");
    let fmt = Formatter::new(opts);
    fmt.format_compilation_unit(&cu).expect("format ok")
}

#[test]
fn golden_between_members_single_blank_line() {
    let src = r#"
class C
{
    void M1(){}
    void M2(){}
}
"#;

    let out = format_src(src, FormatOptions::default());

    // Invariant: exactly one blank line between two member lines
    let lines: Vec<&str> = out.lines().collect();
    let mut idx_m1 = None;
    let mut idx_m2 = None;
    for (i, l) in lines.iter().enumerate() {
        if l.contains("void M1") {
            idx_m1 = Some(i);
        }
        if l.contains("void M2") {
            idx_m2 = Some(i);
        }
    }
    let (i1, i2) = (idx_m1.expect("m1 line"), idx_m2.expect("m2 line"));
    let between = &lines[i1 + 1..i2];
    let blank_count = between.iter().filter(|l| l.trim().is_empty()).count();
    assert_eq!(
        blank_count, 1,
        "expected exactly one blank line between members, got {}",
        blank_count
    );

    // Idempotency
    let out2 = format_src(&out, FormatOptions::default());
    assert_eq!(out2, out);
}

#[test]
fn golden_file_scoped_namespace_and_usings_spacing() {
    let src = r#"
namespace X.Y;
using System;
using System.IO;

class C {}
"#;

    let out = format_src(src, FormatOptions::default());

    // Invariant: blank line after file-scoped namespace header
    assert!(out.starts_with("namespace X.Y;\n\n"));

    // Invariant: usings appear before declarations and are followed by a blank line before class
    let ns_end = out.find("\n\n").unwrap();
    let after_ns = &out[ns_end + 2..];
    assert!(after_ns.contains("using System;"));
    assert!(after_ns.contains("using System.IO;"));
    let idx_class = after_ns.find("class C").expect("class present");
    let before_class = &after_ns[..idx_class];
    assert!(
        before_class.ends_with("\n\n"),
        "expected blank line between usings and class"
    );

    // Idempotency
    let out2 = format_src(&out, FormatOptions::default());
    assert_eq!(out2, out);
}

#[test]
fn golden_top_level_usings_then_declaration_spacing() {
    let src = r#"
using System;
using System.IO;
class C {}
"#;

    let out = format_src(src, FormatOptions::default());

    // Invariant: one blank line between using block and first declaration
    let idx_class = out.find("class C").expect("class present");
    let before_class = &out[..idx_class];
    assert!(
        before_class.ends_with("\n\n"),
        "expected one blank line before class"
    );
}
