use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};

// Get the path to test cases directory
pub fn get_test_cases_dir() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src");
    path.push("cs_test_cases");
    path
}

// Get all .cs files in the test directory
pub fn get_all_test_files() -> Vec<PathBuf> {
    let test_dir = get_test_cases_dir();
    let entries = fs::read_dir(&test_dir).expect("Failed to read test directory");

    entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "cs" {
                Some(path)
            } else {
                None
            }
        })
        .collect()
}

fn global_counter() -> &'static AtomicU64 {
    static COUNTER: OnceLock<AtomicU64> = OnceLock::new();
    COUNTER.get_or_init(|| AtomicU64::new(0))
}

fn next_unique_id() -> u64 {
    global_counter().fetch_add(1, Ordering::Relaxed)
}

// Create a temporary directory for test outputs
pub fn create_temp_dir() -> io::Result<PathBuf> {
    let mut temp_dir_base = env::temp_dir();
    let pid = process::id();
    let unique = next_unique_id();
    let suffix = format!("bsharp_cli_tests_{}_{}", pid, unique);
    temp_dir_base.push(suffix);

    fs::create_dir_all(&temp_dir_base)?;

    Ok(temp_dir_base)
}

// Clean up temporary files after tests
pub fn clean_temp_files(temp_dir: &Path) -> io::Result<()> {
    if temp_dir.exists() {
        fs::remove_dir_all(temp_dir)?;
    }
    Ok(())
}
