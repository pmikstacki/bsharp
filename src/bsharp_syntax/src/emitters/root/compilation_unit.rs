use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::ast::CompilationUnit;

impl Emit for CompilationUnit {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        use crate::emitters::emit_trait::Emit as _;
        let _scope = cx.node_scope("CompilationUnit");

        let mut wrote_anything = false;

        // Global attributes (one per line, no trailing newline here)
        for (i, ga) in self.global_attributes.iter().enumerate() {
            if i != 0 { cx.nl(w)?; }
            cx.write_indent(w)?;
            ga.emit(w, cx)?;
            wrote_anything = true;
        }

        // Global using directives (separated by newlines)
        for (i, gu) in self.global_using_directives.iter().enumerate() {
            if i != 0 || !self.global_attributes.is_empty() { cx.nl(w)?; }
            cx.write_indent(w)?;
            gu.emit(w, cx)?;
            wrote_anything = true;
        }

        // Regular using directives (separated by newlines)
        for (i, u) in self.using_directives.iter().enumerate() {
            if i != 0 || !self.global_attributes.is_empty() || !self.global_using_directives.is_empty() { cx.nl(w)?; }
            cx.write_indent(w)?;
            u.emit(w, cx)?;
            wrote_anything = true;
        }

        // If header section present and there will be body, insert a blank line
        let has_body = self.file_scoped_namespace.is_some()
            || !self.declarations.is_empty()
            || !self.top_level_statements.is_empty();
        if wrote_anything && has_body {
            cx.between_header_and_body_of_file(w)?;
        }

        // File-scoped namespace (C# 10+)
        if let Some(ns) = &self.file_scoped_namespace {
            cx.write_indent(w)?;
            ns.emit(w, cx)?; // emits: namespace X.Y;
            cx.after_file_scoped_namespace_header(w)?;

            // Usings inside file-scoped namespace
            let mut any_ns_uses = false;
            for (i, u) in ns.using_directives.iter().enumerate() {
                if i != 0 { cx.nl(w)?; }
                cx.write_indent(w)?;
                u.emit(w, cx)?;
                any_ns_uses = true;
            }
            if any_ns_uses && !ns.declarations.is_empty() {
                cx.between_using_blocks_and_declarations(w)?;
            }

            // Declarations inside file-scoped namespace
            let mut first = true;
            for d in &ns.declarations {
                if !first { cx.between_top_level_declarations(w)?; }
                cx.write_indent(w)?;
                d.emit(w, cx)?;
                first = false;
            }
            wrote_anything = true;
        }

        // Top-level declarations (skip GlobalAttribute variants to avoid duplication)
        let mut first_decl = true;
        for d in &self.declarations {
            // Skip GlobalAttribute if present among declarations
            if let crate::ast::TopLevelDeclaration::GlobalAttribute(_) = d { continue; }

            if !first_decl {
                // blank line between declarations
                cx.nl(w)?;
            }
            cx.write_indent(w)?;
            d.emit(w, cx)?;
            first_decl = false;
            wrote_anything = true;
        }

        // Top-level statements
        if !self.top_level_statements.is_empty() {
            if wrote_anything && first_decl { // had header but no declarations
                cx.nl(w)?;
            }
            if wrote_anything && !first_decl { // had declarations
                cx.nl(w)?;
            }
            for (i, s) in self.top_level_statements.iter().enumerate() {
                if i != 0 { cx.nl(w)?; }
                cx.write_indent(w)?;
                s.emit(w, cx)?;
                wrote_anything = true;
            }
        }

        // Ensure exactly one final newline at EOF if any content was written
        if wrote_anything { cx.nl(w)?; }

        Ok(())
    }
}
