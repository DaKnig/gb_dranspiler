mod sm83;
use sm83::regs;

use iced_x86::code_asm::CodeAssembler;

fn transpile_rom_at(
    rom: Box([u8; 3 + 1 << 15]), // to make bounds checking unnecessary
    addr: i16,
) -> CodeAssembler {
    let mut a = CodeAssembler::new(64)?;
}

#[allow(dead_code)]
pub(crate) fn how_to_use_code_assembler() -> Result<(), IcedError> {
    let mut a = CodeAssembler::new(64)?;

    // Anytime you add something to a register (or subtract from it), you create a
    // memory operand. You can also call word_ptr(), dword_bcst() etc to create memory
    // operands.
    let _ = rax; // register
    let _ = rax + 0; // memory with no size hint
    let _ = ptr(rax); // memory with no size hint
    let _ = rax + rcx * 4 - 123; // memory with no size hint
                                 // To create a memory operand with only a displacement or only a base register,
                                 // you can call one of the memory fns:
    let _ = qword_ptr(123); // memory with a qword size hint
    let _ = dword_bcst(rcx); // memory (broadcast) with a dword size hint
                             // To add a segment override, call the segment methods:
    let _ = ptr(rax).fs(); // fs:[rax]

    // Each mnemonic is a method
    a.push(rcx)?;
    // There are a few exceptions where you must append `_<opcount>` to the mnemonic to
    // get the instruction you need:
    a.ret()?;
    a.ret_1(123)?;
    // Use byte_ptr(), word_bcst(), etc to force the arg to a memory operand and to add a
    // size hint
    a.xor(byte_ptr(rdx + r14 * 4 + 123), 0x10)?;
    // Prefixes are also methods
    a.rep().stosd()?;
    // Sometimes, you must add an integer suffix to help the compiler:
    a.mov(rax, 0x1234_5678_9ABC_DEF0u64)?;

    // Create labels that can be referenced by code
    let mut loop_lbl1 = a.create_label();
    let mut after_loop1 = a.create_label();
    a.mov(ecx, 10)?;
    a.set_label(&mut loop_lbl1)?;
    // If needed, a zero-bytes instruction can be used as a label but this is optional
    a.zero_bytes()?;
    a.dec(ecx)?;
    a.jp(after_loop1)?;
    a.jne(loop_lbl1)?;
    a.set_label(&mut after_loop1)?;

    // It's possible to reference labels with RIP-relative addressing
    let mut skip_data = a.create_label();
    let mut data = a.create_label();
    a.jmp(skip_data)?;
    a.set_label(&mut data)?;
    a.db(b"\x90\xCC\xF1\x90")?;
    a.set_label(&mut skip_data)?;
    a.lea(rax, ptr(data))?;

    // AVX512 opmasks, {z}, {sae}, {er} and broadcasting are also supported:
    a.vsqrtps(zmm16.k2().z(), dword_bcst(rcx))?;
    a.vsqrtps(zmm1.k2().z(), zmm23.rd_sae())?;
    // Sometimes, the encoder doesn't know if you want VEX or EVEX encoding.
    // You can force EVEX globally like so:
    a.set_prefer_vex(false);
    a.vucomiss(xmm31, xmm15.sae())?;
    a.vucomiss(xmm31, ptr(rcx))?;
    // or call vex()/evex() to override the encoding option:
    a.evex().vucomiss(xmm31, xmm15.sae())?;
    a.vex().vucomiss(xmm15, xmm14)?;

    // Encode all added instructions.
    // Use `assemble_options()` if you must get the address of a label
    let bytes = a.assemble(0x1234_5678)?;
    assert_eq!(bytes.len(), 82);
    // If you don't want to encode them, you can get all instructions by calling
    // one of these methods:
    let instrs = a.instructions(); // Get a reference to the internal vec
    assert_eq!(instrs.len(), 19);
    let instrs = a.take_instructions(); // Take ownership of the vec with all instructions
    assert_eq!(instrs.len(), 19);
    assert_eq!(a.instructions().len(), 0);

    Ok(())
}
