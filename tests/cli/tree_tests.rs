use anyhow::Result;
use std::fs;
use std::io::Read;

// Directly import the tree function to avoid module resolution issues
use super::utils::{clean_temp_files, create_temp_dir, get_all_test_files};
use bsharp::cli::commands::tree::execute as tree_execute;

#[test]
fn test_tree_command_creates_valid_svg() -> Result<()> {
    // Create a temporary directory for test outputs
    let temp_dir = create_temp_dir()?;

    // Get all test files
    let test_files = get_all_test_files();
    assert!(
        !test_files.is_empty(),
        "No test files found in test directory"
    );

    for test_file in test_files {
        // Define output path for this test file
        let file_name = test_file.file_name().unwrap().to_string_lossy();
        let mut output_path = temp_dir.clone();
        output_path.push(format!("{}.svg", file_name));

        // Run the tree command
        tree_execute(test_file.clone(), Some(output_path.clone()))?;

        // Verify the output file exists
        assert!(
            output_path.exists(),
            "Output file was not created: {:?}",
            output_path
        );

        // Verify the output is a valid SVG (basic check)
        let mut file = fs::File::open(&output_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Check for basic SVG structure
        assert!(
            contents.contains("<svg"),
            "Output is missing SVG opening tag"
        );
        assert!(
            contents.contains("</svg>"),
            "Output is missing SVG closing tag"
        );

        // Check for visualization elements
        assert!(
            contents.contains("<rect"),
            "SVG doesn't contain any rectangle elements"
        );
        assert!(
            contents.contains("<text"),
            "SVG doesn't contain any text elements"
        );
    }

    // Clean up temporary files
    clean_temp_files(&temp_dir)?;

    Ok(())
}

#[test]
fn test_tree_command_handles_all_files() -> Result<()> {
    // Create a temporary directory for test outputs
    let temp_dir = create_temp_dir()?;

    // Get all test files
    let test_files = get_all_test_files();

    // Test that each file can be processed without errors
    for test_file in test_files {
        let file_name = test_file.file_name().unwrap().to_string_lossy().to_string();
        let mut output_path = temp_dir.clone();
        output_path.push(format!("{}.svg", file_name));

        // Run the tree command
        let result = tree_execute(test_file.clone(), Some(output_path.clone()));

        // All test files should generate an SVG successfully
        assert!(
            result.is_ok(),
            "Failed to generate SVG for test file: {:?}",
            test_file
        );

        // Basic size check (SVG should not be empty)
        let metadata = fs::metadata(&output_path)?;
        assert!(
            metadata.len() > 0,
            "Generated SVG is too small: {} bytes",
            metadata.len()
        );
    }

    // Clean up temporary files
    clean_temp_files(&temp_dir)?;

    Ok(())
}
