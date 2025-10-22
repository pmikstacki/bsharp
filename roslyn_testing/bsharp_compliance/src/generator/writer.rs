// Writer: idempotent writes and module tree updates.

use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
pub fn write_group_file(
    dst_dir: &Path,
    module_name: &str,
    _source_stem: &str,
    contents: &str,
) -> Result<()> {
    // Ensure target directory exists
    fs::create_dir_all(dst_dir).with_context(|| format!("create_dir_all {:?}", dst_dir))?;

    // Write file idempotently
    let file_path = dst_dir.join(format!("{}.rs", module_name));
    write_if_changed(&file_path, contents)?;

    // Update mod.rs
    let mod_path = dst_dir.join("mod.rs");
    update_mod_file(&mod_path, module_name)?;

    Ok(())
}

fn write_if_changed(path: &Path, new_contents: &str) -> Result<()> {
    let needs_write = match fs::read_to_string(path) {
        Ok(existing) => existing != new_contents,
        Err(_) => true,
    };
    if !needs_write {
        return Ok(());
    }
    let tmp_path = tmp_path_for(path);
    {
        let mut f = fs::File::create(&tmp_path)
            .with_context(|| format!("create tmp file {:?}", tmp_path))?;
        f.write_all(new_contents.as_bytes())
            .with_context(|| format!("write tmp file {:?}", tmp_path))?;
        f.flush().ok();
    }
    fs::rename(&tmp_path, path).with_context(|| format!("rename {:?} -> {:?}", tmp_path, path))?;
    Ok(())
}

fn tmp_path_for(path: &Path) -> PathBuf {
    let mut p = path.as_os_str().to_owned();
    let mut s = p.to_string_lossy().to_string();
    s.push_str(".tmp");
    PathBuf::from(s)
}

fn update_mod_file(mod_path: &Path, module_name: &str) -> Result<()> {
    let mut lines: Vec<String> = match fs::read_to_string(mod_path) {
        Ok(s) => s
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect(),
        Err(_) => Vec::new(),
    };
    let decl = format!("mod {};", module_name);
    lines.push(decl);
    // Dedup and sort
    lines.sort();
    lines.dedup();
    let mut buf = String::new();
    for l in &lines {
        // Only keep valid module declarations (best effort)
        if l.starts_with("mod ") && l.ends_with(';') {
            buf.push_str(l);
            buf.push('\n');
        }
    }
    write_if_changed(mod_path, &buf)
}

#[allow(dead_code)]
pub fn ensure_parent_mod_has_submod(parent_dir: &Path, submod: &str) -> Result<()> {
    let mod_path = parent_dir.join("mod.rs");
    let mut lines: Vec<String> = match fs::read_to_string(&mod_path) {
        Ok(s) => s
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect(),
        Err(_) => Vec::new(),
    };
    let decl = format!("mod {};", submod);
    lines.push(decl);
    lines.sort();
    lines.dedup();
    let mut buf = String::new();
    for l in &lines {
        if l.starts_with("mod ") && l.ends_with(';') {
            buf.push_str(l);
            buf.push('\n');
        }
    }
    write_if_changed(&mod_path, &buf)
}
