// Auto-generated from Roslyn: TypeArgumentListParsingTests
/// Roslyn: TypeArgumentListParsingTests.TestPredefinedType (case 1)
#[test]
fn predefined_type() {
    let src = r#"
class C
{
    void M()
    {
        var added = ImmutableDictionary<string, IImmutableDictionary<X, Y>>

        ProjectChange = projectChange;
    }
}
"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestPredefinedType",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestPredefinedType",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestPredefinedType",
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

/// Roslyn: TypeArgumentListParsingTests.TestArrayType (case 2)
#[test]
fn array_type() {
    let src = r#"
class C
{
    void M()
    {
        var added = ImmutableDictionary<X[], IImmutableDictionary<X, Y>>

        ProjectChange = projectChange;
    }
}
"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestArrayType",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestArrayType",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestArrayType",
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

/// Roslyn: TypeArgumentListParsingTests.TestPredefinedPointerType (case 3)
#[test]
fn predefined_pointer_type() {
    let src = r#"
class C
{
    void M()
    {
        var added = ImmutableDictionary<int*, IImmutableDictionary<X, Y>>

        ProjectChange = projectChange;
    }
}
"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestPredefinedPointerType",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestPredefinedPointerType",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestPredefinedPointerType",
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

/// Roslyn: TypeArgumentListParsingTests.TestNonPredefinedPointerType (case 4)
#[test]
fn non_predefined_pointer_type() {
    let src = r#"
class C
{
    void M()
    {
        var added = ImmutableDictionary<X*, IImmutableDictionary<X, Y>>

        ProjectChange = projectChange;
    }
}
"#;
    let expected = Some(ExpectedDiagnostics {
        count: 5,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestNonPredefinedPointerType",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestNonPredefinedPointerType",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestNonPredefinedPointerType",
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

/// Roslyn: TypeArgumentListParsingTests.TestTwoItemTupleType (case 5)
#[test]
fn two_item_tuple_type() {
    let src = r#"
class C
{
    void M()
    {
        var added = ImmutableDictionary<(int, string), IImmutableDictionary<X, Y>>

        ProjectChange = projectChange;
    }
}
"#;
    let expected = Some(ExpectedDiagnostics {
        count: 6,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestTwoItemTupleType",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestTwoItemTupleType",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestTwoItemTupleType",
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

/// Roslyn: TypeArgumentListParsingTests.TestComparisonToTuple (case 6)
#[test]
fn comparison_to_tuple() {
    let src = r#"
public class C
{
    public static void Main()
    {
        XX X = new XX();
        int a = 1, b = 2;
        bool z = X < (a, b), w = false;
    }
}

struct XX
{
    public static bool operator <(XX x, (int a, int b) arg) => true;
    public static bool operator >(XX x, (int a, int b) arg) => false;
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestComparisonToTuple",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestComparisonToTuple",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestComparisonToTuple",
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

/// Roslyn: TypeArgumentListParsingTests.TestOneItemTupleType (case 7)
#[test]
fn one_item_tuple_type() {
    let src = r#"
class C
{
    void M()
    {
        var added = ImmutableDictionary<(A), IImmutableDictionary<X, Y>>

        ProjectChange = projectChange;
    }
}
"#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestOneItemTupleType",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestOneItemTupleType",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestOneItemTupleType",
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

/// Roslyn: TypeArgumentListParsingTests.TestQualifiedName (case 8)
#[test]
fn qualified_name() {
    let src = r#"
class C
{
    void M()
    {
        var added = ImmutableDictionary<A.B, IImmutableDictionary<X, Y>>

        ProjectChange = projectChange;
    }
}
"#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestQualifiedName",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestQualifiedName",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestQualifiedName",
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

/// Roslyn: TypeArgumentListParsingTests.TestAliasName (case 9)
#[test]
fn alias_name() {
    let src = r#"
class C
{
    void M()
    {
        var added = ImmutableDictionary<A::B, IImmutableDictionary<X, Y>>

        ProjectChange = projectChange;
    }
}
"#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestAliasName",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestAliasName",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestAliasName",
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

/// Roslyn: TypeArgumentListParsingTests.TestNullableTypeWithComma (case 10)
#[test]
fn nullable_type_with_comma() {
    let src = r#"
class C
{
    void M()
    {
        var added = ImmutableDictionary<A?, IImmutableDictionary<X, Y>>

        ProjectChange = projectChange;
    }
}
"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestNullableTypeWithComma",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestNullableTypeWithComma",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestNullableTypeWithComma",
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

/// Roslyn: TypeArgumentListParsingTests.TestNullableTypeWithGreaterThan (case 11)
#[test]
fn nullable_type_with_greater_than() {
    let src = r#"
class C
{
    void M()
    {
        var added = ImmutableDictionary<A?>

        ProjectChange = projectChange;
    }
}
"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestNullableTypeWithGreaterThan",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestNullableTypeWithGreaterThan",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestNullableTypeWithGreaterThan",
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

/// Roslyn: TypeArgumentListParsingTests.TestNotNullableType (case 12)
#[test]
fn not_nullable_type() {
    let src = r#"
class C
{
    void M()
    {
        var added = ImmutableDictionary<A?

        ProjectChange = projectChange;
    }
}
"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestNotNullableType",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestNotNullableType",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestNotNullableType",
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

/// Roslyn: TypeArgumentListParsingTests.TestGenericArgWithComma_01 (case 13)
#[test]
fn generic_arg_with_comma_01() {
    let src = r#"
class C
{
    void M()
    {
        var added = ImmutableDictionary<T<S>, IImmutableDictionary<X, Y>>

        ProjectChange = projectChange;
    }
}
"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericArgWithComma_01",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericArgWithComma_01",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestGenericArgWithComma_01",
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

/// Roslyn: TypeArgumentListParsingTests.TestGenericArgWithComma_02 (case 14)
#[test]
fn generic_arg_with_comma_02() {
    let src = r#"
class C
{
    void M()
    {
        var added = U<ImmutableDictionary<T<S>, IImmutableDictionary<X, Y>>>

        ProjectChange = projectChange;
    }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericArgWithComma_02",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericArgWithComma_02",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestGenericArgWithComma_02",
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

/// Roslyn: TypeArgumentListParsingTests.TestGenericArgWithComma_03 (case 15)
#[test]
fn generic_arg_with_comma_03() {
    let src = r#"
class C
{
    void M()
    {
        var added = ImmutableDictionary<T<S>, U<IImmutableDictionary<X, Y>>>

        ProjectChange = projectChange;
    }
}
"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericArgWithComma_03",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericArgWithComma_03",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestGenericArgWithComma_03",
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

/// Roslyn: TypeArgumentListParsingTests.TestGenericArgWithComma_04 (case 16)
#[test]
fn generic_arg_with_comma_04() {
    let src = r#"
class C
{
    void M()
    {
        var added = ImmutableDictionary<T<S>, IImmutableDictionary<X, U<Y>>>

        ProjectChange = projectChange;
    }
}
"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericArgWithComma_04",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericArgWithComma_04",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestGenericArgWithComma_04",
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

/// Roslyn: TypeArgumentListParsingTests.TestGenericArgWithGreaterThan_01 (case 17)
#[test]
fn generic_arg_with_greater_than_01() {
    let src = r#"
class C
{
    void M()
    {
        var added = ImmutableDictionary<T<S>>

        ProjectChange = projectChange;
    }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericArgWithGreaterThan_01",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericArgWithGreaterThan_01",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestGenericArgWithGreaterThan_01",
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

/// Roslyn: TypeArgumentListParsingTests.TestGenericArgWithGreaterThan_02 (case 18)
#[test]
fn generic_arg_with_greater_than_02() {
    let src = r#"
class C
{
    void M()
    {
        var added = ImmutableDictionary<U<T<S>>>

        ProjectChange = projectChange;
    }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericArgWithGreaterThan_02",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericArgWithGreaterThan_02",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestGenericArgWithGreaterThan_02",
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

/// Roslyn: TypeArgumentListParsingTests.TestGenericArgWithGreaterThan_03 (case 19)
#[test]
fn generic_arg_with_greater_than_03() {
    let src = r#"
class C
{
    void M()
    {
        var added = ImmutableDictionary<T<S>>>

        ProjectChange = projectChange;
    }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericArgWithGreaterThan_03",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericArgWithGreaterThan_03",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestGenericArgWithGreaterThan_03",
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

/// Roslyn: TypeArgumentListParsingTests.TestGenericArgWithGreaterThan_04 (case 20)
#[test]
fn generic_arg_with_greater_than_04() {
    let src = r#"
class C
{
    void M()
    {
        var added = ImmutableDictionary<T<(S, U)>>>

        ProjectChange = projectChange;
    }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericArgWithGreaterThan_04",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericArgWithGreaterThan_04",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestGenericArgWithGreaterThan_04",
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

/// Roslyn: TypeArgumentListParsingTests.TestGenericArgWithGreaterThan_05 (case 21)
#[test]
fn generic_arg_with_greater_than_05() {
    let src = r#"
class C
{
    void M()
    {
        var added = ImmutableDictionary<T<(S a, U b)>>>

        ProjectChange = projectChange;
    }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericArgWithGreaterThan_05",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericArgWithGreaterThan_05",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestGenericArgWithGreaterThan_05",
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

/// Roslyn: TypeArgumentListParsingTests.TestGenericWithExtraCommasAndMissingTypes1 (case 22)
#[test]
fn generic_with_extra_commas_and_missing_types_1() {
    let src = r#"
                class C
                {
                    void M()
                    {
                        var added = Goo<string,,>.Instance;
                    }
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericWithExtraCommasAndMissingTypes1",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericWithExtraCommasAndMissingTypes1",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestGenericWithExtraCommasAndMissingTypes1",
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

/// Roslyn: TypeArgumentListParsingTests.TestGenericWithExtraCommasAndMissingTypes2 (case 23)
#[test]
fn generic_with_extra_commas_and_missing_types_2() {
    let src = r#"
                class C
                {
                    void M()
                    {
                        var added = Goo<Id,,>.Instance;
                    }
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericWithExtraCommasAndMissingTypes2",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericWithExtraCommasAndMissingTypes2",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestGenericWithExtraCommasAndMissingTypes2",
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

/// Roslyn: TypeArgumentListParsingTests.TestGenericWithExtraCommasAndMissingTypes3 (case 24)
#[test]
fn generic_with_extra_commas_and_missing_types_3() {
    let src = r#"
                class C
                {
                    void M()
                    {
                        var added = Goo<,Id,>.Instance;
                    }
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericWithExtraCommasAndMissingTypes3",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericWithExtraCommasAndMissingTypes3",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestGenericWithExtraCommasAndMissingTypes3",
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

/// Roslyn: TypeArgumentListParsingTests.TestGenericWithExtraCommasAndMissingTypes4 (case 25)
#[test]
fn generic_with_extra_commas_and_missing_types_4() {
    let src = r#"
                class C
                {
                    void M()
                    {
                        var added = Goo<,,Id>.Instance;
                    }
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericWithExtraCommasAndMissingTypes4",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericWithExtraCommasAndMissingTypes4",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestGenericWithExtraCommasAndMissingTypes4",
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

/// Roslyn: TypeArgumentListParsingTests.TestGenericWithExtraCommasAndMissingTypes5 (case 26)
#[test]
fn generic_with_extra_commas_and_missing_types_5() {
    let src = r#"
                class C
                {
                    void M()
                    {
                        var added = Goo<Id[],,>.Instance;
                    }
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericWithExtraCommasAndMissingTypes5",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericWithExtraCommasAndMissingTypes5",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestGenericWithExtraCommasAndMissingTypes5",
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

/// Roslyn: TypeArgumentListParsingTests.TestGenericWithExtraCommasAndMissingTypes6 (case 27)
#[test]
fn generic_with_extra_commas_and_missing_types_6() {
    let src = r#"
                class C
                {
                    void M()
                    {
                        var added = Goo<(int i, int j),,>.Instance;
                    }
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericWithExtraCommasAndMissingTypes6",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericWithExtraCommasAndMissingTypes6",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestGenericWithExtraCommasAndMissingTypes6",
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

/// Roslyn: TypeArgumentListParsingTests.TestGenericWithExtraCommasAndMissingTypes7 (case 28)
#[test]
fn generic_with_extra_commas_and_missing_types_7() {
    let src = r#"
                class C
                {
                    void M()
                    {
                        var added = Goo<K<int>,,>.Instance;
                    }
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericWithExtraCommasAndMissingTypes7",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericWithExtraCommasAndMissingTypes7",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestGenericWithExtraCommasAndMissingTypes7",
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

/// Roslyn: TypeArgumentListParsingTests.TestGenericWithExtraCommasAndMissingTypes8 (case 29)
#[test]
fn generic_with_extra_commas_and_missing_types_8() {
    let src = r#"
                class C
                {
                    void M()
                    {
                        var added = Goo<K<int,,>,,>.Instance;
                    }
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericWithExtraCommasAndMissingTypes8",
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
                    "type_argument_list_parsing_tests",
                    "TypeArgumentListParsingTests",
                    "TestGenericWithExtraCommasAndMissingTypes8",
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
            "type_argument_list_parsing_tests",
            "TypeArgumentListParsingTests",
            "TestGenericWithExtraCommasAndMissingTypes8",
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
