# Workspace Loading

The BSharp workspace loading system provides comprehensive support for loading C# projects and solutions, including solution files (.sln), project files (.csproj), and directory-based discovery.

---

## Overview

**Location:** `src/bsharp_analysis/src/workspace/`

The workspace loader:
- Parses Visual Studio solution files (.sln)
- Parses MSBuild project files (.csproj)
- Discovers source files
- Resolves project references
- Handles multiple projects deterministically

---

## Workspace Model

### Core Types

```rust
pub struct Workspace {
    pub root: PathBuf,
    pub projects: Vec<Project>,
    pub solution: Option<Solution>,
    pub source_map: SourceMap,
}

pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub target_framework: String,
    pub output_type: String,
    pub files: Vec<ProjectFile>,
    pub references: Vec<ProjectRef>,
    pub package_references: Vec<PackageReference>,
    pub errors: Vec<String>,
}

pub struct Solution {
    pub name: String,
    pub path: PathBuf,
    pub projects: Vec<SolutionProject>,
}
```

---

## Loading Workspaces

### WorkspaceLoader API

```rust
pub struct WorkspaceLoader;

impl WorkspaceLoader {
    // Load from any path (auto-detects type)
    pub fn from_path(path: &Path) -> Result<Workspace>;
    
    // Load with options
    pub fn from_path_with_options(
        path: &Path, 
        opts: WorkspaceLoadOptions
    ) -> Result<Workspace>;
}

pub struct WorkspaceLoadOptions {
    pub follow_refs: bool,  // Follow ProjectReference transitively
}
```

### Loading from Solution File

```rust
use bsharp_analysis::workspace::WorkspaceLoader;

let workspace = WorkspaceLoader::from_path(Path::new("MySolution.sln"))?;

println!("Loaded {} projects", workspace.projects.len());
for project in &workspace.projects {
    println!("  - {}: {} files", project.name, project.files.len());
}
```

### Loading from Project File

```rust
let workspace = WorkspaceLoader::from_path(Path::new("MyProject.csproj"))?;

// Automatically follows ProjectReference if follow_refs = true
assert!(workspace.projects.len() >= 1);
```

### Loading from Directory

```rust
let workspace = WorkspaceLoader::from_path(Path::new("./src"))?;

// Discovers .sln or .csproj files in directory
```

---

## Solution File Parsing

### Solution Format

**Example .sln:**
```
Microsoft Visual Studio Solution File, Format Version 12.00
Project("{FAE04EC0-301F-11D3-BF4B-00C04F79EFBC}") = "MyApp", "MyApp\MyApp.csproj", "{GUID}"
EndProject
Project("{FAE04EC0-301F-11D3-BF4B-00C04F79EFBC}") = "MyLib", "MyLib\MyLib.csproj", "{GUID}"
EndProject
```

### Parsing Implementation

**Location:** `src/bsharp_analysis/src/workspace/sln/reader.rs`

```rust
pub struct SolutionReader;

impl SolutionReader {
    pub fn read(path: &Path) -> Result<Solution> {
        let content = fs::read_to_string(path)?;
        Self::parse(&content, path)
    }
    
    fn parse(content: &str, base_path: &Path) -> Result<Solution> {
        // Parse solution format
        // Extract project entries
        // Resolve project paths
    }
}
```

### Solution Structure

```rust
pub struct Solution {
    pub name: String,
    pub path: PathBuf,
    pub projects: Vec<SolutionProject>,
}

pub struct SolutionProject {
    pub name: String,
    pub path: PathBuf,
    pub type_guid: String,
    pub guid: String,
}
```

---

## Project File Parsing

### Project Format

**Example .csproj:**
```xml
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <TargetFramework>net8.0</TargetFramework>
    <OutputType>Exe</OutputType>
  </PropertyGroup>
  
  <ItemGroup>
    <Compile Include="Program.cs" />
    <Compile Include="Utils.cs" />
  </ItemGroup>
  
  <ItemGroup>
    <ProjectReference Include="..\MyLib\MyLib.csproj" />
  </ItemGroup>
  
  <ItemGroup>
    <PackageReference Include="Newtonsoft.Json" Version="13.0.1" />
  </ItemGroup>
</Project>
```

### Parsing Implementation

**Location:** `src/bsharp_analysis/src/workspace/csproj/reader.rs`

```rust
pub struct CsprojReader;

impl CsprojReader {
    pub fn read(path: &Path) -> Result<Project> {
        let content = fs::read_to_string(path)?;
        Self::parse(&content, path)
    }
    
    fn parse(content: &str, project_path: &Path) -> Result<Project> {
        // Parse XML
        // Extract properties (TargetFramework, OutputType)
        // Discover source files (Compile items)
        // Extract ProjectReference entries
        // Extract PackageReference entries
    }
}
```

