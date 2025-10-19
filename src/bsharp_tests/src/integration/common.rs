use std::path::PathBuf;

pub fn repo_fixtures_root() -> PathBuf {
    // Fixtures for tests live in this crate under src/fixtures
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("src");
    p.push("fixtures");
    p
}

pub fn happy_path_sln() -> PathBuf {
    repo_fixtures_root()
        .join("happy_path")
        .join("testSolution.sln")
}

pub fn happy_path_app_csproj() -> PathBuf {
    repo_fixtures_root()
        .join("happy_path")
        .join("testApplication")
        .join("testApplication.csproj")
}

pub fn complex_app_csproj() -> PathBuf {
    repo_fixtures_root()
        .join("complex")
        .join("testApplication")
        .join("testApplication.csproj")
}
