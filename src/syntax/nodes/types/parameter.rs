// Import TypeSyntax from the same directory's mod.rs (which will re-export it)
use super::Type;
use crate::syntax::nodes::declarations::Modifier;
use crate::syntax::nodes::identifier::Identifier;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub modifier: Option<ParameterModifier>,
    pub parameter_type: Type,
    pub name: Identifier,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ParameterModifier {
    Ref,
    Out,
    In,
    Params,
}

impl ParameterModifier {
    /// Convert from general Modifier to ParameterModifier
    pub fn from_modifier(modifier: &Modifier) -> Option<Self> {
        match modifier {
            Modifier::Ref => Some(ParameterModifier::Ref),
            Modifier::Out => Some(ParameterModifier::Out),
            Modifier::In => Some(ParameterModifier::In),
            Modifier::Params => Some(ParameterModifier::Params),
            _ => None,
        }
    }

    /// Check if the parameter is by reference (ref, out, in)
    pub fn is_by_reference(&self) -> bool {
        matches!(
            self,
            ParameterModifier::Ref | ParameterModifier::Out | ParameterModifier::In
        )
    }

    /// Check if the parameter requires initialization before use
    pub fn requires_initialization(&self) -> bool {
        matches!(self, ParameterModifier::Ref | ParameterModifier::In)
    }

    /// Check if the parameter must be assigned in the method
    pub fn must_be_assigned(&self) -> bool {
        matches!(self, ParameterModifier::Out)
    }

    /// Check if the parameter is read-only
    pub fn is_read_only(&self) -> bool {
        matches!(self, ParameterModifier::In)
    }

    /// Check if this is a params parameter (variable arguments)
    pub fn is_params(&self) -> bool {
        matches!(self, ParameterModifier::Params)
    }
}
