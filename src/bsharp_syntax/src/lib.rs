// Declare the subdirectories as modules
pub mod declarations;
pub mod expressions;
pub mod identifier;
pub mod statements;
pub mod types;
pub use identifier::Identifier;
pub mod keywords;
pub mod root;
pub mod trivia;
// Added for XML documentation

// Optional: Re-export all public items from submodules for easier access
// pub use types::*;
// pub use declarations::*;
// pub use statements::*;
// pub use expressions::*;

// Compatibility layer for older paths: `syntax::nodes::...`
pub mod nodes {
    pub mod declarations {
        pub use crate::declarations::*;
    }
    pub mod statements {
        pub use crate::statements::*;
    }
    pub mod expressions {
        pub use crate::expressions::*;
    }
    pub mod types {
        pub use crate::types::*;
    }
    pub mod identifier {
        pub use crate::identifier::*;
    }
}

// Compatibility alias so code can use `syntax::ast::...`
pub use crate::root::ast;
