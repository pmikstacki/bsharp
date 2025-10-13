use crate::declarations::{ConstructorInitializer, Modifier, TypeParameterConstraintClause};
use crate::expressions::Expression;
use crate::statements::statement::Statement;
use crate::types::{Parameter, Type, TypeParameter};
use crate::Identifier;
use serde::{Deserialize, Serialize};

/// Unified member declaration that handles both methods and constructors
/// structurally without semantic distinction. The syntax creates this
/// structure and the analyzer determines semantic meaning.
#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MemberDeclaration {
    pub modifiers: Vec<Modifier>,
    /// Return type if present. None indicates constructor parser,
    /// Some(Type) indicates method parser. Semantic validation
    /// happens in the analyzer, not the syntax.
    pub return_type: Option<Type>,
    pub name: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    pub parameters: Vec<Parameter>,
    pub body: Option<Statement>,
    pub constraints: Option<Vec<TypeParameterConstraintClause>>,
    /// Optional constructor initializer (": base(...)" or ": this(...)") for constructor syntax
    pub initializer: Option<ConstructorInitializer>,
}

/// Body types supported for member declarations
#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum MemberBody {
    /// Block body: { statements }
    Block(Statement),
    /// Expression body: => expression
    Expression(Box<Expression>),
    /// Abstract/interface member: ;
    None,
}

impl MemberDeclaration {
    /// Helper to determine if this is syntactically a constructor (no return type)
    pub fn has_constructor_syntax(&self) -> bool {
        self.return_type.is_none()
    }

    /// Helper to determine if this is syntactically a method (has return type)
    pub fn has_method_syntax(&self) -> bool {
        self.return_type.is_some()
    }
}
