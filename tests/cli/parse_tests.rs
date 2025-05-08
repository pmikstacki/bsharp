use std::fs;
use std::io::Read;
use anyhow::Result;
use serde_json::Value;

// Directly import the parse function to avoid module resolution issues
use bsharp::cli::commands::parse::execute as parse_execute;
use super::utils::{get_all_test_files, create_temp_dir, clean_temp_files};

#[test]
fn test_parse_command_creates_valid_json() -> Result<()> {
    // Create a temporary directory for test outputs
    let temp_dir = create_temp_dir()?;
    
    // Get all test files
    let test_files = get_all_test_files();
    assert!(!test_files.is_empty(), "No test files found in test directory");
    
    for test_file in test_files {
        // Define output path for this test file
        let file_name = test_file.file_name().unwrap().to_string_lossy();
        let mut output_path = temp_dir.clone();
        output_path.push(format!("{}.json", file_name));
        
        // Run the parse command
        parse_execute(test_file.clone(), Some(output_path.clone()))?;
        
        // Verify the output file exists
        assert!(output_path.exists(), "Output file was not created: {:?}", output_path);
        
        // Verify the output is valid JSON
        let mut file = fs::File::open(&output_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        let json_result: Value = serde_json::from_str(&contents)?;
        assert!(json_result.is_object(), "JSON output is not a valid object");
        
        // Verify basic structure of the AST JSON
        assert!(json_result.get("usings").is_some(), "JSON missing 'usings' field");
        assert!(json_result.get("members").is_some(), "JSON missing 'members' field");
    }
    
    // Clean up temporary files
    clean_temp_files(&temp_dir)?;
    
    Ok(())
}

#[test]
fn test_parse_command_handles_all_syntax_features() -> Result<()> {
    // Create a temporary directory for test outputs
    let temp_dir = create_temp_dir()?;
    
    // Get all test files
    let test_files = get_all_test_files();
    
    // Test each file individually to check specific syntax features
    for test_file in test_files {
        let file_name = test_file.file_name().unwrap().to_string_lossy().to_string();
        let mut output_path = temp_dir.clone();
        output_path.push(format!("{}.json", file_name));
        
        // Run the parse command
        let result = parse_execute(test_file.clone(), Some(output_path.clone()));
        
        // All test files should parse successfully
        assert!(result.is_ok(), "Failed to parse test file: {:?}", test_file);
        
        // Load the generated JSON
        let json_contents = fs::read_to_string(&output_path)?;
        let json: Value = serde_json::from_str(&json_contents)?;
        
        // Check for specific syntax features based on filename
        if file_name.contains("simple_class") {
            check_simple_class_features(&json)?;
        } else if file_name.contains("control_flow") {
            check_control_flow_features(&json)?;
        } else if file_name.contains("advanced_features") {
            check_advanced_features(&json)?;
        }
    }
    
    // Clean up temporary files
    clean_temp_files(&temp_dir)?;
    
    Ok(())
}

fn check_simple_class_features(json: &Value) -> Result<()> {
    // Check for class declaration
    let members = json.get("members").unwrap().as_array().unwrap();
    assert!(!members.is_empty(), "No members found in JSON output");
    
    // Check for the presence of a class declaration
    let has_class = members.iter().any(|member| {
        member.get("Class").is_some()
    });
    assert!(has_class, "No class declaration found in JSON output");
    
    Ok(())
}

fn check_control_flow_features(json: &Value) -> Result<()> {
    // Check for various control flow statements
    // This is a simplified check - in a real implementation, you would
    // traverse the AST to verify specific control flow nodes
    let json_str = json.to_string();
    
    // Check for control flow related keywords
    assert!(json_str.contains("if") || json_str.contains("If"), "No if statements found");
    assert!(json_str.contains("switch") || json_str.contains("Switch"), "No switch statements found");
    assert!(json_str.contains("for") || json_str.contains("For"), "No for loops found");
    assert!(json_str.contains("while") || json_str.contains("While"), "No while loops found");
    assert!(json_str.contains("foreach") || json_str.contains("Foreach"), "No foreach loops found");
    
    Ok(())
}

fn check_advanced_features(json: &Value) -> Result<()> {
    // Check for advanced C# features
    let json_str = json.to_string();
    
    // Check for interface and generic related syntax
    assert!(json_str.contains("interface") || json_str.contains("Interface"), "No interface declarations found");
    assert!(json_str.contains("generic") || json_str.contains("<"), "No generic types found");
    assert!(json_str.contains("async") || json_str.contains("Async"), "No async features found");
    assert!(json_str.contains("await") || json_str.contains("Await"), "No await expressions found");
    assert!(json_str.contains("try") || json_str.contains("Try"), "No try-catch blocks found");
    
    Ok(())
}
