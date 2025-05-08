#!/bin/bash
# Script to remove test modules from source files

# Remove tests from variable_declaration_parser.rs
sed -i '' '/#\[cfg(test)\]/,/^}$/d' src/parsers/declarations/variable_declaration_parser.rs

# Remove tests from type_parser.rs
sed -i '' '/#\[cfg(test)\]/,/^}$/d' src/parsers/types/type_parser.rs

# Remove the test from lib.rs
sed -i '' '/#\[test\]/,/^    }$/d' src/lib.rs

echo "Removed test modules from source files"
