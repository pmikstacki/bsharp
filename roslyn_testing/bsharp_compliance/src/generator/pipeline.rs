use crate::generator::{scanner::{find_next_call, CallKind}, string_lexer::{extract_first_string, has_following_nonws_comma_before_paren, find_call_closing_paren}};
use crate::tests_writer::utility::{self, Category, ExtractedTest};

pub fn extract_tests_facade(content: &str, methods: &[(usize, String)], skip_diagnostics: bool) -> Vec<ExtractedTest> {
    let mut out = Vec::new();
    let mut cursor: usize = 0;
    loop {
        let next = find_next_call(content, cursor);
        let Some(hit) = next else { break; };
        if let Some((literal, end_idx)) = extract_first_string(content, hit.start_args) {
            let has_more_args = has_following_nonws_comma_before_paren(content, end_idx);
            if skip_diagnostics && has_more_args {
                cursor = end_idx + 1;
                continue;
            }
            let method_name = utility::find_enclosing_method_name(methods, hit.call_pos);
            let mut expected_diag_count: Option<usize> = None;
            if has_more_args {
                let open_paren_idx = hit.start_args.saturating_sub(1);
                if let Some(close_paren_idx) = find_call_closing_paren(content, open_paren_idx) {
                    let args_slice = &content[end_idx..=close_paren_idx];
                    let count = crate::generator::roslyn_args::count_diagnostic_invocations(args_slice);
                    if count > 0 { expected_diag_count = Some(count); }
                }
            }
            let category = map_kind_to_category(hit.kind);
            out.push(ExtractedTest { category, method_name, code: literal, expected_diag_count });
            cursor = end_idx + 1;
        } else {
            cursor = hit.start_args + 1;
        }
    }
    out
}

fn map_kind_to_category(kind: CallKind) -> Category {
    match kind {
        CallKind::UsingTree | CallKind::ParseCompilationUnit | CallKind::ParseTree => Category::Tree,
        CallKind::UsingStatement | CallKind::ParseStatement => Category::Statement,
        CallKind::UsingDeclaration | CallKind::ParseMemberDeclaration => Category::Declaration,
        CallKind::UsingExpression | CallKind::ParseExpression => Category::Expression,
        CallKind::ParseName => Category::Declaration, // will be wrapped as a type occurrence
        CallKind::ParseTypeName => Category::Declaration, // wrapped as type occurrence
        CallKind::ParseParameterList => Category::Declaration, // wrapped as method signature
        CallKind::ParseAttributeList => Category::Declaration, // wrapped as method attribute list
    }
}
