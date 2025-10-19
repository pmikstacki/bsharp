// Integration tests for switch_statement_parser.rs
// Content moved from src/parser/statements/switch_statement_parser.rs

use syntax::expressions::expression::Expression;

use parser::expressions::statements::switch_statement_parser::parse_switch_statement;
use parser::syntax::test_helpers::parse_all;
use syntax::identifier::Identifier;
use syntax::statements::statement::Statement;
use syntax::statements::switch_label::SwitchLabel;

#[test]
fn test_parse_switch_statement() {
    let input = "switch (x) { case 1: DoCase1(); break; case 2: { DoCase2(); } default: DoDefault(); break; }";
    let result = parse_all(parse_switch_statement, input.into());
    assert!(result.is_ok());
    match result.unwrap().1 {
        Statement::Switch(ss) => {
            assert_eq!(ss.expression, Expression::Variable(Identifier::new("x")));
            assert_eq!(ss.sections.len(), 3);
            // Section 1
            assert_eq!(ss.sections[0].labels.len(), 1);
            assert!(matches!(ss.sections[0].labels[0], SwitchLabel::Case(_)));
            assert_eq!(ss.sections[0].statements.len(), 2);
            // Section 2
            assert_eq!(ss.sections[1].labels.len(), 1);
            assert!(matches!(ss.sections[1].labels[0], SwitchLabel::Case(_)));
            assert_eq!(ss.sections[1].statements.len(), 1); // Block counts as one statement
            // Section 3
            assert_eq!(ss.sections[2].labels.len(), 1);
            assert!(matches!(ss.sections[2].labels[0], SwitchLabel::Default));
            assert_eq!(ss.sections[2].statements.len(), 2);
        }
        _ => panic!("Expected Switch statement"),
    }

    let input_empty = "switch(y) {}";
    let result_empty = parse_all(parse_switch_statement, input_empty.into());
    assert!(result_empty.is_ok());
    match result_empty.unwrap().1 {
        Statement::Switch(ss) => {
            assert_eq!(ss.expression, Expression::Variable(Identifier::new("y")));
            assert!(ss.sections.is_empty());
        }
        _ => panic!("Expected Switch statement"),
    }

    let input_fallthrough = "switch(z) { case 0: case 1: DoZeroOrOne(); break; }";
    let result_fallthrough = parse_all(parse_switch_statement, input_fallthrough.into());
    assert!(result_fallthrough.is_ok());
    match result_fallthrough.unwrap().1 {
        Statement::Switch(ss) => {
            assert_eq!(ss.sections.len(), 1);
            assert_eq!(ss.sections[0].labels.len(), 2);
            assert!(matches!(ss.sections[0].labels[0], SwitchLabel::Case(_)));
            assert!(matches!(ss.sections[0].labels[1], SwitchLabel::Case(_)));
            assert_eq!(ss.sections[0].statements.len(), 2);
        }
        _ => panic!("Expected Switch statement"),
    }
}
