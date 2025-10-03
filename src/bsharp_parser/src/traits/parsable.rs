use crate::syntax::errors::BResult;

// Minimal, focused trait for one-shot parsing
pub trait Parsable: Sized {
    fn parse(input: &str) -> BResult<&str, Self>;
}

// Optional variant that also returns the recognized span relative to the input slice
pub trait ParsableSpanned: Sized {
    fn parse_with_span(input: &str) -> BResult<&str, (Self, std::ops::Range<usize>)>;
}

// Macro to generate a Parsable impl delegating to an existing parser function
#[macro_export]
macro_rules! impl_parsable {
    ($ty:path => $parser:path) => {
        impl $crate::traits::parsable::Parsable for $ty {
            fn parse(input: &str) -> $crate::syntax::errors::BResult<&str, Self> {
                $parser(input)
            }
        }
    };
}

// Macro to generate a ParsableSpanned impl using with_recognized_span
#[macro_export]
macro_rules! impl_parsable_spanned {
    ($ty:path => $parser:path) => {
        impl $crate::traits::parsable::ParsableSpanned for $ty {
            fn parse_with_span(
                input: &str,
            ) -> $crate::syntax::errors::BResult<&str, (Self, std::ops::Range<usize>)> {
                $crate::syntax::parser_helpers::with_recognized_span(input, $parser)(input)
            }
        }
    };
}

// ===== Implementations for primary syntax nodes =====
// Type declarations
impl_parsable!(syntax::nodes::declarations::ClassDeclaration     => crate::parser::expressions::declarations::type_declaration_parser::parse_class_declaration);
impl_parsable!(syntax::nodes::declarations::StructDeclaration    => crate::parser::expressions::declarations::type_declaration_parser::parse_struct_declaration);
impl_parsable!(syntax::nodes::declarations::InterfaceDeclaration => crate::parser::expressions::declarations::type_declaration_parser::parse_interface_declaration);
impl_parsable!(syntax::nodes::declarations::RecordDeclaration    => crate::parser::expressions::declarations::type_declaration_parser::parse_record_declaration);
impl_parsable!(syntax::nodes::declarations::EnumDeclaration      => crate::parser::expressions::declarations::enum_declaration_parser::parse_enum_declaration);
impl_parsable!(syntax::nodes::declarations::DelegateDeclaration  => crate::parser::expressions::declarations::delegate_declaration_parser::parse_delegate_declaration);

// Top-level declaration
impl_parsable!(syntax::ast::TopLevelDeclaration => crate::parser::bsharp::parse_top_level_member);
impl_parsable_spanned!(syntax::ast::TopLevelDeclaration => crate::parser::bsharp::parse_top_level_member);

// Namespaces and using
impl_parsable!(syntax::nodes::declarations::NamespaceDeclaration => crate::parser::expressions::declarations::namespace_declaration_parser::parse_namespace_declaration);
impl_parsable!(syntax::nodes::declarations::FileScopedNamespaceDeclaration => crate::parser::expressions::declarations::file_scoped_namespace_parser::parse_file_scoped_namespace_declaration);
impl_parsable!(syntax::nodes::declarations::UsingDirective => crate::parser::expressions::declarations::using_directive_parser::parse_using_directive);

// Member-level declarations
impl_parsable!(syntax::nodes::declarations::FieldDeclaration    => crate::parser::expressions::declarations::field_declaration_parser::parse_field_declaration);
impl_parsable!(syntax::nodes::declarations::PropertyDeclaration => crate::parser::expressions::declarations::property_declaration_parser::parse_property_declaration);
impl_parsable!(syntax::nodes::declarations::EventDeclaration    => crate::parser::expressions::declarations::event_declaration_parser::parse_event_declaration);
impl_parsable!(syntax::nodes::declarations::IndexerDeclaration  => crate::parser::expressions::declarations::indexer_declaration_parser::parse_indexer_declaration);
impl_parsable!(syntax::nodes::declarations::OperatorDeclaration => crate::parser::expressions::declarations::operator_declaration_parser::parse_operator_declaration);
impl_parsable!(syntax::nodes::declarations::DestructorDeclaration => crate::parser::expressions::declarations::destructor_declaration_parser::parse_destructor_declaration);
impl_parsable!(syntax::nodes::declarations::MemberDeclaration   => crate::parser::expressions::declarations::method_declaration_parser::parse_member_declaration);
impl_parsable!(syntax::nodes::declarations::MethodDeclaration   => crate::parser::expressions::declarations::method_declaration_parser::parse_pure_method_declaration);
impl_parsable!(syntax::nodes::declarations::ConstructorDeclaration => crate::parser::expressions::declarations::method_declaration_parser::parse_constructor_declaration);

// Class body element (for span-based collection helper)
impl_parsable!(syntax::nodes::declarations::ClassBodyDeclaration => crate::parser::expressions::declarations::type_declaration_parser::parse_class_member_for_spans);
impl_parsable!(syntax::nodes::declarations::StructBodyDeclaration => crate::parser::expressions::declarations::type_declaration_parser::parse_struct_member_for_spans);
impl_parsable!(syntax::nodes::declarations::InterfaceBodyDeclaration => crate::parser::expressions::declarations::type_declaration_parser::parse_interface_member_for_spans);
impl_parsable!(syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration => crate::parser::expressions::declarations::namespace_declaration_parser::parse_namespace_member_for_spans);

// Statements and expressions
impl_parsable!(syntax::nodes::statements::statement::Statement => crate::parser::statement_parser::parse_statement);
impl_parsable!(syntax::nodes::expressions::expression::Expression => crate::parser::expressions::primary_expression_parser::parse_expression);

// Types and identifiers
impl_parsable!(syntax::nodes::types::Type => crate::parser::types::type_parser::parse_type_expression);
impl_parsable!(syntax::nodes::identifier::Identifier => crate::parser::identifier_parser::parse_identifier);
impl_parsable!(syntax::nodes::types::Parameter => crate::parser::expressions::declarations::parameter_parser::parse_parameter);
impl_parsable!(syntax::nodes::declarations::local_variable_declaration::LocalVariableDeclaration => crate::parser::expressions::declarations::variable_declaration_parser::parse_variable_declaration);
impl_parsable!(syntax::nodes::types::TypeParameter => crate::parser::expressions::declarations::type_parameter_parser::parse_type_parameter_node);
impl_parsable!(syntax::nodes::declarations::TypeParameterConstraint => crate::parser::expressions::declarations::type_parameter_parser::parse_type_parameter_constraint_node);
impl_parsable!(syntax::nodes::declarations::TypeParameterConstraintClause => crate::parser::expressions::declarations::type_parameter_parser::parse_type_parameter_where_clause);

// Attributes
impl_parsable!(syntax::declarations::Attribute => crate::parser::expressions::declarations::attribute_parser::parse_attribute);
impl_parsable!(syntax::declarations::AttributeList => crate::parser::expressions::declarations::attribute_parser::parse_attribute_list);
impl_parsable!(syntax::nodes::declarations::global_attribute::GlobalAttribute => crate::parser::expressions::declarations::global_attribute_parser::parse_global_attribute);

// Preprocessor directives
impl_parsable!(syntax::trivia::preprocessor::PreprocessorDirective => crate::parser::trivia::preprocessor_directive_parser::parse_preprocessor_directive);

// Root
impl_parsable!(syntax::ast::CompilationUnit => crate::parser::bsharp::parse_csharp_source);
