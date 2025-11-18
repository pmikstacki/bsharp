use std::fmt::Write as _;

use crate::module::{IrBlock, IrFunction, IrModule};
use crate::{IrInstr};

pub fn dump_module(m: &IrModule) -> String {
    let mut s = String::new();
    let _ = writeln!(&mut s, "Module: entry={:?}", m.entry);
    if !m.globals.is_empty() {
        let _ = writeln!(&mut s, "Globals ({}):", m.globals.len());
        for g in &m.globals {
            let _ = writeln!(&mut s, "  {}: {:?}", g.name, g.kind);
        }
    }
    if !m.constants.is_empty() {
        let _ = writeln!(&mut s, "Constants ({}):", m.constants.len());
    }
    for f in &m.functions {
        let _ = writeln!(&mut s, "");
        s.push_str(&dump_function(f));
    }
    s
}

pub fn dump_function(f: &IrFunction) -> String {
    let mut s = String::new();
    let _ = writeln!(&mut s, "fn {}({}) flags={{entry:{},intrinsic:{}}} regs:{}", f.name, params_sig(f), f.flags.is_script_entry, f.flags.is_intrinsic, f.register_count);
    if !f.locals.is_empty() {
        let _ = writeln!(&mut s, "  locals:");
        for l in &f.locals {
            let _ = writeln!(&mut s, "    %{}/r{}: {:?}", l.id.0, l.register.0, l.kind);
        }
    }
    for b in &f.blocks {
        s.push_str(&dump_block(b));
    }
    s
}

fn params_sig(f: &IrFunction) -> String {
    let mut parts = Vec::new();
    for p in &f.params {
        let name = p.name.as_deref().unwrap_or("_");
        parts.push(format!("{}: {:?}", name, p.kind));
    }
    parts.join(", ")
}

pub fn dump_block(b: &IrBlock) -> String {
    let mut s = String::new();
    let _ = writeln!(&mut s, "  block {}:", b.id.0);
    for i in &b.instructions {
        let _ = writeln!(&mut s, "    {:?}", pretty_instr(i));
    }
    s
}

fn pretty_instr(i: &IrInstr) -> &IrInstr { i }
