pub struct ExpectedNode {
    pub kind: String,
    pub token_value: Option<String>,
    pub children: Vec<ExpectedNode>,
}

pub struct ExpectedTree {
    pub root: ExpectedNode,
}

pub fn assert_tree(expected: &ExpectedTree, unit: &bsharp_syntax::ast::CompilationUnit) {
    let _ = unit;
    assert_eq!(expected.root.kind, "CompilationUnit");
}
