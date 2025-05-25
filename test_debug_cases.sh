#!/bin/bash

# Create output directory for JSON files
mkdir -p debug_output

# Build the project first
echo "Building bsharp..."
cargo build --release

# Check if build was successful
if [ $? -ne 0 ]; then
    echo "Build failed, exiting..."
    exit 1
fi

echo "Testing all CS files in debug_cases directory..."
echo "JSON outputs will be saved to debug_output directory"
echo "=================================================="

# Counter for tracking results
total=0
success=0
failed=0

# Process each CS file in debug_cases
for cs_file in debug_cases/*.cs; do
    if [ -f "$cs_file" ]; then
        total=$((total + 1))
        filename=$(basename "$cs_file")
        base_name="${filename%.cs}"
        json_output="debug_output/${base_name}.json"
        
        echo "Testing: $filename"
        echo "Output: $json_output"
        
        # Run bsharp parser on the file
        if ./target/release/bsharp parse "$cs_file" --output "$json_output" 2>&1; then
            echo "✅ SUCCESS: $filename"
            success=$((success + 1))
        else
            echo "❌ FAILED: $filename"
            failed=$((failed + 1))
        fi
        echo "----------------------------------------"
    fi
done

echo "=================================================="
echo "Summary:"
echo "Total files: $total"
echo "Successful: $success"
echo "Failed: $failed"
echo "JSON outputs in: debug_output/"

if [ $failed -eq 0 ]; then
    echo "🎉 All tests passed!"
    exit 0
else
    echo "⚠️  Some tests failed"
    exit 1
fi 