use bsharp_vm_ir::lowering::entry::lower_compilation_unit;
use bsharp_vm_ir::{IrModule, IrInstr};
use bsharp_syntax::ast::CompilationUnit;
use bsharp_syntax::expressions::Expression;
use bsharp_syntax::expressions::literal::Literal;
use bsharp_syntax::statements::statement::Statement;

#[test]
fn lowers_literals_and_return() {
    // Build a tiny CU: { 1; return 2; }
    let cu = CompilationUnit {
        global_attributes: vec![],
        using_directives: vec![],
        global_using_directives: vec![],
        declarations: vec![],
        file_scoped_namespace: None,
        top_level_statements: vec![
            Statement::Expression(Expression::Literal(Literal::Integer(1))),
            Statement::Return(Some(Box::new(Expression::Literal(Literal::Integer(2))))),
        ],
    };

    let ir = lower_compilation_unit(&cu).expect("lowering should succeed");

    // Basic shape assertions
    assert_eq!(ir.functions.len(), 1, "one entry function expected");
    let f = &ir.functions[0];
    assert!(f.flags.is_script_entry, "entry function should be flagged as script entry");
    assert!(!f.blocks.is_empty(), "entry function should have at least one block");

    // Ensure we used at least one constant (from literals)
    assert!(!ir.constants.is_empty(), "should materialize at least one constant");
}
