use crate::generator::model::{ParseContext, TestOptions};

// Parse TestOptions.* flags and Script vs Regular context from a Roslyn file slice.
// Initial stub: returns Regular context and no flags.
pub fn parse_options_from_trailing_args(_args_slice: &str) -> TestOptions {
    TestOptions { ctx: ParseContext::Regular, roslyn_flags: Vec::new() }
}
