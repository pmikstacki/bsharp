use serde::{Deserialize, Serialize};

/// Represents modifier categories in C#
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ModifierCategory {
    /// Access modifiers: public, private, protected, internal
    Access,
    /// Static, instance modifier
    StaticInstance,
    /// Inheritance modifiers: abstract, sealed, virtual, override
    Inheritance,
    /// State modifiers: readonly, const, volatile
    State,
    /// Special modifiers: async, unsafe, extern, partial, new, etc.
    Special,
    /// Parameter modifiers: ref, out, in, params
    Parameter,
    /// Storage modifiers: fixed
    Storage,
}

/// Represents a C# modifier with its category
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ModifierInfo {
    pub category: ModifierCategory,
    pub order: u8, // Used to enforce correct ordering within categories
}

/// Represents a C# modifier
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Modifier {
    // Access modifiers
    Public,
    Private,
    Protected,
    Internal,

    // Static/instance modifier
    Static,

    // Inheritance modifiers
    Abstract,
    Sealed,
    Virtual,
    Override,

    // State modifiers
    Readonly,
    Volatile,
    Const,

    // Special modifiers
    Unsafe,
    Extern,
    New,
    Partial,
    Async,
    Required,

    // Parameter modifiers
    Ref,
    Out,
    In,
    Params,

    // Storage modifiers
    Fixed,
}

impl Modifier {
    /// Get the modifier info including category and ordering
    pub fn get_info(&self) -> ModifierInfo {
        match self {
            // Access modifiers (should be first)
            Modifier::Public => ModifierInfo {
                category: ModifierCategory::Access,
                order: 1,
            },
            Modifier::Private => ModifierInfo {
                category: ModifierCategory::Access,
                order: 1,
            },
            Modifier::Protected => ModifierInfo {
                category: ModifierCategory::Access,
                order: 1,
            },
            Modifier::Internal => ModifierInfo {
                category: ModifierCategory::Access,
                order: 1,
            },

            // Static modifier (should be second)
            Modifier::Static => ModifierInfo {
                category: ModifierCategory::StaticInstance,
                order: 2,
            },

            // Inheritance modifiers (should be third)
            Modifier::Abstract => ModifierInfo {
                category: ModifierCategory::Inheritance,
                order: 3,
            },
            Modifier::Sealed => ModifierInfo {
                category: ModifierCategory::Inheritance,
                order: 3,
            },
            Modifier::Virtual => ModifierInfo {
                category: ModifierCategory::Inheritance,
                order: 3,
            },
            Modifier::Override => ModifierInfo {
                category: ModifierCategory::Inheritance,
                order: 3,
            },

            // State modifiers (should be fourth)
            Modifier::Readonly => ModifierInfo {
                category: ModifierCategory::State,
                order: 4,
            },
            Modifier::Volatile => ModifierInfo {
                category: ModifierCategory::State,
                order: 4,
            },
            Modifier::Const => ModifierInfo {
                category: ModifierCategory::State,
                order: 4,
            },

            // Special modifiers (can come after state modifiers)
            Modifier::Unsafe => ModifierInfo {
                category: ModifierCategory::Special,
                order: 5,
            },
            Modifier::Extern => ModifierInfo {
                category: ModifierCategory::Special,
                order: 5,
            },
            Modifier::New => ModifierInfo {
                category: ModifierCategory::Special,
                order: 5,
            },
            Modifier::Partial => ModifierInfo {
                category: ModifierCategory::Special,
                order: 5,
            },
            Modifier::Async => ModifierInfo {
                category: ModifierCategory::Special,
                order: 5,
            },
            Modifier::Required => ModifierInfo {
                category: ModifierCategory::Special,
                order: 5,
            },

            // Parameter modifiers (usually in parameter lists)
            Modifier::Ref => ModifierInfo {
                category: ModifierCategory::Parameter,
                order: 6,
            },
            Modifier::Out => ModifierInfo {
                category: ModifierCategory::Parameter,
                order: 6,
            },
            Modifier::In => ModifierInfo {
                category: ModifierCategory::Parameter,
                order: 6,
            },
            Modifier::Params => ModifierInfo {
                category: ModifierCategory::Parameter,
                order: 6,
            },

            // Storage modifiers
            Modifier::Fixed => ModifierInfo {
                category: ModifierCategory::Storage,
                order: 7,
            },
        }
    }

