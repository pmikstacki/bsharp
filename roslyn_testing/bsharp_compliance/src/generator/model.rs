use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestCategory {
    Tree,
    Statement,
    Declaration,
    Expression,
    Name,
    Type,
    ParameterList,
    AttributeList,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseContext {
    Regular,
    Script,
}

impl Default for ParseContext {
    fn default() -> Self {
        ParseContext::Regular
    }
}

#[derive(Debug, Clone, Default)]
pub struct TestOptions {
    pub ctx: ParseContext,
    pub roslyn_flags: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct RoslynDiagnosticExpectation {
    pub code: Option<String>,
    pub span: Option<(usize, usize)>,
    pub message_args: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ExpectedDiagnostics {
    pub count: usize,
    pub items: Vec<RoslynDiagnosticExpectation>,
}

#[derive(Debug, Clone)]
pub struct ExtractedTest {
    pub category: TestCategory,
    pub method_name: Option<String>,
    pub code: String,
    pub expected: Option<ExpectedDiagnostics>,
    pub options: TestOptions,
}

#[derive(Debug, Clone, Default)]
pub struct GeneratorConfig {
    pub src: PathBuf,
    pub dst: PathBuf,
    pub include: Vec<String>,
    pub exclude: Vec<String>,
    // Legacy bridging options for current tests_writer runner
    pub max_per_file: usize,
    pub skip_overrides: bool,
    pub skip_diagnostics: bool,
    pub skip_options: bool,
    pub verbose: bool,
    pub dry_run: bool,
    pub fail_fast: bool,
}
