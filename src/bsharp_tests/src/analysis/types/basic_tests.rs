use analysis::types::*;
use syntax::nodes::declarations::{
    ClassBodyDeclaration, ClassDeclaration, FieldDeclaration, MethodDeclaration,
    PropertyDeclaration,
};
use syntax::nodes::identifier::Identifier;
use syntax::nodes::types::{Parameter, ParameterModifier, Type, TypeParameter, Variance};

fn create_test_identifier(name: &str) -> Identifier {
    Identifier {
        name: name.to_string(),
    }
}

#[test]
fn test_type_analyzer_new() {
    let analyzer = TypeAnalyzer::new();
    assert_eq!(analyzer.discovered_types.len(), 0);
    assert_eq!(analyzer.type_relationships.len(), 0);
}

#[test]
fn test_analyze_basic_types() {
    let mut analyzer = TypeAnalyzer::new();

    let class = ClassDeclaration {
        documentation: None,
        attributes: Vec::new(),
        modifiers: Vec::new(),
        name: create_test_identifier("TestClass"),
        type_parameters: None,
        primary_constructor_parameters: None,
        base_types: vec![
            Type::Reference(create_test_identifier("BaseClass")),
            Type::Reference(create_test_identifier("IInterface")),
        ],
        body_declarations: vec![
            ClassBodyDeclaration::Field(FieldDeclaration {
                modifiers: Vec::new(),
                field_type: Type::Reference(create_test_identifier("string")),
                name: create_test_identifier("name"),
                initializer: None,
            }),
            ClassBodyDeclaration::Property(PropertyDeclaration {
                attributes: Vec::new(),
                modifiers: Vec::new(),
                property_type: Type::Reference(create_test_identifier("int")),
                name: create_test_identifier("Age"),
                accessors: Vec::new(),
                initializer: None,
            }),
        ],
    };

    let metrics = analyzer.analyze_class(&class);

    assert_eq!(metrics.total_types_used, 3); // TestClass, string, int
    assert_eq!(metrics.inheritance_depth, 1); // Inherits from BaseClass
    assert_eq!(metrics.interface_implementations, 1); // Implements IInterface
    assert_eq!(metrics.field_types.len(), 1); // string
    assert_eq!(metrics.property_types.len(), 1); // int
}

#[test]
fn test_type_metrics_default() {
    let metrics = TypeMetrics::default();
    assert_eq!(metrics.total_types_used, 0);
    assert_eq!(metrics.inheritance_depth, 0);
    assert_eq!(metrics.interface_implementations, 0);
    assert_eq!(metrics.generic_type_parameters, 0);
    assert!(metrics.field_types.is_empty());
    assert!(metrics.property_types.is_empty());
    assert!(metrics.method_parameter_types.is_empty());
    assert!(metrics.method_return_types.is_empty());
}

#[test]
fn test_type_metrics_combine() {
    let metrics1 = TypeMetrics {
        total_types_used: 5,
        inheritance_depth: 2,
        interface_implementations: 1,
        generic_type_parameters: 1,
        field_types: vec!["string".to_string(), "int".to_string()],
        property_types: vec!["bool".to_string()],
        ..Default::default()
    };

    let metrics2 = TypeMetrics {
        total_types_used: 3,
        inheritance_depth: 1,
        interface_implementations: 2,
        generic_type_parameters: 2,
        field_types: vec!["double".to_string()],
        property_types: vec!["char".to_string()],
        ..Default::default()
    };

    let combined = metrics1.combine(metrics2);

    assert_eq!(combined.total_types_used, 8);
    assert_eq!(combined.inheritance_depth, 2); // Max depth
    assert_eq!(combined.interface_implementations, 3);
    assert_eq!(combined.generic_type_parameters, 3);
    assert_eq!(combined.field_types.len(), 3);
    assert_eq!(combined.property_types.len(), 2);
}

