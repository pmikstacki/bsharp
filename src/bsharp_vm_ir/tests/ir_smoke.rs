use bsharp_vm_ir::*;

#[test]
fn build_and_verify_minimal_function() {
    let mut mb = IrModuleBuilder::new();

    let mut fb = mb.new_function("main", FunctionFlags { is_script_entry: true, is_intrinsic: false });
    let entry = fb.new_block();

    // No params/locals for this trivial function; just return null
    let r0 = fb.add_local(Some("tmp".to_string()), ValueKind::Null).1;
    // Emulate a constant load by using a Nop then Return to satisfy verifier's terminator rule
    let _ = fb.append_instr(entry, IrInstr::Nop);
    let _ = fb.append_instr(entry, IrInstr::Return { value: Some(r0) });

    let func = fb.build();
    let fid = func.id;
    mb.push_function(func);
    mb.set_entry(fid);

    let module = mb.build();

    // Verify should pass because each block ends with a terminator
    IrVerifier::verify_module(&module).unwrap();

    // Exercise dump helpers (not asserting on textual form yet)
    let _dump = dump_module(&module);
}
