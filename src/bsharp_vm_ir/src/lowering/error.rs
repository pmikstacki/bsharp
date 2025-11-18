#[derive(Debug, Clone)]
pub struct CompileError {
    pub code: &'static str,
    pub message: String,
}

impl CompileError {
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self { code, message: message.into() }
    }
}