### Source File Discovery

**Glob Patterns:**
- Default: `**/*.cs` (all C# files recursively)
- Respects `<Compile Include="..." />` items
- Respects `<Compile Remove="..." />` exclusions
- Excludes `obj/` and `bin/` directories

**Implementation:**
```rust
fn discover_source_files(project_dir: &Path) -> Vec<ProjectFile> {
    let pattern = project_dir.join("**/*.cs");
    let mut files = Vec::new();
    
    for entry in glob::glob(pattern.to_str().unwrap()) {
        let path = entry.unwrap();
        
        // Skip obj/ and bin/
        if path.components().any(|c| c.as_os_str() == "obj" || c.as_os_str() == "bin") {
            continue;
        }
        
        files.push(ProjectFile {
            path,
            kind: ProjectFileKind::Compile,
        });
    }
    
    files
}
```

---

## Project References

### Transitive Resolution

**follow_refs Option:**
```rust
let opts = WorkspaceLoadOptions { follow_refs: true };
let workspace = WorkspaceLoader::from_path_with_options(path, opts)?;
```

**Behavior:**
- Follows `<ProjectReference>` transitively
- Loads all referenced projects
- Avoids duplicates
- Stays within workspace root
- Deterministic ordering (sorted by path)

**Example:**
```
MyApp.csproj
  → MyLib.csproj
    → MyCore.csproj

Result: [MyApp, MyLib, MyCore]
```

### Implementation

```rust
fn follow_project_references(root: &Path, projects: &mut Vec<Project>) {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    
    // Add initial projects
    for proj in projects.iter() {
        seen.insert(proj.path.clone());
        queue.push_back(proj.path.clone());
    }
    
    // BFS traversal
    while let Some(proj_path) = queue.pop_front() {
        let proj = match CsprojReader::read(&proj_path) {
            Ok(p) => p,
            Err(_) => continue,
        };
        
        for ref_path in proj.references.iter().map(|r| &r.path) {
            // Resolve relative to project directory
            let abs_path = proj_path.parent().unwrap().join(ref_path);
            
            // Skip if outside root
            if !abs_path.starts_with(root) {
                continue;
            }
            
            // Skip if already seen
            if seen.insert(abs_path.clone()) {
                queue.push_back(abs_path.clone());
                
                // Load and add project
                if let Ok(referenced_proj) = CsprojReader::read(&abs_path) {
                    projects.push(referenced_proj);
                }
            }
        }
    }
    
    // Sort for determinism
    projects.sort_by(|a, b| a.path.cmp(&b.path));
}
```

---

## Source Map

### Purpose

The `SourceMap` provides fast lookup of source files:

```rust
pub struct SourceMap {
    files: HashMap<PathBuf, SourceFileInfo>,
}

impl SourceMap {
    pub fn get(&self, path: &Path) -> Option<&SourceFileInfo>;
    pub fn all_files(&self) -> Vec<&Path>;
}
```

### Usage

```rust
let workspace = WorkspaceLoader::from_path(path)?;

// Look up file
if let Some(info) = workspace.source_map.get(Path::new("Program.cs")) {
    println!("Found in project: {}", info.project_name);
}

// Iterate all files
for file_path in workspace.source_map.all_files() {
    println!("File: {}", file_path.display());
}
```

---

## Error Handling

### Resilient Loading

**Philosophy:** Continue loading even if individual projects fail

```rust
// Failed projects recorded as stubs with errors
let workspace = WorkspaceLoader::from_path(sln_path)?;

for project in &workspace.projects {
    if !project.errors.is_empty() {
        eprintln!("Errors in {}: {:?}", project.name, project.errors);
    }
}
```

### Error Types

```rust
pub enum WorkspaceError {
    IoError(io::Error),
    ParseError(String),
    InvalidPath(String),
    Unsupported(String),
}
```

---

## CLI Integration

### Analyze Command

```bash
# Analyze solution
bsharp analyze MySolution.sln

# Analyze project
bsharp analyze MyProject.csproj

# Follow references (default: true)
bsharp analyze MyProject.csproj --follow-refs true

# Don't follow references
bsharp analyze MyProject.csproj --follow-refs false
```

### Filtering

```bash
# Include only specific files
bsharp analyze MySolution.sln --include "**/*Service.cs"

# Exclude test files
bsharp analyze MySolution.sln --exclude "**/Tests/**"

# Multiple patterns
bsharp analyze MySolution.sln \
    --include "src/**/*.cs" \
    --exclude "**/obj/**" "**/bin/**"
```

---

## Deterministic Behavior

### Guarantees

1. **Project Order:** Always sorted by absolute path
2. **File Order:** Always sorted within each project
3. **Deduplication:** No duplicate projects or files
4. **Reproducible:** Same input always produces same output

### Implementation

```rust
// Sort projects
projects.sort_by(|a, b| a.path.cmp(&b.path));

// Deduplicate by path
let mut seen = HashSet::new();
projects.retain(|p| seen.insert(p.path.clone()));

// Sort files within each project
for project in &mut projects {
    project.files.sort_by(|a, b| a.path.cmp(&b.path));
}
```

---

## Performance

### Loading Speed

- **Small solution** (1-5 projects): < 100ms
- **Medium solution** (5-20 projects): 100-500ms
- **Large solution** (20-100 projects): 500ms-2s

### Memory Usage

- Minimal: Only metadata loaded, not source content
- Typical: 1-5 MB per solution

### Optimization

- Parallel project loading (with `parallel_analysis` feature)
- Lazy source file reading
- Efficient path canonicalization

---

## Examples

### Example 1: Load and Analyze

```rust
use bsharp_analysis::workspace::WorkspaceLoader;
use bsharp_parser::facade::Parser;

let workspace = WorkspaceLoader::from_path(Path::new("MySolution.sln"))?;

let parser = Parser::new();
for project in &workspace.projects {
    for file in &project.files {
        let source = fs::read_to_string(&file.path)?;
        match parser.parse(&source) {
            Ok(cu) => println!("Parsed: {}", file.path.display()),
            Err(e) => eprintln!("Error in {}: {}", file.path.display(), e),
        }
    }
}
```

### Example 2: Project Statistics

```rust
let workspace = WorkspaceLoader::from_path(path)?;

println!("Solution: {}", workspace.solution.as_ref().unwrap().name);
println!("Projects: {}", workspace.projects.len());

let total_files: usize = workspace.projects.iter()
    .map(|p| p.files.len())
    .sum();
println!("Total files: {}", total_files);

for project in &workspace.projects {
    println!("  {}: {} files", project.name, project.files.len());
}
```

### Example 3: Dependency Graph

```rust
let workspace = WorkspaceLoader::from_path(path)?;

println!("Project Dependencies:");
for project in &workspace.projects {
    if !project.references.is_empty() {
        println!("{}:", project.name);
        for ref_ in &project.references {
            println!("  → {}", ref_.name);
        }
    }
}
```

---

## Testing

### Test Fixtures

**Location:** `tests/fixtures/`

```
tests/fixtures/
├── happy_path/
│   ├── test.sln
│   ├── testApplication/
│   │   ├── testApplication.csproj
│   │   └── Program.cs
│   └── testDependency/
│       ├── testDependency.csproj
│       └── Library.cs
└── complex/
    └── ...
```

### Test Examples

```rust
#[test]
fn test_load_solution() {
    let sln_path = PathBuf::from("tests/fixtures/happy_path/test.sln");
    let workspace = WorkspaceLoader::from_path(&sln_path).unwrap();
    
    assert_eq!(workspace.projects.len(), 2);
    assert!(workspace.solution.is_some());
}

#[test]
fn test_follow_references() {
    let proj_path = PathBuf::from("tests/fixtures/happy_path/testApplication/testApplication.csproj");
    let workspace = WorkspaceLoader::from_path(&proj_path).unwrap();
    
    // Should load both testApplication and testDependency
    assert_eq!(workspace.projects.len(), 2);
}
```

---

## Future Enhancements

### Planned Features

1. **NuGet Package Resolution**
   - Resolve package references
   - Download packages if needed
   - Parse package assemblies

2. **MSBuild Integration**
   - Full MSBuild evaluation
   - Property expansion
   - Target execution

3. **Multi-targeting Support**
   - Handle multiple target frameworks
   - Conditional compilation

4. **Incremental Loading**
   - Cache workspace metadata
   - Reload only changed projects

---

## Related Documentation

- [CLI Overview](../cli/overview.md) - CLI integration
- [Analysis Pipeline](../analysis/pipeline.md) - Using workspace in analysis
- [Architecture](../development/architecture.md) - Design decisions

---

## References

- **Implementation:** `src/bsharp_analysis/src/workspace/`
- **Loader:** `src/bsharp_analysis/src/workspace/loader.rs`
- **Solution Reader:** `src/bsharp_analysis/src/workspace/sln/reader.rs`
- **Project Reader:** `src/bsharp_analysis/src/workspace/csproj/reader.rs`
- **Model:** `src/bsharp_analysis/src/workspace/model.rs`
- **Source Map:** `src/bsharp_analysis/src/workspace/source_map.rs`
- **Tests:** `src/bsharp_tests/src/workspace/` and `src/bsharp_tests/src/integration/`