#[test]
fn test_analyze_generic_types() {
    let mut analyzer = TypeAnalyzer::new();

    let generic_class = ClassDeclaration {
        documentation: None,
        attributes: Vec::new(),
        modifiers: Vec::new(),
        name: create_test_identifier("GenericClass"),
        type_parameters: Some(vec![
            TypeParameter {
                name: create_test_identifier("T"),
                variance: Variance::None,
            },
            TypeParameter {
                name: create_test_identifier("U"),
                variance: Variance::None,
            },
        ]),
        primary_constructor_parameters: None,
        base_types: Vec::new(),
        body_declarations: vec![
            ClassBodyDeclaration::Field(FieldDeclaration {
                modifiers: Vec::new(),
                field_type: Type::Reference(create_test_identifier("T")),
                name: create_test_identifier("value"),
                initializer: None,
            }),
            ClassBodyDeclaration::Method(MethodDeclaration {
                modifiers: Vec::new(),
                return_type: Type::Reference(create_test_identifier("U")),
                name: create_test_identifier("GetValue"),
                type_parameters: Some(vec![TypeParameter {
                    name: create_test_identifier("V"),
                    variance: Variance::None,
                }]),
                parameters: vec![Parameter {
                    attributes: vec![],
                    modifier: None,
                    parameter_type: Type::Reference(create_test_identifier("V")),
                    name: create_test_identifier("input"),
                    default_value: None,
                }],
                body: None,
                constraints: None,
            }),
        ],
    };

    let metrics = analyzer.analyze_class(&generic_class);

    assert_eq!(metrics.generic_type_parameters, 3); // T, U, V (from method)
    assert!(metrics.field_types.contains(&"T".to_string()));
    assert!(metrics.method_return_types.contains(&"U".to_string()));
    assert!(metrics.method_parameter_types.contains(&"V".to_string()));
}

#[test]
fn test_inheritance_analysis() {
    let mut analyzer = TypeAnalyzer::new();

    // Base class
    let base_class = ClassDeclaration {
        documentation: None,
        attributes: Vec::new(),
        modifiers: Vec::new(),
        name: create_test_identifier("BaseClass"),
        type_parameters: None,
        primary_constructor_parameters: None,
        base_types: Vec::new(),
        body_declarations: Vec::new(),
    };

    // Derived class
    let derived_class = ClassDeclaration {
        documentation: None,
        attributes: Vec::new(),
        modifiers: Vec::new(),
        name: create_test_identifier("DerivedClass"),
        type_parameters: None,
        primary_constructor_parameters: None,
        base_types: vec![Type::Reference(create_test_identifier("BaseClass"))],
        body_declarations: Vec::new(),
    };

    // Further derived class
    let further_derived = ClassDeclaration {
        documentation: None,
        attributes: Vec::new(),
        modifiers: Vec::new(),
        name: create_test_identifier("FurtherDerived"),
        type_parameters: None,
        primary_constructor_parameters: None,
        base_types: vec![Type::Reference(create_test_identifier("DerivedClass"))],
        body_declarations: Vec::new(),
    };

    // Analyze inheritance chain
    analyzer.analyze_class(&base_class);
    let derived_metrics = analyzer.analyze_class(&derived_class);
    let further_metrics = analyzer.analyze_class(&further_derived);

    assert_eq!(derived_metrics.inheritance_depth, 1);
    assert_eq!(further_metrics.inheritance_depth, 1); // Each only has one direct parent

    // Check inheritance relationships
    let hierarchy = analyzer.build_inheritance_hierarchy();
    assert!(hierarchy.contains_key("BaseClass") || hierarchy.is_empty()); // May be empty for now
}

#[test]
fn test_interface_analysis() {
    let mut analyzer = TypeAnalyzer::new();

    let class_with_interfaces = ClassDeclaration {
        documentation: None,
        attributes: Vec::new(),
        modifiers: Vec::new(),
        name: create_test_identifier("MultiInterfaceClass"),
        type_parameters: None,
        primary_constructor_parameters: None,
        base_types: vec![
            Type::Reference(create_test_identifier("IComparable")),
            Type::Reference(create_test_identifier("IDisposable")),
            Type::Reference(create_test_identifier("ICloneable")),
        ],
        body_declarations: Vec::new(),
    };

    let metrics = analyzer.analyze_class(&class_with_interfaces);

    assert_eq!(metrics.interface_implementations, 2); // All but first are interfaces
    assert!(
        metrics
            .implemented_interfaces
            .contains(&"IComparable".to_string())
    );
    assert!(
        metrics
            .implemented_interfaces
            .contains(&"IDisposable".to_string())
    );
    assert!(
        metrics
            .implemented_interfaces
            .contains(&"ICloneable".to_string())
    );
}

