use crate::errors::BResult;
use nom::sequence::delimited;
use nom::Parser as _;
use crate::trivia::comment_parser::ws;
use crate::span_ext::ParserExt as _;
use syntax::span::Span;
use crate::span::Spanned;


// Minimal, focused trait for one-shot parsing
pub trait Parsable<'a>: Sized {
    fn parse(input: Span<'a>) -> BResult<'a, Spanned<Self>>;
}

// Optional variant that also returns the recognized span relative to the input slice
pub trait ParsableSpanned<'a>: Sized {
    fn parse_with_span(input: Span<'a>) -> BResult<'a, (Self, std::ops::Range<usize>)>;
}

// Macro to generate a Parsable impl delegating to an existing parser function
#[macro_export]
macro_rules! impl_parsable {
    ($ty:path => $parser:path) => {
        impl<'a> $crate::traits::parsable::Parsable<'a> for $ty {
            fn parse(input: Span<'a>) -> $crate::errors::BResult<'a, $crate::span::Spanned<Self>> {
                use $crate::span_ext::ParserExt as _;
                let p = |i: Span<'a>| $parser(i);
                p.spanned()(input)
            }
        }
    };
}

// Macro to generate a ParsableSpanned impl using with_recognized_span
#[macro_export]
macro_rules! impl_parsable_spanned {
    ($ty:path => $parser:path) => {
        impl<'a> $crate::traits::parsable::ParsableSpanned<'a> for $ty {
            fn parse_with_span(
                input: Span<'a>,
            ) -> $crate::errors::BResult<'a, (Self, std::ops::Range<usize>)> {
                let (rest, s) = <Self as $crate::traits::parsable::Parsable<'a>>::parse(input)?;
                Ok((rest, (s.node, s.abs.start..s.abs.end)))
            }
        }
    };
}

