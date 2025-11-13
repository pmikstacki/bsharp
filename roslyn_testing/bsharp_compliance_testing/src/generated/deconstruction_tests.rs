// Auto-generated from Roslyn: DeconstructionTests
/// Roslyn: DeconstructionTests.ParenExpression (case 1)
#[test]
fn paren_expression() {
    let src = r#"
class C
{
    void Goo()
    {
        (x).ToString();
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "ParenExpression",
                    1,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "ParenExpression",
                    1,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "ParenExpression",
            1,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.TupleTypeWithElementNames (case 2)
#[test]
fn tuple_type_with_element_names() {
    let src = r#"
class C
{
    void Goo()
    {
        (Int32 a, Int64 b) x;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "TupleTypeWithElementNames",
                    2,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "TupleTypeWithElementNames",
                    2,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "TupleTypeWithElementNames",
            2,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.TupleType (case 3)
#[test]
fn tuple_type() {
    let src = r#"
class C
{
    void Goo()
    {
        (Int32, Int64) x;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "TupleType",
                    3,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "TupleType",
                    3,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "TupleType",
            3,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.TupleTypeArray (case 4)
#[test]
fn tuple_type_array() {
    let src = r#"
class C
{
    void Goo()
    {
        (Int32, Int64)[] x;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "TupleTypeArray",
                    4,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "TupleTypeArray",
                    4,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "TupleTypeArray",
            4,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.TupleLiteral (case 5)
#[test]
fn tuple_literal() {
    let src = r#"
class C
{
    void Goo()
    {
        (Int32, Int64).Goo();
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "TupleLiteral",
                    5,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "TupleLiteral",
                    5,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "TupleLiteral",
            5,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.DeconstructionAssignment (case 6)
#[test]
fn deconstruction_assignment() {
    let src = r#"
class C
{
    void Goo()
    {
        (x, y) = goo;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DeconstructionAssignment",
                    6,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DeconstructionAssignment",
                    6,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "DeconstructionAssignment",
            6,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.SimpleDeclaration (case 7)
#[test]
fn simple_declaration() {
    let src = r#"
class C
{
    void Goo()
    {
        for(Int32 x = goo; ; ) { }
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "SimpleDeclaration",
                    7,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "SimpleDeclaration",
                    7,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "SimpleDeclaration",
            7,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.NestedDeconstructionAssignment (case 8)
#[test]
fn nested_deconstruction_assignment() {
    let src = r#"
class C
{
    void Goo()
    {
        (x, (y, z)) = goo;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "NestedDeconstructionAssignment",
                    8,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "NestedDeconstructionAssignment",
                    8,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "NestedDeconstructionAssignment",
            8,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.DeconstructionDeclaration (case 9)
#[test]
fn deconstruction_declaration() {
    let src = r#"
class C
{
    void Goo()
    {
        (Int32 a, Int64 b) = goo;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DeconstructionDeclaration",
                    9,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DeconstructionDeclaration",
                    9,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "DeconstructionDeclaration",
            9,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.NestedDeconstructionDeclaration (case 10)
#[test]
fn nested_deconstruction_declaration() {
    let src = r#"
class C
{
    void Goo()
    {
        ((Int32 a, Int64 b), Int32 c) = goo;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "NestedDeconstructionDeclaration",
                    10,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "NestedDeconstructionDeclaration",
                    10,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "NestedDeconstructionDeclaration",
            10,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.VarDeconstructionDeclaration (case 11)
#[test]
fn var_deconstruction_declaration() {
    let src = r#"
class C
{
    void Goo()
    {
        var (a, b) = goo;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "VarDeconstructionDeclaration",
                    11,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "VarDeconstructionDeclaration",
                    11,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "VarDeconstructionDeclaration",
            11,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.VarNestedDeconstructionDeclaration (case 12)
#[test]
fn var_nested_deconstruction_declaration() {
    let src = r#"
        class C
        {
            void Goo()
            {
                var ((a, b), c) = goo;
            }
        }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "VarNestedDeconstructionDeclaration",
                    12,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "VarNestedDeconstructionDeclaration",
                    12,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "VarNestedDeconstructionDeclaration",
            12,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.VarMethodCall (case 13)
#[test]
fn var_method_call() {
    let src = r#"
class C
{
    void Goo()
    {
        var(a, b);
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "VarMethodCall",
                    13,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "VarMethodCall",
                    13,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "VarMethodCall",
            13,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.MixedDeconstructionVariables (case 14)
#[test]
fn mixed_deconstruction_variables() {
    let src = r#"
class C
{
    void Goo()
    {
        (Int32 x, var (y, z)) = goo;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "MixedDeconstructionVariables",
                    14,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "MixedDeconstructionVariables",
                    14,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "MixedDeconstructionVariables",
            14,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.DeconstructionFor (case 15)
#[test]
fn deconstruction_for() {
    let src = r#"
        class C
        {
            void Goo()
            {
                for ((Int32 x, Int64 y) = goo; ; ) { }
            }
        }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DeconstructionFor",
                    15,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DeconstructionFor",
                    15,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "DeconstructionFor",
            15,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.VarDeconstructionFor (case 16)
#[test]
fn var_deconstruction_for() {
    let src = r#"
        class C
        {
            void Goo()
            {
                for (var (x, y) = goo; ; ) { }
            }
        }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "VarDeconstructionFor",
                    16,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "VarDeconstructionFor",
                    16,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "VarDeconstructionFor",
            16,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.DeconstructionForeach (case 17)
#[test]
fn deconstruction_foreach() {
    let src = r#"
        class C
        {
            void Goo()
            {
                foreach ((int x, var y) in goo) { }
            }
        }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DeconstructionForeach",
                    17,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DeconstructionForeach",
                    17,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "DeconstructionForeach",
            17,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.VarDeconstructionForeach (case 18)
#[test]
fn var_deconstruction_foreach() {
    let src = r#"
        class C
        {
            void Goo()
            {
                foreach (var (x, y) in goo) { }
            }
        }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "VarDeconstructionForeach",
                    18,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "VarDeconstructionForeach",
                    18,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "VarDeconstructionForeach",
            18,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.DeconstructionInScript (case 19)
#[test]
fn deconstruction_in_script() {
    let src = r#" (int x, int y) = (1, 2); "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DeconstructionInScript",
                    19,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DeconstructionInScript",
                    19,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "DeconstructionInScript",
            19,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.DeconstructionForEachInScript (case 20)
#[test]
fn deconstruction_for_each_in_script() {
    let src = r#" foreach ((int x, int y) in new[] { (1, 2) }) { }; "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DeconstructionForEachInScript",
                    20,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DeconstructionForEachInScript",
                    20,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "DeconstructionForEachInScript",
            20,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.DeconstructionDeclarationWithDiscard (case 21)
#[test]
fn deconstruction_declaration_with_discard() {
    let src = r#"
class C
{
    void Goo()
    {
        (int _, var _, var (_, _), _) = e;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DeconstructionDeclarationWithDiscard",
                    21,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DeconstructionDeclarationWithDiscard",
                    21,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "DeconstructionDeclarationWithDiscard",
            21,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.DiscardsInDeconstruction_01 (case 22)
#[test]
fn discards_in_deconstruction_01() {
    let src = r#"void M() { var (x, _) = e; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DiscardsInDeconstruction_01",
                    22,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DiscardsInDeconstruction_01",
                    22,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "DiscardsInDeconstruction_01",
            22,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.DiscardsInDeconstruction_02 (case 23)
#[test]
fn discards_in_deconstruction_02() {
    let src = r#"void M() { (var x, var _) = e; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DiscardsInDeconstruction_02",
                    23,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DiscardsInDeconstruction_02",
                    23,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "DiscardsInDeconstruction_02",
            23,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.DiscardsInOut_01 (case 24)
#[test]
fn discards_in_out_01() {
    let src = r#"void M() { M(out var _); }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DiscardsInOut_01",
                    24,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DiscardsInOut_01",
                    24,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "DiscardsInOut_01",
            24,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.DiscardsInOut_02 (case 25)
#[test]
fn discards_in_out_02() {
    let src = r#"void M() { M(out int _); }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DiscardsInOut_02",
                    25,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DiscardsInOut_02",
                    25,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "DiscardsInOut_02",
            25,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.DiscardsInPattern_01 (case 26)
#[test]
fn discards_in_pattern_01() {
    let src = r#"void M() { if (e is int _) {} }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DiscardsInPattern_01",
                    26,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DiscardsInPattern_01",
                    26,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "DiscardsInPattern_01",
            26,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.DiscardsInPattern_02 (case 27)
#[test]
fn discards_in_pattern_02() {
    let src = r#"void M() { if (e is var _) {} }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DiscardsInPattern_02",
                    27,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DiscardsInPattern_02",
                    27,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "DiscardsInPattern_02",
            27,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.DiscardsInPattern_03 (case 28)
#[test]
fn discards_in_pattern_03() {
    let src = r#"void M() { switch (e) { case int _: break; } }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DiscardsInPattern_03",
                    28,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DiscardsInPattern_03",
                    28,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "DiscardsInPattern_03",
            28,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.DiscardsInPattern_04 (case 29)
#[test]
fn discards_in_pattern_04() {
    let src = r#"void M() { switch (e) { case var _: break; } }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DiscardsInPattern_04",
                    29,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "DiscardsInPattern_04",
                    29,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "DiscardsInPattern_04",
            29,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: DeconstructionTests.BadTypeForDeconstruct_00 (case 30)
#[test]
fn bad_type_for_deconstruct_00() {
    let src = r#"var (x, y) = e;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "BadTypeForDeconstruct_00",
                    30,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "BadTypeForDeconstruct_00",
                    30,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "BadTypeForDeconstruct_00",
            30,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeconstructionTests.BadTypeForDeconstruct_01 (case 31)
#[test]
fn bad_type_for_deconstruct_01() {
    let src = r#"var::var (x, y) = e;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "BadTypeForDeconstruct_01",
                    31,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "BadTypeForDeconstruct_01",
                    31,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "BadTypeForDeconstruct_01",
            31,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeconstructionTests.BadTypeForDeconstruct_02 (case 32)
#[test]
fn bad_type_for_deconstruct_02() {
    let src = r#"var.var (x, y) = e;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "BadTypeForDeconstruct_02",
                    32,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "BadTypeForDeconstruct_02",
                    32,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "BadTypeForDeconstruct_02",
            32,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeconstructionTests.BadTypeForDeconstruct_03 (case 33)
#[test]
fn bad_type_for_deconstruct_03() {
    let src = r#"var<var> (x, y) = e;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "BadTypeForDeconstruct_03",
                    33,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "BadTypeForDeconstruct_03",
                    33,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "BadTypeForDeconstruct_03",
            33,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeconstructionTests.BadTypeForDeconstruct_04 (case 34)
#[test]
fn bad_type_for_deconstruct_04() {
    let src = r#"var[] (x, y) = e;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "BadTypeForDeconstruct_04",
                    34,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "BadTypeForDeconstruct_04",
                    34,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "BadTypeForDeconstruct_04",
            34,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeconstructionTests.BadTypeForDeconstruct_05 (case 35)
#[test]
fn bad_type_for_deconstruct_05() {
    let src = r#"var* (x, y) = e;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "BadTypeForDeconstruct_05",
                    35,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "BadTypeForDeconstruct_05",
                    35,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "BadTypeForDeconstruct_05",
            35,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeconstructionTests.BadTypeForDeconstruct_06 (case 36)
#[test]
fn bad_type_for_deconstruct_06() {
    let src = r#"var? (x, y) = e;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "BadTypeForDeconstruct_06",
                    36,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "BadTypeForDeconstruct_06",
                    36,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "BadTypeForDeconstruct_06",
            36,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeconstructionTests.BadTypeForDeconstruct_07 (case 37)
#[test]
fn bad_type_for_deconstruct_07() {
    let src = r#"var?.var (x, y) = e;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "BadTypeForDeconstruct_07",
                    37,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "deconstruction_tests",
                    "DeconstructionTests",
                    "BadTypeForDeconstruct_07",
                    37,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "deconstruction_tests",
            "DeconstructionTests",
            "BadTypeForDeconstruct_07",
            37,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}