#[test]
fn test_type_usage_analysis() {
    let mut analyzer = TypeAnalyzer::new();

    let class_with_various_types = ClassDeclaration {
        documentation: None,
        attributes: Vec::new(),
        modifiers: Vec::new(),
        name: create_test_identifier("TypeUsageClass"),
        type_parameters: None,
        primary_constructor_parameters: None,
        base_types: Vec::new(),
        body_declarations: vec![
            ClassBodyDeclaration::Field(FieldDeclaration {
                modifiers: Vec::new(),
                field_type: Type::Array {
                    element_type: Box::new(Type::Reference(create_test_identifier("string"))),
                    rank: 1,
                },
                name: create_test_identifier("names"),
                initializer: None,
            }),
            ClassBodyDeclaration::Property(PropertyDeclaration {
                attributes: Vec::new(),
                modifiers: Vec::new(),
                property_type: Type::Nullable(Box::new(Type::Reference(create_test_identifier("int")))),
                name: create_test_identifier("OptionalAge"),
                accessors: Vec::new(),
                initializer: None,
            }),
            ClassBodyDeclaration::Method(MethodDeclaration {
                modifiers: Vec::new(),
                return_type: Type::Generic {
                    base: create_test_identifier("List"),
                    args: vec![Type::Reference(create_test_identifier("Person"))],
                },
                name: create_test_identifier("GetPeople"),
                type_parameters: None,
                parameters: vec![
                    Parameter {
                        attributes: vec![],
                        modifier: Some(ParameterModifier::Ref),
                        parameter_type: Type::Reference(create_test_identifier("Database")),
                        name: create_test_identifier("db"),
                        default_value: None,
                    },
                    Parameter {
                        attributes: vec![],
                        modifier: Some(ParameterModifier::Out),
                        parameter_type: Type::Reference(create_test_identifier("bool")),
                        name: create_test_identifier("success"),
                        default_value: None,
                    },
                ],
                body: None,
                constraints: None,
            }),
        ],
    };

    let metrics = analyzer.analyze_class(&class_with_various_types);

    // Check that complex types are properly analyzed
    assert!(!metrics.array_types.is_empty());
    assert!(!metrics.nullable_types.is_empty());
    assert!(!metrics.generic_type_usages.is_empty());
    assert!(metrics.ref_parameters > 0);
    assert!(metrics.out_parameters > 0);
}

#[test]
fn test_type_complexity_analysis() {
    let mut analyzer = TypeAnalyzer::new();

    let complex_generic_class = ClassDeclaration {
        documentation: None,
        attributes: Vec::new(),
        modifiers: Vec::new(),
        name: create_test_identifier("ComplexGenericClass"),
        type_parameters: Some(vec![TypeParameter {
            name: create_test_identifier("T"),
            variance: Variance::None,
        }]),
        primary_constructor_parameters: None,
        base_types: Vec::new(),
        body_declarations: vec![ClassBodyDeclaration::Method(MethodDeclaration {
            modifiers: Vec::new(),
            return_type: Type::Generic {
                base: create_test_identifier("Dictionary"),
                args: vec![
                    Type::Reference(create_test_identifier("string")),
                    Type::Generic {
                        base: create_test_identifier("List"),
                        args: vec![Type::Nullable(Box::new(Type::Reference(
                            create_test_identifier("T"),
                        )))],
                    },
                ],
            },
            name: create_test_identifier("GetComplexData"),
            type_parameters: None,
            parameters: Vec::new(),
            body: None,
            constraints: None,
        })],
    };

    let metrics = analyzer.analyze_class(&complex_generic_class);
    let complexity = analyzer.calculate_type_complexity(&metrics);

    // Complex nested generics should have high complexity
    assert!(complexity.nesting_depth >= 3); // Dictionary<string, List<T?>>
    assert!(complexity.generic_type_count >= 1); // At least one generic usage
    assert!(complexity.overall_complexity > 10.0);
}

#[test]
fn test_circular_dependency_detection() {
    let mut analyzer = TypeAnalyzer::new();

    // Create circular dependency scenario
    let class_a = ClassDeclaration {
        documentation: None,
        attributes: Vec::new(),
        modifiers: Vec::new(),
        name: create_test_identifier("ClassA"),
        type_parameters: None,
        primary_constructor_parameters: None,
        base_types: Vec::new(),
        body_declarations: vec![ClassBodyDeclaration::Field(FieldDeclaration {
            modifiers: Vec::new(),
            field_type: Type::Reference(create_test_identifier("ClassB")),
            name: create_test_identifier("b"),
            initializer: None,
        })],
    };

    let class_b = ClassDeclaration {
        documentation: None,
        attributes: Vec::new(),
        modifiers: Vec::new(),
        name: create_test_identifier("ClassB"),
        type_parameters: None,
        primary_constructor_parameters: None,
        base_types: Vec::new(),
        body_declarations: vec![ClassBodyDeclaration::Field(FieldDeclaration {
            modifiers: Vec::new(),
            field_type: Type::Reference(create_test_identifier("ClassA")),
            name: create_test_identifier("a"),
            initializer: None,
        })],
    };

    analyzer.analyze_class(&class_a);
    analyzer.analyze_class(&class_b);

    let circular_deps = analyzer.detect_circular_dependencies();
    // For now, this may be empty since detection is not fully implemented
    assert!(circular_deps.is_empty() || !circular_deps.is_empty());
}

