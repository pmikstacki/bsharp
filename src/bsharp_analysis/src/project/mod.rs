use bsharp_parser::SpanTable;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct FileId(u32);

#[derive(Default)]
pub struct SourceMap {
    next_id: u32,
    files: HashMap<FileId, (String /*path*/, String /*source*/)>,
}

impl SourceMap {
    pub fn add_file(&mut self, path: impl Into<String>, source: impl Into<String>) -> FileId {
        let id = FileId(self.next_id);
        self.next_id += 1;
        self.files.insert(id, (path.into(), source.into()));
        id
    }

    pub fn get(&self, id: FileId) -> Option<&(String, String)> {
        self.files.get(&id)
    }
}

#[derive(Default)]
pub struct Project {
    pub sources: SourceMap,
    pub spans: HashMap<FileId, SpanTable>,
}

impl Project {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_single_file(
        mut self,
        path: impl Into<String>,
        source: impl Into<String>,
        spans: SpanTable,
    ) -> (Self, FileId) {
        let id = self.sources.add_file(path, source);
        self.spans.insert(id, spans);
        (self, id)
    }
}
