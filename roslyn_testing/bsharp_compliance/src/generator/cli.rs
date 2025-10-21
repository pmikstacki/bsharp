use anyhow::Result;

// Legacy bridge: keep existing runner while we migrate.
pub fn dispatch_legacy(cfg: crate::tests_writer::utility::Config) -> Result<()> {
    crate::tests_writer::runner::run(cfg)
}
