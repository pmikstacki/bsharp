use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use bsharp::workspace::sln::SolutionReader;

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let mut dir = std::env::temp_dir();
    dir.push(format!("{}_{}", prefix, nanos));
    dir
}

fn write(path: &Path, content: &str) {
    if let Some(parent) = path.parent() { fs::create_dir_all(parent).unwrap(); }
    let mut f = fs::File::create(path).unwrap();
    f.write_all(content.as_bytes()).unwrap();
}

#[test]
fn reads_solution_and_resolves_csproj_paths() {
    let root = unique_temp_dir("bsharp_sln_test");
    fs::create_dir_all(&root).unwrap();

    let csproj_path = root.join("src/App/App.csproj");
    write(&csproj_path, r#"<Project Sdk=\"Microsoft.NET.Sdk\"></Project>"#);

    let sln_path = root.join("MySolution.sln");
    let sln_content = r#"Microsoft Visual Studio Solution File, Format Version 12.00
Project("{FAE04EC0-301F-11D3-BF4B-00C04F79EFBC}") = "App", "src\App\App.csproj", "{11111111-1111-1111-1111-111111111111}"
EndProject
Global
EndGlobal
"#;
    write(&sln_path, &sln_content);

    let solution = SolutionReader::read(&sln_path).expect("solution parse");
    assert_eq!(solution.projects.len(), 1, "should have 1 C# project");
    assert!(solution.errors.is_empty(), "no errors resolving csproj path");
    assert_eq!(solution.projects[0].name, "App");
    assert_eq!(solution.projects[0].path.canonicalize().unwrap(), csproj_path.canonicalize().unwrap());
}

#[test]
fn filters_non_csproj_projects() {
    let root = unique_temp_dir("bsharp_sln_filter_test");
    fs::create_dir_all(&root).unwrap();

    let csproj_path = root.join("a/a.csproj");
    write(&csproj_path, "<Project Sdk=\"Microsoft.NET.Sdk\"></Project>");

    let sln_path = root.join("mix.sln");
    let sln_content = r#"Microsoft Visual Studio Solution File, Format Version 12.00
Project("{FAE04EC0-301F-11D3-BF4B-00C04F79EFBC}") = "A", "a\\a.csproj", "{11111111-1111-1111-1111-111111111111}"
EndProject
Project("{8BC9CEB8-8B4A-11D0-8D11-00A0C91BC942}") = "Native", "c\\c.vcxproj", "{22222222-2222-2222-2222-222222222222}"
EndProject
Global
EndGlobal
"#;
    write(&sln_path, sln_content);

    let solution = SolutionReader::read(&sln_path).expect("solution parse");
    assert_eq!(solution.projects.len(), 1, "non-csproj should be filtered out");
    assert!(solution.errors.is_empty());
}
