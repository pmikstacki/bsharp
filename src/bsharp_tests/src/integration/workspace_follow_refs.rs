use std::path::PathBuf;

use analysis::workspace::loader::WorkspaceLoadOptions;
use analysis::workspace::WorkspaceLoader;
use crate::integration::common::happy_path_app_csproj;

#[test]
fn loader_respects_follow_refs_flag_for_csproj() {
    // Arrange: load the application csproj directly (it references testDependency)
    let app_csproj: PathBuf = happy_path_app_csproj();

    // Act: follow_refs = false
    let ws_no_refs = WorkspaceLoader::from_path_with_options(
        &app_csproj,
        WorkspaceLoadOptions { follow_refs: false },
    )
    .expect("workspace without following refs");
    // Act: follow_refs = true
    let ws_with_refs = WorkspaceLoader::from_path_with_options(
        &app_csproj,
        WorkspaceLoadOptions { follow_refs: true },
    )
    .expect("workspace with following refs");

    // Assert: with refs we should see more or equal projects/files than without
    assert!(
        ws_with_refs.projects.len() >= ws_no_refs.projects.len(),
        "following refs should not reduce project count"
    );
    let files_no_refs = ws_no_refs.all_source_files().len();
    let files_with_refs = ws_with_refs.all_source_files().len();
    assert!(
        files_with_refs >= files_no_refs,
        "following refs should not reduce source file count"
    );

    // Expect strictly more when dependency project has sources
    assert!(
        files_with_refs > files_no_refs,
        "expected more files when following project references"
    );
}
