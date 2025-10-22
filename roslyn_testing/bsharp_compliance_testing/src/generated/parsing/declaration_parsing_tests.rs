// Auto-generated STRUCTURE tests from Roslyn: DeclarationParsingTests
use bsharp_parser::syntax::span::Span;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use crate::custom_asserts::structure_assert;
#[test]
fn cs_0071_01() {
    let src = r#"
unsafe struct s
{
    public fixed bool _Type1[10]; 
    public fixed byte _Type12[10]; 
    public fixed int _Type2[10]; 
    public fixed short _Type3[10]; 
    public fixed long _Type4[10]; 
    public fixed char _Type5[10]; 
    public fixed sbyte _Type6[10]; 
    public fixed ushort _Type7[10]; 
    public fixed uint _Type8[10]; 
    public fixed ulong _Type9[10]; 
    public fixed float _Type10[10]; 
    public fixed double _Type11[10];     
 }


"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "InterfaceDeclaration".to_string(), token_value: None, children: vec![] },     structure_assert::ExpectedNode { kind: "InterfaceDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "EventDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "QualifiedName".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("System".to_string()), children: vec![] },                 structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("Action".to_string()), children: vec![] }] },             structure_assert::ExpectedNode { kind: "ExplicitInterfaceSpecifier".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("I2".to_string()), children: vec![] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn cs_0071_02() {
    let src = r#"
unsafe struct s
{
    public fixed bool _Type1[10]; 
    public fixed byte _Type12[10]; 
    public fixed int _Type2[10]; 
    public fixed short _Type3[10]; 
    public fixed long _Type4[10]; 
    public fixed char _Type5[10]; 
    public fixed sbyte _Type6[10]; 
    public fixed ushort _Type7[10]; 
    public fixed uint _Type8[10]; 
    public fixed ulong _Type9[10]; 
    public fixed float _Type10[10]; 
    public fixed double _Type11[10];     
 }


"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "InterfaceDeclaration".to_string(), token_value: None, children: vec![] },     structure_assert::ExpectedNode { kind: "InterfaceDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "EventDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "QualifiedName".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("System".to_string()), children: vec![] },                 structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("Action".to_string()), children: vec![] }] },             structure_assert::ExpectedNode { kind: "ExplicitInterfaceSpecifier".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("I2".to_string()), children: vec![] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn cs_0071_03() {
    let src = r#"
unsafe struct s
{
    public fixed bool _Type1[10]; 
    public fixed byte _Type12[10]; 
    public fixed int _Type2[10]; 
    public fixed short _Type3[10]; 
    public fixed long _Type4[10]; 
    public fixed char _Type5[10]; 
    public fixed sbyte _Type6[10]; 
    public fixed ushort _Type7[10]; 
    public fixed uint _Type8[10]; 
    public fixed ulong _Type9[10]; 
    public fixed float _Type10[10]; 
    public fixed double _Type11[10];     
 }


"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "InterfaceDeclaration".to_string(), token_value: None, children: vec![] },     structure_assert::ExpectedNode { kind: "InterfaceDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "EventDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "QualifiedName".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("System".to_string()), children: vec![] },                 structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("Action".to_string()), children: vec![] }] },             structure_assert::ExpectedNode { kind: "ExplicitInterfaceSpecifier".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("I2".to_string()), children: vec![] }] }] }] },     structure_assert::ExpectedNode { kind: "IncompleteMember".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("P10".to_string()), children: vec![] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn cs_0071_04() {
    let src = r#"
unsafe struct s
{
    public fixed bool _Type1[10]; 
    public fixed byte _Type12[10]; 
    public fixed int _Type2[10]; 
    public fixed short _Type3[10]; 
    public fixed long _Type4[10]; 
    public fixed char _Type5[10]; 
    public fixed sbyte _Type6[10]; 
    public fixed ushort _Type7[10]; 
    public fixed uint _Type8[10]; 
    public fixed ulong _Type9[10]; 
    public fixed float _Type10[10]; 
    public fixed double _Type11[10];     
 }


"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "InterfaceDeclaration".to_string(), token_value: None, children: vec![] },     structure_assert::ExpectedNode { kind: "InterfaceDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "EventDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "QualifiedName".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("System".to_string()), children: vec![] },                 structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("Action".to_string()), children: vec![] }] },             structure_assert::ExpectedNode { kind: "ExplicitInterfaceSpecifier".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("I2".to_string()), children: vec![] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn non_accessor_after_incomplete_property() {
    let src = r#"
unsafe struct s
{
    public fixed bool _Type1[10]; 
    public fixed byte _Type12[10]; 
    public fixed int _Type2[10]; 
    public fixed short _Type3[10]; 
    public fixed long _Type4[10]; 
    public fixed char _Type5[10]; 
    public fixed sbyte _Type6[10]; 
    public fixed ushort _Type7[10]; 
    public fixed uint _Type8[10]; 
    public fixed ulong _Type9[10]; 
    public fixed float _Type10[10]; 
    public fixed double _Type11[10];     
 }


"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("C".to_string()), children: vec![] },         structure_assert::ExpectedNode { kind: "PropertyDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "AccessorList".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "GetAccessorDeclaration".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "ReturnStatement".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "SimpleMemberAccessExpression".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "ThisExpression".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] }] },     structure_assert::ExpectedNode { kind: "FieldDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "VariableDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "VariableDeclarator".to_string(), token_value: None, children: vec![] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn expression_bodied_ctor_dtor_prop() {
    let src = r#"
class C1
{
    static (T, T) Test3<T>((byte, byte) arg0)
    {
        return default((T, T));
    }

    (T, T) Test3<T>((byte a, byte b)[] arg0)
    {
        return default((T, T));
    }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "ConstructorDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "BaseConstructorInitializer".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "ArgumentList".to_string(), token_value: None, children: vec![] }] },             structure_assert::ExpectedNode { kind: "ArrowExpressionClause".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "InvocationExpression".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: None, children: vec![] },                     structure_assert::ExpectedNode { kind: "ArgumentList".to_string(), token_value: None, children: vec![] }] }] }] },         structure_assert::ExpectedNode { kind: "ConstructorDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "ArrowExpressionClause".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "InvocationExpression".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: None, children: vec![] },                     structure_assert::ExpectedNode { kind: "ArgumentList".to_string(), token_value: None, children: vec![] }] }] }] },         structure_assert::ExpectedNode { kind: "DestructorDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "ArrowExpressionClause".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "InvocationExpression".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: None, children: vec![] },                     structure_assert::ExpectedNode { kind: "ArgumentList".to_string(), token_value: None, children: vec![] }] }] }] },         structure_assert::ExpectedNode { kind: "PropertyDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "AccessorList".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "SetAccessorDeclaration".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "ArrowExpressionClause".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "InvocationExpression".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: None, children: vec![] },                             structure_assert::ExpectedNode { kind: "ArgumentList".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] },     structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("C".to_string()), children: vec![] },             structure_assert::ExpectedNode { kind: "MethodDeclaration".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },                 structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![] },                 structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "ExpressionStatement".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "InvocationExpression".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("M".to_string()), children: vec![] },                             structure_assert::ExpectedNode { kind: "ArgumentList".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "DeclarationExpression".to_string(), token_value: None, children: vec![                                        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("var".to_string()), children: vec![] },                                         structure_assert::ExpectedNode { kind: "SingleVariableDesignation".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn parse_out_var() {
    let src = r#"
class C1
{
    static (T, T) Test3<T>((byte, byte) arg0)
    {
        return default((T, T));
    }

    (T, T) Test3<T>((byte a, byte b)[] arg0)
    {
        return default((T, T));
    }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("C".to_string()), children: vec![] },         structure_assert::ExpectedNode { kind: "MethodDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "ExpressionStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "InvocationExpression".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("M".to_string()), children: vec![] },                         structure_assert::ExpectedNode { kind: "ArgumentList".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "DeclarationExpression".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("var".to_string()), children: vec![] },                                     structure_assert::ExpectedNode { kind: "SingleVariableDesignation".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn partially_written_constraint_clause_in_base_list_1() {
    let src = r#"
class C1
{
    static (T, T) Test3<T>((byte, byte) arg0)
    {
        return default((T, T));
    }

    (T, T) Test3<T>((byte a, byte b)[] arg0)
    {
        return default((T, T));
    }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("C".to_string()), children: vec![] },         structure_assert::ExpectedNode { kind: "TypeParameterList".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "TypeParameter".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("T".to_string()), children: vec![] }] }] },         structure_assert::ExpectedNode { kind: "BaseList".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "SimpleBaseType".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("where".to_string()), children: vec![] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn partially_written_constraint_clause_in_base_list_2() {
    let src = r#"
class C1
{
    static (T, T) Test3<T>((byte, byte) arg0)
    {
        return default((T, T));
    }

    (T, T) Test3<T>((byte a, byte b)[] arg0)
    {
        return default((T, T));
    }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("C".to_string()), children: vec![] },         structure_assert::ExpectedNode { kind: "TypeParameterList".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "TypeParameter".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("T".to_string()), children: vec![] }] }] },         structure_assert::ExpectedNode { kind: "BaseList".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "SimpleBaseType".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("where".to_string()), children: vec![] }] },             structure_assert::ExpectedNode { kind: "SimpleBaseType".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("T".to_string()), children: vec![] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn partially_written_constraint_clause_in_base_list_3() {
    let src = r#"
class C1
{
    static (T, T) Test3<T>((byte, byte) arg0)
    {
        return default((T, T));
    }

    (T, T) Test3<T>((byte a, byte b)[] arg0)
    {
        return default((T, T));
    }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("C".to_string()), children: vec![] },         structure_assert::ExpectedNode { kind: "TypeParameterList".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "TypeParameter".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("T".to_string()), children: vec![] }] }] },         structure_assert::ExpectedNode { kind: "BaseList".to_string(), token_value: None, children: vec![] }] },     structure_assert::ExpectedNode { kind: "TypeParameterConstraintClause".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("T".to_string()), children: vec![] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn partially_written_constraint_clause_in_base_list_4() {
    let src = r#"
class C1
{
    static (T, T) Test3<T>((byte, byte) arg0)
    {
        return default((T, T));
    }

    (T, T) Test3<T>((byte a, byte b)[] arg0)
    {
        return default((T, T));
    }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("C".to_string()), children: vec![] },         structure_assert::ExpectedNode { kind: "TypeParameterList".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "TypeParameter".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("T".to_string()), children: vec![] }] }] },         structure_assert::ExpectedNode { kind: "BaseList".to_string(), token_value: None, children: vec![] }] },     structure_assert::ExpectedNode { kind: "TypeParameterConstraintClause".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("T".to_string()), children: vec![] },         structure_assert::ExpectedNode { kind: "SpecificType".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("X".to_string()), children: vec![] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn null_checked_method() {
    let src = r#"
class C1
{
    static (T, T) Test3<T>((byte, byte) arg0)
    {
        return default((T, T));
    }

    (T, T) Test3<T>((byte a, byte b)[] arg0)
    {
        return default((T, T));
    }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("C".to_string()), children: vec![] },         structure_assert::ExpectedNode { kind: "MethodDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "Parameter".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] }] }] },             structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![] }] }] },     structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("C".to_string()), children: vec![] },             structure_assert::ExpectedNode { kind: "ConstructorDeclaration".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "Parameter".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] }] }] },                 structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![] }] }] }] },     structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "OperatorDeclaration".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },                 structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "Parameter".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: None, children: vec![] }] },                     structure_assert::ExpectedNode { kind: "Parameter".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: None, children: vec![] }] },                     structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "ReturnStatement".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "NumericLiteralExpression".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] },     structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "DelegateDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "Parameter".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] }] }] }] },         structure_assert::ExpectedNode { kind: "GlobalStatement".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "LocalDeclarationStatement".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "VariableDeclaration".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("Del".to_string()), children: vec![] },                     structure_assert::ExpectedNode { kind: "VariableDeclarator".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "EqualsValueClause".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "AnonymousMethodExpression".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "Parameter".to_string(), token_value: None, children: vec![                                        structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] }] }] },                                 structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] }] },     structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("B".to_string()), children: vec![] },             structure_assert::ExpectedNode { kind: "BaseList".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "SimpleBaseType".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "GenericName".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "TypeArgumentList".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn null_checked_constructor() {
    let src = r#"
class C1
{
    static (T, T) Test3<T>((byte, byte) arg0)
    {
        return default((T, T));
    }

    (T, T) Test3<T>((byte a, byte b)[] arg0)
    {
        return default((T, T));
    }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("C".to_string()), children: vec![] },         structure_assert::ExpectedNode { kind: "ConstructorDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "Parameter".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] }] }] },             structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![] }] }] },     structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "OperatorDeclaration".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },                 structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "Parameter".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: None, children: vec![] }] },                     structure_assert::ExpectedNode { kind: "Parameter".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: None, children: vec![] }] },                     structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "ReturnStatement".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "NumericLiteralExpression".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] },     structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "DelegateDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "Parameter".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] }] }] }] },         structure_assert::ExpectedNode { kind: "GlobalStatement".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "LocalDeclarationStatement".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "VariableDeclaration".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("Del".to_string()), children: vec![] },                     structure_assert::ExpectedNode { kind: "VariableDeclarator".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "EqualsValueClause".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "AnonymousMethodExpression".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "Parameter".to_string(), token_value: None, children: vec![                                        structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] }] }] },                                 structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] }] },     structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("B".to_string()), children: vec![] },             structure_assert::ExpectedNode { kind: "BaseList".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "SimpleBaseType".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "GenericName".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "TypeArgumentList".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn null_checked_operator() {
    let src = r#"
class C1
{
    static (T, T) Test3<T>((byte, byte) arg0)
    {
        return default((T, T));
    }

    (T, T) Test3<T>((byte a, byte b)[] arg0)
    {
        return default((T, T));
    }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "OperatorDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "Parameter".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: None, children: vec![] }] },                 structure_assert::ExpectedNode { kind: "Parameter".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: None, children: vec![] }] },                 structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "ReturnStatement".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "NumericLiteralExpression".to_string(), token_value: None, children: vec![] }] }] }] }] }] },     structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "DelegateDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "Parameter".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] }] }] }] },         structure_assert::ExpectedNode { kind: "GlobalStatement".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "LocalDeclarationStatement".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "VariableDeclaration".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("Del".to_string()), children: vec![] },                     structure_assert::ExpectedNode { kind: "VariableDeclarator".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "EqualsValueClause".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "AnonymousMethodExpression".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "Parameter".to_string(), token_value: None, children: vec![                                        structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] }] }] },                                 structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] }] },     structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("B".to_string()), children: vec![] },             structure_assert::ExpectedNode { kind: "BaseList".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "SimpleBaseType".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "GenericName".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "TypeArgumentList".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn anonymous_delegate_null_checking() {
    let src = r#"
class C1
{
    static (T, T) Test3<T>((byte, byte) arg0)
    {
        return default((T, T));
    }

    (T, T) Test3<T>((byte a, byte b)[] arg0)
    {
        return default((T, T));
    }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "DelegateDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },         structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "Parameter".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] }] }] }] },     structure_assert::ExpectedNode { kind: "GlobalStatement".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "LocalDeclarationStatement".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "VariableDeclaration".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("Del".to_string()), children: vec![] },                 structure_assert::ExpectedNode { kind: "VariableDeclarator".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "EqualsValueClause".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "AnonymousMethodExpression".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "Parameter".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] }] }] },                             structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] },     structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("B".to_string()), children: vec![] },             structure_assert::ExpectedNode { kind: "BaseList".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "SimpleBaseType".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "GenericName".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "TypeArgumentList".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn interface_no_body() {
    let src = r#"
interface C"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "InterfaceDeclaration".to_string(), token_value: None, children: vec![] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn interface_semicolon_body() {
    let src = r#"
interface C
;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "InterfaceDeclaration".to_string(), token_value: None, children: vec![] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn interface_semicolon_body_after_base_01() {
    let src = r#"
interface C : I1
;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "InterfaceDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "BaseList".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "SimpleBaseType".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("I1".to_string()), children: vec![] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn interface_semicolon_body_after_base_02() {
    let src = r#"
interface C : I1, I2
;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "InterfaceDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "BaseList".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "SimpleBaseType".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("I1".to_string()), children: vec![] }] },             structure_assert::ExpectedNode { kind: "SimpleBaseType".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("I2".to_string()), children: vec![] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn interface_semicolon_body_after_constraint_01() {
    let src = r#"
interface C where T1 : U1
;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "InterfaceDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "TypeParameterConstraintClause".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("T1".to_string()), children: vec![] },             structure_assert::ExpectedNode { kind: "SpecificType".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("U1".to_string()), children: vec![] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn interface_semicolon_body_after_constraint_02() {
    let src = r#"
interface C where T1 : U1 where T2 : U2
;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "InterfaceDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "TypeParameterConstraintClause".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("T1".to_string()), children: vec![] },             structure_assert::ExpectedNode { kind: "SpecificType".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("U1".to_string()), children: vec![] }] }] },         structure_assert::ExpectedNode { kind: "TypeParameterConstraintClause".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("T2".to_string()), children: vec![] },             structure_assert::ExpectedNode { kind: "SpecificType".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("U2".to_string()), children: vec![] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

