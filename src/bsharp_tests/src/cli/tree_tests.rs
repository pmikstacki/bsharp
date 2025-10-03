use anyhow::Result;
use std::fs;
use std::io::Read;

// Directly import the tree function to avoid module resolution issues
use super::tree_execute;
use super::utils::{clean_temp_files, create_temp_dir, get_all_test_files};

#[test]
fn test_tree_command_creates_valid_mermaid() -> Result<()> {
    // Create a temporary directory for test outputs
    let temp_dir = create_temp_dir()?;

    // Get all test files
    let test_files: Vec<_> = get_all_test_files()
        .into_iter()
        .filter(|p| p.file_name().is_some_and(|n| n != "failure.cs"))
        .collect();
    assert!(
        !test_files.is_empty(),
        "No test files found in test directory"
    );

    for test_file in test_files {
        // Define output path for this test file
        let file_name = test_file.file_name().unwrap().to_string_lossy();
        let mut output_path = temp_dir.clone();
        output_path.push(format!("{}.mmd", file_name));

        // Run the tree command
        tree_execute(
            test_file.clone(),
            Some(output_path.clone()),
            "mermaid".to_string(),
        )?;

        // Verify the output file exists
        assert!(
            output_path.exists(),
            "Output file was not created: {:?}",
            output_path
        );

        // Verify the output is a valid Mermaid graph (basic check)
        let mut file = fs::File::open(&output_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Check for basic Mermaid structure
        assert!(
            contents.contains("graph TD"),
            "Missing Mermaid graph header"
        );
        assert!(
            contents.contains("CompilationUnit"),
            "Missing root node label"
        );
    }

    // Clean up temporary files
    clean_temp_files(&temp_dir)?;

    Ok(())
}

#[test]
fn test_tree_command_handles_all_files_mermaid_and_dot() -> Result<()> {
    // Create a temporary directory for test outputs
    let temp_dir = create_temp_dir()?;

    // Get all test files (skip intentionally invalid fixture)
    let test_files: Vec<_> = get_all_test_files()
        .into_iter()
        .filter(|p| p.file_name().is_some_and(|n| n != "failure.cs"))
        .collect();

    // Test that each file can be processed without errors
    for test_file in test_files {
        let file_name = test_file.file_name().unwrap().to_string_lossy().to_string();
        // Mermaid
        let mut output_path_mmd = temp_dir.clone();
        output_path_mmd.push(format!("{}.mmd", file_name));
        let result_mmd = tree_execute(
            test_file.clone(),
            Some(output_path_mmd.clone()),
            "mermaid".to_string(),
        );
        assert!(
            result_mmd.is_ok(),
            "Failed to generate Mermaid for: {:?}",
            test_file
        );
        let metadata_mmd = fs::metadata(&output_path_mmd)?;
        assert!(metadata_mmd.len() > 0, "Generated Mermaid is empty");

        // DOT
        let mut output_path_dot = temp_dir.clone();
        output_path_dot.push(format!("{}.dot", file_name));
        let result_dot = tree_execute(
            test_file.clone(),
            Some(output_path_dot.clone()),
            "dot".to_string(),
        );
        assert!(
            result_dot.is_ok(),
            "Failed to generate DOT for: {:?}",
            test_file
        );
        let metadata_dot = fs::metadata(&output_path_dot)?;
        assert!(metadata_dot.len() > 0, "Generated DOT is empty");
    }

    // Clean up temporary files
    clean_temp_files(&temp_dir)?;

    Ok(())
}
