use analysis::artifacts::external_il::{
    ExternalAssemblyIndex, IlProviderArtifact, IlTypeIndex, TypeEnvironment,
};
use analysis::context::AnalysisContext;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use bsharp_il::IlProvider;
use parser::facade::Parser;

fn fixture_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn fixture_dir() -> std::path::PathBuf {
    fixture_path("src/fixtures/dll")
}

fn list_fixture_dlls() -> Vec<std::path::PathBuf> {
    let dir = fixture_dir();
    let mut out = Vec::new();
    if let Ok(rd) = std::fs::read_dir(&dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.extension()
                .and_then(|s| s.to_str())
                .map(|s| s.eq_ignore_ascii_case("dll"))
                .unwrap_or(false)
            {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

#[test]
fn loads_all_fixture_dlls_and_indexes_types() {
    let dlls = list_fixture_dlls();
    assert!(
        !dlls.is_empty(),
        "no DLLs found in {}",
        fixture_dir().display()
    );

    let mut success_count = 0usize;
    for dll in dlls.iter() {
        assert!(dll.exists(), "fixture not found at {}", dll.display());

        let src = "class A { void M(){} }";
        let (cu, spans) = Parser::new().parse_with_spans(src).unwrap();
        let mut ctx = AnalysisContext::new("file.cs", src);
        ctx.config.pe_references = vec![dll.display().to_string()];
        let mut session = AnalysisSession::new(ctx, spans);
        AnalyzerPipeline::run_with_defaults(&cu, &mut session);

        let assemblies = session.get_artifact::<ExternalAssemblyIndex>().unwrap();
        if assemblies.assemblies.len() == 1 {
            success_count += 1;

            let types = session.get_artifact::<IlTypeIndex>().unwrap();
            assert!(
                !types.by_fqn.is_empty(),
                "type index empty for {}",
                dll.display()
            );

            let env = session.get_artifact::<TypeEnvironment>().unwrap();
            assert_eq!(
                env.primitive_aliases.get("int"),
                Some(&"System.Int32".to_string())
            );
        }
    }
    assert!(
        success_count > 0,
        "no fixture DLLs could be loaded from {}",
        fixture_dir().display()
    );
}

#[test]
fn provider_lists_methods_without_panic() {
    let dlls = list_fixture_dlls();
    assert!(
        !dlls.is_empty(),
        "no DLLs found in {}",
        fixture_dir().display()
    );

    for dll in dlls {
        let src = "class A { void M(){} }";
        let (cu, spans) = Parser::new().parse_with_spans(src).unwrap();
        let mut ctx = AnalysisContext::new("file.cs", src);
        ctx.config.pe_references = vec![dll.display().to_string()];
        let mut session = AnalysisSession::new(ctx, spans);
        AnalyzerPipeline::run_with_defaults(&cu, &mut session);

        let provider = session.get_artifact::<IlProviderArtifact>().unwrap();
        let types = session.get_artifact::<IlTypeIndex>().unwrap();

        if let Some(th) = types.by_fqn.get("System.String").cloned() {
            let methods = provider.0.list_methods(&th);
            for mh in methods.iter().take(5) {
                let _ = provider.0.method_name(mh);
                let _ = provider.0.method_sig(mh);
            }
        }
    }
}
