use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use bsharp::workspace::WorkspaceLoader;

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
fn loader_from_csproj_single_project() {
    let root = unique_temp_dir("bsharp_loader_csproj");
    fs::create_dir_all(&root).unwrap();

    let code = root.join("Program.cs");
    write(&code, "class Program { static void Main() {} }");

    let csproj = root.join("App.csproj");
    write(&csproj, "<Project Sdk=\"Microsoft.NET.Sdk\"></Project>");

    let ws = WorkspaceLoader::from_path(&csproj).expect("workspace from csproj");
    assert_eq!(ws.projects.len(), 1);
    assert_eq!(ws.solution.is_none(), true);
    assert!(ws.projects[0].files.iter().any(|f| f.path.ends_with("Program.cs")));
}

#[test]
fn loader_from_sln_with_transitive_project_references() {
    let root = unique_temp_dir("bsharp_loader_sln");
    fs::create_dir_all(&root).unwrap();

    // proj A references B, B references C
    let proj_a = root.join("A/A.csproj");
    write(&proj_a, r#"<Project Sdk="Microsoft.NET.Sdk"><ItemGroup><ProjectReference Include="../B/B.csproj" /></ItemGroup></Project>"#);
    write(&root.join("A/A.cs"), "class A {}");

    let proj_b = root.join("B/B.csproj");
    write(&proj_b, r#"<Project Sdk="Microsoft.NET.Sdk"><ItemGroup><ProjectReference Include="../C/C.csproj" /></ItemGroup></Project>"#);
    write(&root.join("B/B.cs"), "class B {}");

    let proj_c = root.join("C/C.csproj");
    write(&proj_c, r#"<Project Sdk="Microsoft.NET.Sdk"></Project>"#);
    write(&root.join("C/C.cs"), "class C {}");

    // solution contains only A explicitly
    let sln = root.join("All.sln");
    write(&sln, "Microsoft Visual Studio Solution File, Format Version 12.00\nProject(\"{FAE04EC0-301F-11D3-BF4B-00C04F79EFBC}\") = \"A\", \"A\\A.csproj\", \"{11111111-1111-1111-1111-111111111111}\"\nEndProject\nGlobal\nEndGlobal\n");

    let ws = WorkspaceLoader::from_path(&sln).expect("workspace from sln");
    // Should include A, and follow B and C transitively
    assert_eq!(ws.projects.len(), 3);
    let names: Vec<_> = ws.projects.iter().map(|p| p.name.as_str()).collect();
    assert!(names.contains(&"A"));
    assert!(names.contains(&"B"));
    assert!(names.contains(&"C"));
}

#[test]
fn loader_from_dir_prefers_sln() {
    let root = unique_temp_dir("bsharp_loader_dir");
    fs::create_dir_all(&root).unwrap();

    // create a csproj under dir
    let csproj = root.join("App.csproj");
    write(&csproj, "<Project Sdk=\"Microsoft.NET.Sdk\"></Project>");
    write(&root.join("App.cs"), "class App {}");

    // also create a sln that points to the csproj
    let sln = root.join("App.sln");
    write(&sln, "Microsoft Visual Studio Solution File, Format Version 12.00\nProject(\"{FAE04EC0-301F-11D3-BF4B-00C04F79EFBC}\") = \"App\", \"App.csproj\", \"{11111111-1111-1111-1111-111111111111}\"\nEndProject\nGlobal\nEndGlobal\n");

    let ws = WorkspaceLoader::from_path(&root).expect("workspace from dir");
    assert!(ws.solution.is_some(), "should prefer solution if present");
    assert_eq!(ws.projects.len(), 1);
}
