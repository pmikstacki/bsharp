use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use std::io;

// Get the path to test cases directory
pub fn get_test_cases_dir() -> PathBuf {
    let mut path = env::current_dir().expect("Failed to get current directory");
    path.push("tests");
    path.push("cs_test_cases");
    path
}

// Get all .cs files in the test directory
pub fn get_all_test_files() -> Vec<PathBuf> {
    let test_dir = get_test_cases_dir();
    let entries = fs::read_dir(&test_dir).expect("Failed to read test directory");
    
    entries.filter_map(|entry| {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.extension()? == "cs" {
            Some(path)
        } else {
            None
        }
    }).collect()
}

// Create a temporary directory for test outputs
pub fn create_temp_dir() -> io::Result<PathBuf> {
    let mut temp_dir = env::temp_dir();
    temp_dir.push("bsharp_cli_tests");
    
    if !temp_dir.exists() {
        fs::create_dir_all(&temp_dir)?;
    }
    
    Ok(temp_dir)
}

// Clean up temporary files after tests
pub fn clean_temp_files(temp_dir: &Path) -> io::Result<()> {
    if temp_dir.exists() {
        fs::remove_dir_all(temp_dir)?;
    }
    Ok(())
}
