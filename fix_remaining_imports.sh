#!/bin/bash
# Script to fix the remaining import issues

# Fix field_declaration_parser.rs
sed -i '' '1s/^/use crate::parsers::identifier_parser::parse_identifier;\n/' src/parsers/declarations/field_declaration_parser.rs

# Fix property_declaration_parser.rs imports
sed -i '' '1s/^/use crate::parser::parser_helpers::{bws, nom_to_bs};\n/' src/parsers/declarations/property_declaration_parser.rs
sed -i '' 's/tag(/tag_no_case(/g' src/parsers/declarations/property_declaration_parser.rs
sed -i '' '2s/^/use nom::bytes::complete::tag_no_case;\n/' src/parsers/declarations/property_declaration_parser.rs

echo "Fixed remaining import issues"