    /// Check if two modifiers are incompatible
    pub fn is_incompatible_with(&self, other: &Modifier) -> bool {
        match (self, other) {
            // Access modifiers are incompatible with each other
            (Modifier::Public, Modifier::Private) | (Modifier::Private, Modifier::Public) => true,
            (Modifier::Public, Modifier::Protected) | (Modifier::Protected, Modifier::Public) => {
                true
            }
            (Modifier::Public, Modifier::Internal) | (Modifier::Internal, Modifier::Public) => true,
            (Modifier::Private, Modifier::Protected) | (Modifier::Protected, Modifier::Private) => {
                true
            }
            (Modifier::Private, Modifier::Internal) | (Modifier::Internal, Modifier::Private) => {
                true
            }
            // Protected and internal can be combined as "protected internal"

            // Inheritance modifiers are incompatible with each other
            (Modifier::Abstract, Modifier::Sealed) | (Modifier::Sealed, Modifier::Abstract) => true,
            (Modifier::Abstract, Modifier::Static) | (Modifier::Static, Modifier::Abstract) => true,
            (Modifier::Abstract, Modifier::Virtual) | (Modifier::Virtual, Modifier::Abstract) => {
                true
            }
            (Modifier::Abstract, Modifier::Override) | (Modifier::Override, Modifier::Abstract) => {
                true
            }
            (Modifier::Virtual, Modifier::Sealed) | (Modifier::Sealed, Modifier::Virtual) => true,
            (Modifier::Virtual, Modifier::Static) | (Modifier::Static, Modifier::Virtual) => true,
            (Modifier::Virtual, Modifier::Override) | (Modifier::Override, Modifier::Virtual) => {
                true
            }
            (Modifier::Override, Modifier::Sealed) | (Modifier::Sealed, Modifier::Override) => true,
            (Modifier::Override, Modifier::Static) | (Modifier::Static, Modifier::Override) => true,

            // Parameter modifiers are incompatible with each other
            (Modifier::Ref, Modifier::Out) | (Modifier::Out, Modifier::Ref) => true,
            (Modifier::Ref, Modifier::In) | (Modifier::In, Modifier::Ref) => true,
            (Modifier::Ref, Modifier::Params) | (Modifier::Params, Modifier::Ref) => true,
            (Modifier::Out, Modifier::In) | (Modifier::In, Modifier::Out) => true,
            (Modifier::Out, Modifier::Params) | (Modifier::Params, Modifier::Out) => true,
            (Modifier::In, Modifier::Params) | (Modifier::Params, Modifier::In) => true,

            // Other incompatible combinations
            (Modifier::Readonly, Modifier::Static) | (Modifier::Static, Modifier::Readonly) => true,
            (Modifier::Const, Modifier::Static) | (Modifier::Static, Modifier::Const) => true,
            (Modifier::Const, Modifier::Readonly) | (Modifier::Readonly, Modifier::Const) => true,

            // Otherwise, modifiers are compatible
            _ => false,
        }
    }

    /// Get a list of compatible modifiers that can be applied to a specific declaration type
    pub fn get_compatible_modifiers_for(declaration_type: &str) -> Vec<Modifier> {
        match declaration_type {
            "class" => vec![
                Modifier::Public,
                Modifier::Private,
                Modifier::Protected,
                Modifier::Internal,
                Modifier::Static,
                Modifier::Abstract,
                Modifier::Sealed,
                Modifier::Partial,
                Modifier::New,
            ],
            "struct" => vec![
                Modifier::Public,
                Modifier::Private,
                Modifier::Protected,
                Modifier::Internal,
                Modifier::Static,
                Modifier::Readonly,
                Modifier::Partial,
                Modifier::New,
                Modifier::Unsafe,
            ],
            "interface" => vec![
                Modifier::Public,
                Modifier::Private,
                Modifier::Protected,
                Modifier::Internal,
                Modifier::Partial,
                Modifier::New,
            ],
            "record" => vec![
                Modifier::Public,
                Modifier::Private,
                Modifier::Protected,
                Modifier::Internal,
                Modifier::Static,
                Modifier::Abstract,
                Modifier::Sealed,
                Modifier::Partial,
                Modifier::New,
            ],
            "enum" => vec![
                Modifier::Public,
                Modifier::Private,
                Modifier::Protected,
                Modifier::Internal,
                Modifier::New,
            ],
            "method" => vec![
                Modifier::Public,
                Modifier::Private,
                Modifier::Protected,
                Modifier::Internal,
                Modifier::Static,
                Modifier::Abstract,
                Modifier::Virtual,
                Modifier::Override,
                Modifier::Extern,
                Modifier::Unsafe,
                Modifier::New,
                Modifier::Async,
                Modifier::Partial,
            ],
            "property" => vec![
                Modifier::Public,
                Modifier::Private,
                Modifier::Protected,
                Modifier::Internal,
                Modifier::Static,
                Modifier::Abstract,
                Modifier::Virtual,
                Modifier::Override,
                Modifier::New,
                Modifier::Readonly,
                Modifier::Required,
            ],
            "field" => vec![
                Modifier::Public,
                Modifier::Private,
                Modifier::Protected,
                Modifier::Internal,
                Modifier::Static,
                Modifier::Readonly,
                Modifier::Volatile,
                Modifier::New,
                Modifier::Const,
                Modifier::Required,
            ],
            "parameter" => vec![Modifier::Ref, Modifier::Out, Modifier::In, Modifier::Params],
            "constructor" => vec![
                Modifier::Public,
                Modifier::Private,
                Modifier::Protected,
                Modifier::Internal,
                Modifier::Static,
            ],
            "delegate" => vec![
                Modifier::Public,
                Modifier::Private,
                Modifier::Protected,
                Modifier::Internal,
                Modifier::New,
                Modifier::Unsafe,
            ],
            "event" => vec![
                Modifier::Public,
                Modifier::Private,
                Modifier::Protected,
                Modifier::Internal,
                Modifier::Static,
                Modifier::Abstract,
                Modifier::Virtual,
                Modifier::Override,
                Modifier::New,
                Modifier::Sealed,
            ],
            "event_accessor" => vec![
                // Event accessors typically don't have modifiers
            ],
            _ => Vec::new(),
        }
    }

    /// Order modifiers according to C# conventional ordering
    pub fn order_modifiers(modifiers: &mut Vec<Modifier>) {
        modifiers.sort_by(|a, b| {
            let a_info = a.get_info();
            let b_info = b.get_info();
            a_info.order.cmp(&b_info.order)
        });
    }
}
