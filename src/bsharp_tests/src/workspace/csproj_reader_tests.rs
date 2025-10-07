use analysis::workspace::csproj::CsprojReader;
use analysis::workspace::{Language, ProjectFileKind};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let mut dir = std::env::temp_dir();
    dir.push(format!("{}_{}", prefix, nanos));
    dir
}

fn write(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let mut f = fs::File::create(path).unwrap();
    f.write_all(content.as_bytes()).unwrap();
}

#[test]
fn sdk_style_defaults_collect_cs_files_and_exclude_common_dirs() {
    let root = unique_temp_dir("bsharp_csproj_defaults");
    fs::create_dir_all(&root).unwrap();

    // Create a few files
    let f1 = root.join("Program.cs");
    write(&f1, "class Program { static void Main() {} }");
    let f2 = root.join("src/Utils.cs");
    write(&f2, "class Utils {}");
    let bin_cs = root.join("bin/Debug/Temp.cs");
    write(&bin_cs, "class Bin {}");
    let obj_cs = root.join("obj/Gen.cs");
    write(&obj_cs, "class Obj {}");

    let csproj_path = root.join("App.csproj");
    write(
        &csproj_path,
        r#"<Project Sdk="Microsoft.NET.Sdk"></Project>"#,
    );

    let prj = CsprojReader::read(&csproj_path).expect("csproj parse");
    let paths: Vec<_> = prj
        .files
        .iter()
        .map(|f| f.path.canonicalize().unwrap())
        .collect();

    assert!(paths.contains(&f1.canonicalize().unwrap()));
    assert!(paths.contains(&f2.canonicalize().unwrap()));
    assert!(
        !paths.contains(&bin_cs.canonicalize().unwrap()),
        "bin/** should be excluded"
    );
    assert!(
        !paths.contains(&obj_cs.canonicalize().unwrap()),
        "obj/** should be excluded"
    );

    // Ensure kinds and language set for .cs
    for pf in &prj.files {
        if pf.path.extension().and_then(|s| s.to_str()).unwrap_or("") == "cs" {
            assert!(matches!(pf.kind, ProjectFileKind::Source));
            assert_eq!(pf.language, Some(Language::CSharp));
        }
    }
}

#[test]
fn compile_include_and_remove_patterns_are_applied() {
    let root = unique_temp_dir("bsharp_csproj_patterns");
    fs::create_dir_all(&root).unwrap();

    let keep_a = root.join("A.cs");
    let remove_b = root.join("B.cs");
    write(&keep_a, "class A {}");
    write(&remove_b, "class B {}");

    let csproj_path = root.join("App.csproj");
    write(
        &csproj_path,
        r#"<Project Sdk="Microsoft.NET.Sdk">
  <ItemGroup>
    <Compile Include="A.cs" />
    <Compile Remove="B.cs" />
  </ItemGroup>
</Project>"#,
    );

    let prj = CsprojReader::read(&csproj_path).expect("csproj parse");
    let paths: Vec<_> = prj
        .files
        .iter()
        .map(|f| f.path.file_name().unwrap().to_string_lossy().to_string())
        .collect();
    assert!(paths.contains(&"A.cs".to_string()));
    assert!(!paths.contains(&"B.cs".to_string()));
}

#[test]
fn project_reference_is_resolved_to_absolute_path_or_warns() {
    let root = unique_temp_dir("bsharp_csproj_refs");
    fs::create_dir_all(&root).unwrap();

    // referenced project
    let ref_dir = root.join("lib");
    fs::create_dir_all(&ref_dir).unwrap();
    let ref_csproj = ref_dir.join("Lib.csproj");
    write(&ref_csproj, "<Project Sdk=\"Microsoft.NET.Sdk\"></Project>");

    // main project
    let csproj_path = root.join("App.csproj");
    write(
        &csproj_path,
        r#"<Project Sdk="Microsoft.NET.Sdk">
  <ItemGroup>
    <ProjectReference Include="lib/Lib.csproj" />
  </ItemGroup>
</Project>"#,
    );

    let prj = CsprojReader::read(&csproj_path).expect("csproj parse");
    assert!(
        prj.project_references
            .iter()
            .any(|p| p.canonicalize().unwrap() == ref_csproj.canonicalize().unwrap())
    );
}
