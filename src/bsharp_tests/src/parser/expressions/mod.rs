mod anonymous_method_expression_tests; // Added
mod anonymous_object_creation_expression_tests;
mod array_index_expression_tests;
mod assignment_expression_tests;
mod await_expression_tests;
mod binary_expression_tests;
mod binary_operator_tests;
mod cast_expression_tests; // Added
mod checked_expression_tests;
mod conditional_expression_tests;
mod deconstruction_expression_tests;
mod default_expression_tests;
mod expression_parser_tests;
mod expression_tests;
mod interpolated_string_tests; // Renamed from interpolated_string_expression_tests
mod invocation_expression_tests;
mod lambda_expression_tests;
mod literal_tests; // Renamed from literal_expression_tests
mod member_access_expression_tests;
mod nameof_expression_tests;
mod new_expression_tests;
mod null_conditional_expression_tests;
mod null_forgiving_expression_tests;
mod parenthesized_expression_tests; // Added
mod pattern_matching_tests; // Renamed from pattern_expression_tests
mod pattern_tests;
mod postfix_unary_expression_tests; // Added
mod primary_expression_tests;
mod query_expression_tests;
mod range_expression_tests; // New file
mod ref_expression_tests; // New file for ref expressions
mod sizeof_expression_tests;
mod stackalloc_expression_tests;
mod switch_expression_tests; // Added
mod this_expression_tests; // Added
mod throw_expression_tests;
mod tuple_expression_tests; // New file
mod typeof_expression_tests;
mod unary_expression_tests;
mod unary_operator_tests; // Added

// Additional expression tests not previously included
mod ambiguity_tests;
mod collection_expression_tests;
mod identifier_tests;
mod invocation_arguments_tests;
mod literal_suffix_and_raw_multiline_tests;
mod lookahead_boundaries2_tests;
mod target_typed_new_tests;
mod utf8_string_tests;
mod with_expression_tests; // New boundary tests