// ===== Implementations for primary syntax nodes =====
// Type declarations (custom: exclude surrounding trivia)
impl<'a> crate::traits::parsable::Parsable<'a> for syntax::declarations::ClassDeclaration {
    fn parse(input: Span<'a>) -> BResult<'a, crate::span::Spanned<Self>> {
        delimited(ws, (|i| crate::parser::expressions::declarations::type_declaration_parser::parse_class_declaration(i)).spanned(), ws).parse(input)
    }
}
impl<'a> crate::traits::parsable::Parsable<'a> for syntax::declarations::StructDeclaration {
    fn parse(input: Span<'a>) -> BResult<'a, crate::span::Spanned<Self>> {
        delimited(ws, (|i| crate::parser::expressions::declarations::type_declaration_parser::parse_struct_declaration_span(i)).spanned(), ws).parse(input)
    }
}
impl<'a> crate::traits::parsable::Parsable<'a> for syntax::declarations::InterfaceDeclaration {
    fn parse(input: Span<'a>) -> BResult<'a, crate::span::Spanned<Self>> {
        delimited(ws, (|i| crate::parser::expressions::declarations::type_declaration_parser::parse_interface_declaration(i)).spanned(), ws).parse(input)
    }
}
impl<'a> crate::traits::parsable::Parsable<'a> for syntax::declarations::RecordDeclaration {
    fn parse(input: Span<'a>) -> BResult<'a, crate::span::Spanned<Self>> {
        delimited(ws, (|i| crate::parser::expressions::declarations::type_declaration_parser::parse_record_declaration(i)).spanned(), ws).parse(input)
    }
}
impl<'a> crate::traits::parsable::Parsable<'a> for syntax::declarations::EnumDeclaration {
    fn parse(input: Span<'a>) -> BResult<'a, crate::span::Spanned<Self>> {
        delimited(ws, (|i| crate::parser::expressions::declarations::enum_declaration_parser::parse_enum_declaration(i)).spanned(), ws).parse(input)
    }
}
impl<'a> crate::traits::parsable::Parsable<'a> for syntax::declarations::DelegateDeclaration {
    fn parse(input: Span<'a>) -> BResult<'a, crate::span::Spanned<Self>> {
        delimited(ws, (|i| crate::parser::expressions::declarations::delegate_declaration_parser::parse_delegate_declaration(i)).spanned(), ws).parse(input)
    }
}

// Top-level declaration (custom: exclude surrounding trivia)
impl<'a> crate::traits::parsable::Parsable<'a> for syntax::ast::TopLevelDeclaration {
    fn parse(input: Span<'a>) -> BResult<'a, crate::span::Spanned<Self>> {
        delimited(ws, (|i| crate::parser::bsharp::parse_top_level_member(i)).spanned(), ws).parse(input)
    }
}

// Namespaces and using (custom: exclude surrounding trivia)
impl<'a> crate::traits::parsable::Parsable<'a> for syntax::declarations::NamespaceDeclaration {
    fn parse(input: Span<'a>) -> BResult<'a, crate::span::Spanned<Self>> {
        delimited(ws, (|i| crate::parser::expressions::declarations::namespace_declaration_parser::parse_namespace_declaration(i)).spanned(), ws).parse(input)
    }
}
impl<'a> crate::traits::parsable::Parsable<'a> for syntax::declarations::FileScopedNamespaceDeclaration {
    fn parse(input: Span<'a>) -> BResult<'a, crate::span::Spanned<Self>> {
        delimited(ws, (|i| crate::parser::expressions::declarations::file_scoped_namespace_parser::parse_file_scoped_namespace_declaration(i)).spanned(), ws).parse(input)
    }
}
impl<'a> crate::traits::parsable::Parsable<'a> for syntax::declarations::UsingDirective {
    fn parse(input: Span<'a>) -> BResult<'a, crate::span::Spanned<Self>> {
        delimited(ws, (|i| crate::parser::expressions::declarations::using_directive_parser::parse_using_directive(i)).spanned(), ws).parse(input)
    }
}

// Member-level declarations (custom: exclude surrounding trivia)
impl<'a> crate::traits::parsable::Parsable<'a> for syntax::declarations::FieldDeclaration {
    fn parse(input: Span<'a>) -> BResult<'a, crate::span::Spanned<Self>> {
        delimited(ws, (|i| crate::parser::expressions::declarations::field_declaration_parser::parse_field_declaration(i)).spanned(), ws).parse(input)
    }
}
impl<'a> crate::traits::parsable::Parsable<'a> for syntax::declarations::PropertyDeclaration {
    fn parse(input: Span<'a>) -> BResult<'a, crate::span::Spanned<Self>> {
        delimited(ws, (|i| crate::parser::expressions::declarations::property_declaration_parser::parse_property_declaration(i)).spanned(), ws).parse(input)
    }
}
impl<'a> crate::traits::parsable::Parsable<'a> for syntax::declarations::EventDeclaration {
    fn parse(input: Span<'a>) -> BResult<'a, crate::span::Spanned<Self>> {
        delimited(ws, (|i| crate::parser::expressions::declarations::event_declaration_parser::parse_event_declaration(i)).spanned(), ws).parse(input)
    }
}
impl<'a> crate::traits::parsable::Parsable<'a> for syntax::declarations::IndexerDeclaration {
    fn parse(input: Span<'a>) -> BResult<'a, crate::span::Spanned<Self>> {
        delimited(ws, (|i| crate::parser::expressions::declarations::indexer_declaration_parser::parse_indexer_declaration(i)).spanned(), ws).parse(input)
    }
}
impl<'a> crate::traits::parsable::Parsable<'a> for syntax::declarations::OperatorDeclaration {
    fn parse(input: Span<'a>) -> BResult<'a, crate::span::Spanned<Self>> {
        delimited(ws, (|i| crate::parser::expressions::declarations::operator_declaration_parser::parse_operator_declaration(i)).spanned(), ws).parse(input)
    }
}
impl<'a> crate::traits::parsable::Parsable<'a> for syntax::declarations::DestructorDeclaration {
    fn parse(input: Span<'a>) -> BResult<'a, crate::span::Spanned<Self>> {
        delimited(ws, (|i| crate::parser::expressions::declarations::destructor_declaration_parser::parse_destructor_declaration(i)).spanned(), ws).parse(input)
    }
}
impl<'a> crate::traits::parsable::Parsable<'a> for syntax::declarations::MemberDeclaration {
    fn parse(input: Span<'a>) -> BResult<'a, crate::span::Spanned<Self>> {
        delimited(ws, (|i| crate::parser::expressions::declarations::method_declaration_parser::parse_member_declaration(i)).spanned(), ws).parse(input)
    }
}
impl<'a> crate::traits::parsable::Parsable<'a> for syntax::declarations::MethodDeclaration {
    fn parse(input: Span<'a>) -> BResult<'a, crate::span::Spanned<Self>> {
        delimited(ws, (|i| crate::parser::expressions::declarations::method_declaration_parser::parse_pure_method_declaration(i)).spanned(), ws).parse(input)
    }
}
impl<'a> crate::traits::parsable::Parsable<'a> for syntax::declarations::ConstructorDeclaration {
    fn parse(input: Span<'a>) -> BResult<'a, crate::span::Spanned<Self>> {
        delimited(ws, (|i| crate::parser::expressions::declarations::method_declaration_parser::parse_constructor_declaration(i)).spanned(), ws).parse(input)
    }
}

// Class body element (for span-based collection helper)
impl_parsable!(syntax::declarations::ClassBodyDeclaration => crate::parser::expressions::declarations::type_declaration_parser::parse_class_member_for_spans);
impl_parsable!(syntax::declarations::StructBodyDeclaration => crate::parser::expressions::declarations::type_declaration_parser::parse_struct_member_for_spans);
impl_parsable!(syntax::declarations::InterfaceBodyDeclaration => crate::parser::expressions::declarations::type_declaration_parser::parse_interface_member_for_spans);
impl_parsable!(syntax::declarations::namespace_declaration::NamespaceBodyDeclaration => crate::parser::expressions::declarations::namespace_declaration_parser::parse_namespace_member_for_spans);

// Statements and expressions
// Statement: custom impl to use parse_statement_spanned so spans exclude trivia
impl<'a> crate::traits::parsable::Parsable<'a> for syntax::statements::statement::Statement {
    fn parse(input: Span<'a>) -> BResult<'a, crate::span::Spanned<Self>> {
        crate::parser::statement_parser::parse_statement_spanned(input)
    }
}
// Expression: custom impl to use parse_expression_spanned so spans exclude trivia
impl<'a> crate::traits::parsable::Parsable<'a> for syntax::expressions::expression::Expression {
    fn parse(input: Span<'a>) -> BResult<'a, crate::span::Spanned<Self>> {
        crate::parser::expressions::primary_expression_parser::parse_expression_spanned(input)
    }
}

// Types and identifiers
impl_parsable!(syntax::types::Type => crate::parser::types::type_parser::parse_type_expression);
// Identifier: custom impl to use parse_identifier_spanned so spans exclude trivia
impl<'a> crate::traits::parsable::Parsable<'a> for syntax::identifier::Identifier {
    fn parse(input: Span<'a>) -> BResult<'a, crate::span::Spanned<Self>> {
        crate::parser::identifier_parser::parse_identifier_spanned(input)
    }
}
impl_parsable!(syntax::types::Parameter => crate::parser::expressions::declarations::parameter_parser::parse_parameter);
impl_parsable!(syntax::declarations::local_variable_declaration::LocalVariableDeclaration => crate::parser::expressions::declarations::variable_declaration_parser::parse_variable_declaration);
impl_parsable!(syntax::types::TypeParameter => crate::parser::expressions::declarations::type_parameter_parser::parse_type_parameter_node);
impl_parsable!(syntax::declarations::TypeParameterConstraint => crate::parser::expressions::declarations::type_parameter_parser::parse_type_parameter_constraint_node);
impl_parsable!(syntax::declarations::TypeParameterConstraintClause => crate::parser::expressions::declarations::type_parameter_parser::parse_type_parameter_where_clause);

// Attributes
impl_parsable!(syntax::declarations::Attribute => crate::parser::expressions::declarations::attribute_parser::parse_attribute);
impl_parsable!(syntax::declarations::AttributeList => crate::parser::expressions::declarations::attribute_parser::parse_attribute_list);
impl_parsable!(syntax::declarations::global_attribute::GlobalAttribute => crate::parser::expressions::declarations::global_attribute_parser::parse_global_attribute);

// Preprocessor directives
impl_parsable!(syntax::trivia::preprocessor::PreprocessorDirective => crate::parser::trivia::preprocessor_directive_parser::parse_preprocessor_directive);

// Root
impl<'a> crate::traits::parsable::Parsable<'a> for syntax::ast::CompilationUnit {
    fn parse(input: Span<'a>) -> BResult<'a, crate::span::Spanned<Self>> {
        crate::parser::bsharp::parse_csharp_source_spanned(input)
    }
}