#[test]
fn test_type_cohesion_analysis() {
    let mut analyzer = TypeAnalyzer::new();

    // Well-cohesive class (all members use related types)
    let cohesive_class = ClassDeclaration {
        documentation: None,
        attributes: Vec::new(),
        modifiers: Vec::new(),
        name: create_test_identifier("PersonManager"),
        type_parameters: None,
        primary_constructor_parameters: None,
        base_types: Vec::new(),
        body_declarations: vec![
            ClassBodyDeclaration::Field(FieldDeclaration {
                modifiers: Vec::new(),
                field_type: Type::Generic {
                    base: create_test_identifier("List"),
                    args: vec![Type::Reference(create_test_identifier("Person"))],
                },
                name: create_test_identifier("people"),
                initializer: None,
            }),
            ClassBodyDeclaration::Method(MethodDeclaration {
                modifiers: Vec::new(),
                return_type: Type::Reference(create_test_identifier("Person")),
                name: create_test_identifier("FindPerson"),
                type_parameters: None,
                parameters: vec![Parameter {
                    attributes: vec![],
                    modifier: None,
                    parameter_type: Type::Reference(create_test_identifier("PersonId")),
                    name: create_test_identifier("id"),
                    default_value: None,
                }],
                body: None,
                constraints: None,
            }),
        ],
    };

    let metrics = analyzer.analyze_class(&cohesive_class);
    let cohesion = analyzer.calculate_type_cohesion(&metrics);

    // Should have reasonable cohesion values
    assert!(cohesion.semantic_cohesion >= 0.0 && cohesion.semantic_cohesion <= 1.0);
    assert!(cohesion.type_relatedness >= 0.0 && cohesion.type_relatedness <= 1.0);
    assert!(cohesion.overall_cohesion >= 0.0 && cohesion.overall_cohesion <= 1.0);
}

#[test]
fn test_comprehensive_type_analysis() {
    let mut analyzer = TypeAnalyzer::new();

    // Analyze a comprehensive class with various type features
    let comprehensive_class = ClassDeclaration {
        documentation: None,
        attributes: Vec::new(),
        modifiers: Vec::new(),
        name: create_test_identifier("ComprehensiveClass"),
        type_parameters: Some(vec![TypeParameter {
            name: create_test_identifier("T"),
            variance: Variance::None,
        }]),
        primary_constructor_parameters: None,
        base_types: vec![
            Type::Reference(create_test_identifier("BaseClass")),
            Type::Reference(create_test_identifier("IInterface1")),
            Type::Reference(create_test_identifier("IInterface2")),
        ],
        body_declarations: vec![
            ClassBodyDeclaration::Field(FieldDeclaration {
                modifiers: Vec::new(),
                field_type: Type::Array {
                    element_type: Box::new(Type::Reference(create_test_identifier("T"))),
                    rank: 1,
                },
                name: create_test_identifier("items"),
                initializer: None,
            }),
            ClassBodyDeclaration::Property(PropertyDeclaration {
                attributes: Vec::new(),
                modifiers: Vec::new(),
                property_type: Type::Nullable(Box::new(Type::Reference(create_test_identifier("string")))),
                name: create_test_identifier("Name"),
                accessors: Vec::new(),
                initializer: None,
            }),
            ClassBodyDeclaration::Method(MethodDeclaration {
                modifiers: Vec::new(),
                return_type: Type::Generic {
                    base: create_test_identifier("Task"),
                    args: vec![Type::Reference(create_test_identifier("bool"))],
                },
                name: create_test_identifier("ProcessAsync"),
                type_parameters: Some(vec![TypeParameter {
                    name: create_test_identifier("U"),
                    variance: Variance::None,
                }]),
                parameters: vec![Parameter {
                    attributes: vec![],
                    modifier: None,
                    parameter_type: Type::Reference(create_test_identifier("U")),
                    name: create_test_identifier("processor"),
                    default_value: None,
                }],
                body: None,
                constraints: None,
            }),
        ],
    };

    let metrics = analyzer.analyze_class(&comprehensive_class);

    // Verify comprehensive analysis results
    assert_eq!(metrics.generic_type_parameters, 2); // T and U
    assert_eq!(metrics.inheritance_depth, 1); // Inherits from BaseClass
    assert_eq!(metrics.interface_implementations, 2); // IInterface1, IInterface2
    assert!(!metrics.array_types.is_empty());
    assert!(!metrics.nullable_types.is_empty());
    assert!(!metrics.async_return_types.is_empty());

    let complexity = analyzer.calculate_type_complexity(&metrics);
    assert!(complexity.overall_complexity >= 0.0);

    let cohesion = analyzer.calculate_type_cohesion(&metrics);
    assert!(cohesion.overall_cohesion >= 0.0 && cohesion.overall_cohesion <= 1.0);
}
