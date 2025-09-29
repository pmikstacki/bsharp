use std::path::PathBuf;

use bsharp::workspace::csproj::CsprojReader;

#[test]
fn complex_csproj_include_remove_update_and_condition_and_macro_warnings() {
    // Arrange
    let csproj: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "fixtures", "complex", "testApplication", "testApplication.csproj"].iter().collect();

    // Act
    let prj = CsprojReader::read(&csproj).expect("read complex csproj");

    // Collect file names
    let files: Vec<String> = prj
        .files
        .iter()
        .map(|f| f.path.file_name().unwrap().to_string_lossy().to_string())
        .collect();

    // Assert includes
    assert!(files.contains(&"Extra.cs".to_string()), "Extra.cs should be included via <Compile Include>");
    assert!(files.contains(&"Updated.cs".to_string()), "Updated.cs should be included via <Compile Update>");
    assert!(files.contains(&"Conditional.cs".to_string()), "Conditional.cs should be included even if Condition not evaluated");
    assert!(files.contains(&"Include.cs".to_string()), "Include.cs should be included via overlapping Include pattern");

    // Assert removes
    assert!(!files.contains(&"Removed.cs".to_string()), "Removed.cs should be excluded via <Compile Remove>");
    assert!(!files.contains(&"Exclude.cs".to_string()), "Exclude.cs should be removed explicitly from overlap pattern");

    // Assert warnings for Condition and macro
    let errs_joined = prj.errors.join("\n");
    assert!(errs_joined.contains("MSBuild Condition not evaluated in v1"), "should warn about Condition not evaluated");
    assert!(errs_joined.contains("MSBuild macro not expanded in v1"), "should warn about MSBuild macro not expanded");
}
