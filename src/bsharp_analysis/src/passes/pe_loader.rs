use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use globwalk::GlobWalkerBuilder;

use crate::artifacts::external_il::{
    AssemblyRecord, ExternalAssemblyIndex, IlProviderArtifact, IlTypeIndex, TypeEnvironment,
};
use crate::framework::AnalysisSession;
use crate::framework::{AnalyzerPass, Phase};
use crate::syntax::ast::CompilationUnit;

use bsharp_il::{DotscopeProvider, IlProvider};

pub struct PeLoaderPass;

impl AnalyzerPass for PeLoaderPass {
    fn id(&self) -> &'static str {
        "passes.pe_loader"
    }
    fn phase(&self) -> Phase {
        Phase::Index
    }

    fn run(&self, _cu: &CompilationUnit, session: &mut AnalysisSession) {
        if session.get_artifact::<ExternalAssemblyIndex>().is_some() {
            return;
        }

        let mut provider = DotscopeProvider::new();
        let mut seen: HashSet<PathBuf> = HashSet::new();
        let mut assemblies: Vec<AssemblyRecord> = Vec::new();
        let mut types = IlTypeIndex::default();

        for s in &session.config.pe_references {
            let p = PathBuf::from(s);
            collect_and_load(&mut provider, &mut seen, &mut assemblies, &mut types, &p);
        }
        for dir in &session.config.pe_reference_paths {
            let dir_path = PathBuf::from(dir);
            if dir_path.is_dir() {
                if let Ok(walker) =
                    GlobWalkerBuilder::from_patterns(&dir_path, &["**/*.dll"]).build()
                {
                    for entry in walker.flatten() {
                        let path = entry.path().to_path_buf();
                        collect_and_load(
                            &mut provider,
                            &mut seen,
                            &mut assemblies,
                            &mut types,
                            &path,
                        );
                    }
                }
            }
        }

        session.insert_artifact(IlProviderArtifact(Arc::new(provider)));
        session.insert_artifact(ExternalAssemblyIndex { assemblies });
        session.insert_artifact(types);
        session.insert_artifact(TypeEnvironment::with_defaults());
    }
}

fn collect_and_load(
    provider: &mut DotscopeProvider,
    seen: &mut HashSet<PathBuf>,
    assemblies: &mut Vec<AssemblyRecord>,
    types: &mut IlTypeIndex,
    path: &Path,
) {
    if let Ok(canon) = path.canonicalize() {
        if !seen.insert(canon.clone()) {
            return;
        }
        if let Ok(handle) = provider.load_assembly(&canon) {
            let list = provider.list_types(handle);
            let count = list.len();
            for t in &list {
                types.by_fqn.entry(t.fullname.clone()).or_insert(t.clone());
            }
            assemblies.push(AssemblyRecord {
                path: canon,
                type_count: count,
            });
        }
    }
}
