/// provides way of reading sln and csproj files
pub mod error;
pub mod model;
pub mod source_map;
pub mod sln;
pub mod csproj;
pub mod loader;

pub use loader::WorkspaceLoader;
pub use model::{Workspace, Project, Solution, ProjectFile, ProjectRef, Language, ProjectFileKind};