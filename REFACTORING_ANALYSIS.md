# Refactoring Analysis (Post-Consolidation)

This document outlines issues and recommendations after the initial parser consolidation phase.

**1. AST Node Structure (`src/parser/nodes/declarations/`)**

*   **Attribute Storage**: Field `attributes` in declaration AST nodes.
    *   `DeclarationHeader` (in `type_declaration_parser.rs`) provides `attributes: Vec<AttributeList>`.
    *   The function `convert_attributes` in `type_declaration_parser.rs` flattens `Vec<AttributeList>` to `Vec<Attribute>`.
    *   **Decision**: AST nodes for declarations (Class, Struct, Interface, Enum, Record, EnumMember) will store `attributes: Vec<AttributeList>`. This mirrors C# syntax more closely where attributes can be grouped (e.g., `[Attr1, Attr2][Attr3]`).
    *   **Action**: Ensure all relevant AST node definitions use `Vec<AttributeList>` for their `attributes` field. Parsers should assign the `Vec<AttributeList>` (often from `DeclarationHeader` or `parse_attribute_lists`) directly, without calling `convert_attributes`.

*   **Optional Collections**:
    *   `ClassDeclaration`: `type_parameters: Option<Vec<TypeParameter>>`, `base_types: Option<Vec<Type>>`.
    *   `StructDeclaration`: `type_parameters: Option<Vec<TypeParameter>>`, `base_types: Vec<Type>`.
    *   `InterfaceDeclaration`: `type_parameters: Option<Vec<TypeParameter>>`, `base_types: Vec<Type>`.
    *   `RecordDeclaration`: `type_parameters: Option<Vec<TypeParameter>>` (from header), `base_types: Vec<Type>` (from header).
    *   **Issue**: Inconsistency in `base_types` being `Option<Vec<Type>>` for `ClassDeclaration` vs. `Vec<Type>` for others. An empty `Vec` can represent no base types.
    *   **Recommendation**: For `base_types`, prefer `Vec<Type>` for all type declarations. An empty vector signifies no base types. `ClassDeclaration.base_types` should change from `Option<Vec<Type>>` to `Vec<Type>`. The parser for class declarations needs to be updated to pass `header.base_types` directly.

*   **`InterfaceBodyDeclaration` and its members**:
    *   `InterfaceBodyDeclaration` enum includes `Method`, `Property`, `Event`, `Indexer`.
    *   The parser `parse_interface_member` in `type_declaration_parser.rs` currently only implements `Method`. Properties, events, and indexers are commented out.
    *   **Issue**: Incomplete parsing for interface members.
    *   **Recommendation**: Implement or uncomment parsers for interface properties, events, and indexers. Ensure `parse_interface_property` handles attribute parsing and that properties don't have bodies. `parse_interface_event` and `parse_interface_indexer` need full implementations.

**2. Parser Logic (`src/parsers/declarations/`)**

*   **`type_declaration_parser.rs`**:
    *   `parse_struct_member`: The `nom_to_bs` wrapper for `parse_method_declaration` is missing, unlike in `parse_class_member`. Review if it's needed for consistency or error handling (though `map` should handle `BResult`).
    *   **Attribute Handling (Action)**: Ensure `parse_class_declaration`, `parse_struct_declaration`, `parse_interface_declaration`, and `parse_record_declaration` assign `header.attributes` (which is `Vec<AttributeList>`) directly to their AST nodes. Calls to `convert_attributes` for this purpose should be removed from these parsers.

*   **`enum_declaration_parser.rs` & `record_declaration_parser.rs`**:
    *   `enum_declaration_parser.rs`: Needs to be updated to ensure it assigns `Vec<AttributeList>` to `EnumDeclaration.attributes` and `EnumMember.attributes`.
    *   `record_declaration_parser.rs`: `parse_record_class_declaration` should use `header.attributes`. `parse_record_struct_declaration` parses attributes manually and should ensure it results in `Vec<AttributeList>`.

**3. TODOs and Completeness**

*   `StructBodyDeclaration`: Needs parsers for constructors, properties, etc. (`parse_struct_member` is incomplete).
*   `ClassBodyDeclaration`: Needs parsers for events, indexers, operators, nested types (`parse_class_member` is incomplete).
*   `InterfaceBodyDeclaration`: Needs parsers for properties, events, indexers (`parse_interface_member` is incomplete).
*   Generic constraints parsing needs to be verified and ensured it is used correctly by method/type parsers.
*   Documentation comment parsing (`ClassDeclaration.documentation`, etc.) is missing.

**4. Test Coverage**
*   Ensure test coverage for struct and interface parsing is robust in `type_declaration_parser_tests.rs` or other relevant test files, now that their dedicated parser files are deleted.
*   Consider cleaning up or integrating `tests/parser/declarations/temp_struct_test.rs`. 