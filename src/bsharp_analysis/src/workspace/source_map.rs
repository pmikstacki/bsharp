use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct FileId(pub u32);

#[derive(Debug, Default, Clone)]
pub struct SourceMap {
    paths: Vec<PathBuf>,
    index: HashMap<PathBuf, FileId>,
    cache: HashMap<FileId, Arc<String>>, // lazy-read cache
}

impl SourceMap {
    pub fn from_paths<I>(paths: I) -> Self
    where
        I: IntoIterator<Item = PathBuf>,
    {
        let mut unique: Vec<PathBuf> = paths.into_iter().collect();
        unique.sort();
        unique.dedup();

        let mut index = HashMap::new();
        for (i, p) in unique.iter().enumerate() {
            index.insert(p.clone(), FileId(i as u32));
        }

        Self {
            paths: unique,
            index,
            cache: HashMap::new(),
        }
    }

    pub fn id_for(&self, path: &Path) -> Option<FileId> {
        self.index.get(path).copied()
    }

    pub fn path_for(&self, id: FileId) -> Option<&Path> {
        self.paths.get(id.0 as usize).map(|p| p.as_path())
    }

    pub fn read(&mut self, id: FileId) -> std::io::Result<Arc<String>> {
        if let Some(s) = self.cache.get(&id) {
            return Ok(s.clone());
        }
        let path = self
            .path_for(id)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "invalid FileId"))?;
        let content = fs::read_to_string(path)?;
        let arc = Arc::new(content);
        self.cache.insert(id, arc.clone());
        Ok(arc)
    }

    pub fn all_files(&self) -> impl Iterator<Item = (FileId, &Path)> {
        self.paths
            .iter()
            .enumerate()
            .map(|(i, p)| (FileId(i as u32), p.as_path()))
    }
}
