use analysis::artifacts::external_il::{
    ExternalAssemblyIndex, IlProviderArtifact, IlTypeIndex, TypeEnvironment,
};
use analysis::context::AnalysisContext;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use parser::facade::Parser;

#[test]
fn il_artifacts_present_by_default() {
    let src = "class A { void M(){} }";
    let (cu, spans) = Parser::new().parse_with_spans(src).unwrap();
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);
    assert!(session.get_artifact::<IlProviderArtifact>().is_some());
    assert!(session.get_artifact::<ExternalAssemblyIndex>().is_some());
    assert!(session.get_artifact::<IlTypeIndex>().is_some());
    assert!(session.get_artifact::<TypeEnvironment>().is_some());
}

#[test]
fn il_loader_can_be_disabled_via_config() {
    let src = "class A { void M(){} }";
    let (cu, spans) = Parser::new().parse_with_spans(src).unwrap();
    let mut ctx = AnalysisContext::new("file.cs", src);
    ctx.config
        .enable_passes
        .insert("passes.pe_loader".into(), false);
    let mut session = AnalysisSession::new(ctx, spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);
    assert!(session.get_artifact::<IlProviderArtifact>().is_none());
    assert!(session.get_artifact::<ExternalAssemblyIndex>().is_none());
    assert!(session.get_artifact::<IlTypeIndex>().is_none());
    assert!(session.get_artifact::<TypeEnvironment>().is_none());
}

#[test]
fn il_loader_empty_when_no_refs_or_paths() {
    let src = "class A { void M(){} }";
    let (cu, spans) = Parser::new().parse_with_spans(src).unwrap();
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);
    let assemblies = session.get_artifact::<ExternalAssemblyIndex>().unwrap();
    assert_eq!(assemblies.assemblies.len(), 0);
    let types = session.get_artifact::<IlTypeIndex>().unwrap();
    assert!(types.by_fqn.is_empty());
}

#[test]
fn il_loader_ignores_missing_refs_and_paths() {
    let src = "class A { void M(){} }";
    let (cu, spans) = Parser::new().parse_with_spans(src).unwrap();
    let mut ctx = AnalysisContext::new("file.cs", src);
    ctx.config.pe_references = vec!["/nonexistent/does_not_exist.dll".into()];
    ctx.config.pe_reference_paths = vec!["/nonexistent/dir".into()];
    let mut session = AnalysisSession::new(ctx, spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);
    let assemblies = session.get_artifact::<ExternalAssemblyIndex>().unwrap();
    assert_eq!(assemblies.assemblies.len(), 0);
    let types = session.get_artifact::<IlTypeIndex>().unwrap();
    assert!(types.by_fqn.is_empty());
}

#[test]
fn type_environment_has_primitive_aliases() {
    let src = "class A { void M(){} }";
    let (cu, spans) = Parser::new().parse_with_spans(src).unwrap();
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);
    let env = session.get_artifact::<TypeEnvironment>().unwrap();
    assert_eq!(
        env.primitive_aliases.get("int").cloned(),
        Some("System.Int32".into())
    );
    assert_eq!(
        env.primitive_aliases.get("string").cloned(),
        Some("System.String".into())
    );
    assert_eq!(
        env.primitive_aliases.get("object").cloned(),
        Some("System.Object".into())
    );
}

fn unique_tmp_dir() -> std::path::PathBuf {
    let mut base = std::env::temp_dir();
    let ns = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    base.push(format!("bsharp_il_test_{}_{}", std::process::id(), ns));
    let _ = std::fs::create_dir_all(&base);
    base
}

#[test]
fn il_loader_scans_paths_but_filters_invalid_files() {
    let dir = unique_tmp_dir();
    let dll_path = dir.join("dummy.dll");
    let txt_path = dir.join("note.txt");
    let mut f = std::fs::File::create(&dll_path).unwrap();
    let _ = std::io::Write::write_all(&mut f, b"not a real dll");
    let mut g = std::fs::File::create(&txt_path).unwrap();
    let _ = std::io::Write::write_all(&mut g, b"hello");

    let src = "class A { void M(){} }";
    let (cu, spans) = Parser::new().parse_with_spans(src).unwrap();
    let mut ctx = AnalysisContext::new("file.cs", src);
    ctx.config.pe_reference_paths = vec![dir.display().to_string()];
    let mut session = AnalysisSession::new(ctx, spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);
    let assemblies = session.get_artifact::<ExternalAssemblyIndex>().unwrap();
    assert_eq!(assemblies.assemblies.len(), 0);
    let types = session.get_artifact::<IlTypeIndex>().unwrap();
    assert!(types.by_fqn.is_empty());
}

#[test]
fn il_loader_runs_once_per_session() {
    let src = "class A { void M(){} }";
    let (cu, spans) = Parser::new().parse_with_spans(src).unwrap();
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);
    let a1 = session.get_artifact::<IlProviderArtifact>().unwrap();
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);
    let a2 = session.get_artifact::<IlProviderArtifact>().unwrap();
    assert!(std::sync::Arc::ptr_eq(&a1, &a2));
}
