pub mod csproj;
/// provides way of reading sln and csproj files
pub mod error;
pub mod loader;
pub mod model;
pub mod sln;
pub mod source_map;

pub use loader::WorkspaceLoader;
pub use model::{Language, Project, ProjectFile, ProjectFileKind, ProjectRef, Solution, Workspace};
