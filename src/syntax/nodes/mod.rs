// Declare the subdirectories as modules
pub mod declarations;
pub mod expressions;
pub mod identifier;
pub mod statements;
pub mod types;
pub use identifier::Identifier;
pub mod preprocessor; // Declare preprocessor module
pub mod xml_documentation; // Added for XML documentation

// Optional: Re-export all public items from submodules for easier access
// pub use types::*;
// pub use declarations::*;
// pub use statements::*;
// pub use expressions::*;

// Re-export key items explicitly for clarity
// pub use types::*; // Can re-export common types later if needed
// Decide later if wholesale re-exporting is desired or if specific imports are better.
pub use xml_documentation::*;
