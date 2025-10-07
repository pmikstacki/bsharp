use analysis::workspace::WorkspaceLoader;
use std::path::PathBuf;

#[test]
fn loads_fixture_solution_projects_and_files() {
    // Arrange
    let sln: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "..",
        "..",
        "tests",
        "fixtures",
        "happy_path",
        "testSolution.sln",
    ]
    .iter()
    .collect();

    if !sln.exists() {
        eprintln!(
            "skipping fixtures_workspace_loader test (fixture missing): {}",
            sln.display()
        );
        return;
    }

    // Act
    let ws = WorkspaceLoader::from_path(&sln).expect("workspace from fixture sln");

    // Assert projects
    assert_eq!(
        ws.projects.len(),
        2,
        "expected two projects in fixture (app + dependency)"
    );

    // Assert presence of key source files
    let all_sources: Vec<_> = ws
        .all_source_files()
        .iter()
        .map(|p| p.to_path_buf())
        .collect();
    let has_program = all_sources
        .iter()
        .any(|p| p.file_name().unwrap().to_string_lossy() == "Program.cs");
    let has_dep = all_sources
        .iter()
        .any(|p| p.file_name().unwrap().to_string_lossy() == "TestDependency.cs");
    assert!(has_program, "Program.cs should be included");
    assert!(has_dep, "TestDependency.cs should be included");
}
