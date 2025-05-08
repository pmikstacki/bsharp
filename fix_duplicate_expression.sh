#!/bin/bash
# Script to fix the duplicate Expression imports

# Remove duplicate Expression imports
sed -i '' '11d' src/parsers/expressions/expression_parser.rs
sed -i '' '9d' src/parsers/declarations/field_declaration_parser.rs 
sed -i '' '12d' src/parsers/declarations/property_declaration_parser.rs

echo "Fixed duplicate Expression imports"
