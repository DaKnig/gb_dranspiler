#![allow(dead_code)]
#![allow(unused_imports)]

// use iced_x86::code_asm::{
//     byte_ptr, get_gpr16, get_gpr8, word_ptr, AsmRegister64,
//     CodeAssembler,
// };
// use iced_x86::IcedError;

use crate::sm83;
//use crate::sm83::*;

use super::mapping::{g16, g64, g8};

// type Amd64 = iced_x86::Register;

// enum TranspileResult {
//     Success,
//     OutputError(IcedError),
//     InvalidInstr,
//     Required,
// }

pub struct Amd64Instr {
    instr: String,
    dest_label: Option<String>,
}

impl Amd64Instr {
    fn new(instr: String) -> Self {
        Self {
            instr,
            dest_label: None,
        }
    }
    fn with_label(instr: String, dest_label: String) -> Self {
        let dest_label = Some(dest_label);
        Self { instr, dest_label }
    }
}

pub enum TranspileInstrRes {
    Ok,
    Branch {
        // condition for jump
        cond: sm83::Condition,
        // dest in sm83 space
        dest: u16,
        // the index of the output instruction to patch later
        to_patch: usize,
    },
    // same but no condition
    Jump {
        dest: u16,
        to_patch: usize,
    },
    Lockup {
        // that's for stop, halt, invalid instructions
        pc: u16,
    },
}

pub fn transpile_instr_preserve_c_flag(
    instr: sm83::Instruction,
    mem_reg: &str,
    pc: u16,
) -> (Vec<Amd64Instr>, TranspileInstrRes) {
    use sm83::Instruction::*;
    use sm83::{Reg::*, RegPair::*};

    fn single(s: String) -> Vec<Amd64Instr> {
        vec![Amd64Instr::new(s)]
    }

    let res: TranspileInstrRes = match instr {
        JP_c_a16(c, r8) => TranspileInstrRes::Branch {
            cond: c,
            dest: pc.wrapping_add_signed(r8 as i16),
            to_patch: todo!(),
        },
        Invalid | HALT | STOP(_) => TranspileInstrRes::Lockup {
	    pc
	},
        _ => TranspileInstrRes::Ok,
    };

    let instrs = match instr {
        DI | EI | NOP => vec![], // basically nops
        LD_pa16_SP(addr) => {
            single(format!(
                "mov  WORD PTR [{mem_reg} + {:#?}], {:#?}",
                addr,
                g16(SP)
            ))
            // asm.mov(word_ptr(mem_reg + addr as u32), g16(SP))?
        }
        Invalid | HALT | STOP(_) => vec![], // but the result is a Lockup

        LD_rr_d16(rr, d16) => {
            single(format!("mov {:#?}, {d16}", g16(rr)))
            // asm.mov(g16(rr), d16 as u32)?;
        }
        ADD_HL_rr(rr) => {
            single(format!("add {:#?}, {:#?}", g16(HL), g16(rr)))
            //asm.add(g16(HL), g16(rr))?,
        }
        LD_prr_A(rr) => {
            single(format!(
                "mov BYTE PTR [{:#?}], {:#?}",
                g16(rr),
                g8(A)
            ))
            // asm.mov(ptr(rr), g8(A))?,
        }
        LD_A_prr(rr) => {
            single(format!(
                "mov {:#?}, BYTE PTR [{:#?}]",
                g8(A),
                g64(rr)
            ))
            // asm.mov(g8(A), ptr(rr))?,
        }
        LD_pHLi_A => {
            let mut ret = single(format!(
                "mov BYTE PTR [{:#?}], {:#?}",
                g64(HL),
                g8(A)
            ));

            // asm.mov(ptr(HL), g8(A))?;
            // how do I increase HL without changing the flags?
            if g16(HL) == get_gpr16(Amd64::SI).unwrap()
                && g8(A) == get_gpr8(Amd64::AL).unwrap()
            {
                ret.extend(vec![
                    Amd64Instr::new("cld".into()),
                    Amd64Instr::new("lodsb".into()),
                ]);
                // asm.cld()?;
                // asm.lodsb()?;
            } else {
                todo!()
            }
            ret
        }
        LD_A_pHLi => {
            todo!()
        }
        LD_pHLd_A => {
            todo!()
        }
        LD_A_pHLd => {
            todo!()
        }

        INC_rr(rr) => single(format!("inc {:#?}", g16(rr))),
        DEC_rr(rr) => single(format!("dec {:#?}", g16(rr))),

        INC_r(r) => single(format!("inc {:#?}", g8(r))),
        DEC_r(r) => single(format!("dec {:#?}", g8(r))),
        LD_r_d8(r, d8) => single(format!("mov {:#?} {d8}", g8(r))),

        RLCA => {
            todo!()
        }
        RRCA => {
            todo!()
        }
        RLA => {
            todo!()
        }
        RRA => {
            todo!()
        }
        DAA => {
            single("call daa".into())
            /* branchless DAA implementation by ax6 : (A, F) in (al, dil)
              mov r10b, dil
              test dil, 0x20
              setnz sil
              and r10, 0x10
              add r10b, sil
              lea r10, [r10 + r10 * 2]
              lea rsi, [r10 + r10]
              and edi, 0x60
              mov r10, rsi
              neg r10
              test dil, 0x40
              cmovnz rsi, r10
              cmovz r10, rdi
              and r10, 0xf
              sub rsi, r10
              add al, r10b
              add al, sil
              setz r10b
              setc sil
              lea rsi, [rsi + r10 * 8]
              test dil, 0x40
              setnz r10b
              xor rsi, r10
              shl esi, 4
              or dil, sil
            */
        }
        CPL => {
            todo!()
        }
        SCF => {
            todo!()
        }
        CCF => {
            todo!()
        }
        HALT => {
            todo!()
        }

        LD_r_r(r1, r2) => {
            single(format!("mov {:#?}, {:#?}", g8(r1), g8(r2)))
        }
        // asm.mov(g8(r1), g8(r2))?,
        Alu_A_RegOrNum(op, operand) => {
            use sm83::{AluBlockOp::*, RegOrNum};
            let a = g8(A);
            match (op, operand) {
                (ADD, RegOrNum::Reg(r)) => vec![Amd64Instr::new(
		    format!("add {a:#?}, {:#?}", 
			    g8(r)))],
		// asm.add(a, g8(r))?,
		_ => todo!()
                // (ADC, RegOrNum::Reg(r)) => asm.adc(a, g8(r))?,
                // (SUB, RegOrNum::Reg(r)) => asm.sub(a, g8(r))?,
                // (SBC, RegOrNum::Reg(r)) => asm.sbb(a, g8(r))?,
                // (AND, RegOrNum::Reg(r)) => asm.and(a, g8(r))?,
                // (XOR, RegOrNum::Reg(r)) => asm.xor(a, g8(r))?,
                // (OR, RegOrNum::Reg(r)) => asm.or(a, g8(r))?,
                // (CP, RegOrNum::Reg(r)) => asm.cmp(a, g8(r))?,

                // (ADD, RegOrNum::Num(d8)) => asm.add(a, d8 as u32)?,
                // (ADC, RegOrNum::Num(d8)) => asm.adc(a, d8 as u32)?,
                // (SUB, RegOrNum::Num(d8)) => asm.sub(a, d8 as u32)?,
                // (SBC, RegOrNum::Num(d8)) => asm.sbb(a, d8 as u32)?,
                // (AND, RegOrNum::Num(d8)) => asm.and(a, d8 as u32)?,
                // (XOR, RegOrNum::Num(d8)) => asm.xor(a, d8 as u32)?,
                // (OR, RegOrNum::Num(d8)) => asm.or(a, d8 as u32)?,
                // (CP, RegOrNum::Num(d8)) => asm.cmp(a, d8 as u32)?,
            }
        }
        RET_c(c) => {
            use sm83::Condition::*;
            // let mut _after_ret = asm.create_label();
            todo!(); // how tf do we return?!
                     // asm.ret()?;
                     // asm.set_label(&mut after_ret)?
        }
        LDH_pa8_A(a8) => {
            vec![Amd64Instr::new(format!(
                "mov BYTE PTR [{mem_reg} + {}], {:#?}",
                a8 as u16 + 0xff00,
                g8(A)
            ))]
            // asm.mov(byte_ptr(mem_reg + 0xff00 + a8 as u32), g8(A))?
        }
        LDH_A_pa8(a8) => {
            single(format!(
                "mov {:#?}, BYTE PTR [{mem_reg} + {}]",
                g8(A),
                a8 as u16 + 0xff00,
            ))
            // asm.mov(g8(A), byte_ptr(mem_reg + 0xff00 + a8 as u32))?
        }
        ADD_SP_r8(rel8) => {
            single(format!("add {:#?}, {rel8}", g16(SP)))
        }
        LD_HL_SP_r8(rel8) => {
            single(format!(
                "lea {:#?}, {:#?} + {rel8}",
                g64(HL),
                g64(SP)
            ))
            // asm.lea(g64(HL), word_ptr(g64(SP) + rel8))?
        }
        POP_rr(rr) => {
            // if rr is mapped... just do the obvious thing.
            // since both x86 and sm83 are little endian...
            let instrs = [
                format!(
                    "mov {:#?}, WORD PTR [{:#?}]",
                    g16(rr),
                    g64(SP)
                ),
                format!("inc {:#?}", g16(SP)),
                format!("inc {:#?}", g16(SP)),
            ]
            .map(Amd64Instr::new)
            .into();
            // asm.mov(g16(rr), word_ptr(g64(SP)))?;
            // asm.inc(g16(SP))?;
            // asm.inc(g16(SP))?; // to not hurt the carry by accident

            if rr == AF {
                // extract flags
                todo!()
            }
            instrs
        }
        PUSH_rr(rr) => {
            let mut instrs = vec![];
            if rr == AF {
                // encode flags
                todo!()
            }

            // if rr is mapped... just do the obvious thing.
            // since both x86 and sm83 are little endian...
            instrs.extend(
                [
                    format!("dec {:#?}", g16(SP)), // need to wrap at 1<<16
                    format!("dec {:#?}", g16(SP)), // so cant lea
                    format!(
                        "mov WORD PTR [{:#?}], {:#?}",
                        g64(SP),
                        g16(rr)
                    ),
                ]
                .map(Amd64Instr::new)
                .into_iter(),
            );
            instrs
            // asm.dec(g16(SP))?;
            // asm.dec(g16(SP))?; // to not hurt the carry by accident
            // asm.mov(word_ptr(g64(SP)), g16(rr))?;
        }
        RET | RETI => {
            // since we are not emulating interrupts...
            todo!() // how do we return ffs
        }
        JP_HL => {
            // how do we do jumps?
            todo!()
        }
        LD_SP_HL => {
            single(format!("mov {:#?}, {:#?}", SP, HL))
            //asm.mov(g16(SP), g16(HL))?,
        }
        LDH_pC_A => single(format!(
            "mov BYTE PTR [{mem_reg}+{:#?} + $ff00], {:#?}",
            g8(C),
            g8(A)
        )),
        LDH_A_pC => single(format!(
            "mov {:#?}, BYTE PTR [{mem_reg}+{:#?} + $ff00]",
            g8(A),
            g8(C)
        )),
        LD_pa16_A(a16) => {
            single(format!(
                "lea BYTE PTR [{mem_reg} + {a16}], {:#?}",
                g8(A)
            ))
            // asm.mov(byte_ptr(mem_reg + a16 as u32), g8(A))?;
        }
        LD_A_pa16(a16) => {
            single(format!(
                "mov {:#?}, BYTE PTR [{mem_reg} + {a16}]",
                g8(A),
            ))
            // asm.mov(g8(A), byte_ptr(mem_reg + a16 as u32))?;
        }
        Prefix(op, r1) => {
            use sm83::instructions::PrefixOp::*;
            fn bit_op(op: &str, r1: crate::Reg) -> Vec<Amd64Instr> {
                single(format!("{op} {:#?}, 1", r1))
            }
            match op {
                RLC => bit_op("rol", r1),
                RRC => bit_op("ror", r1),
                RL => bit_op("rcl", r1),
                RR => bit_op("rcr", r1),

                SLA => bit_op("sal", r1),
                SRA => bit_op("sar", r1),
                SWAP => [format!("rol {:#?}, 4", g8(r1)), "clc".into()]
                    .map(Amd64Instr::new)
                    .into(),
                SRL => bit_op("shr", r1),

                BIT(_u3) => {
                    // this requires actually preserving carry
                    // something with bt?
                    todo!()
                }
                RES(_u3) => {
                    // how do I set a bit without changing the carry?
                    todo!()
                }
                SET(_u3) => {
                    todo!()
                }
            }
        }
        Invalid => {
            todo!()
        }
        JR_r8(r8) => {
            return transpile_instr_preserve_c_flag(
                JP_a16(pc.wrapping_add_signed(r8 as i16)),
                mem_reg,
                pc,
            )
        }
        JR_c_r8(c, r8) => {
            return transpile_instr_preserve_c_flag(
                JP_c_a16(c, pc.wrapping_add_signed(r8 as i16)),
                mem_reg,
                pc,
            )
        }

        JP_a16(a16) => {
            // how tf do we jump?!
            single(format!("jmp .sm83_{a16:02x}"))
        }
        JP_c_a16(c, a16) => {
            let op: &str = transpile_cond_jump(c);
            single(format!("{op} .sm83_{a16:02x}"))
        }
        CALL_c_a16(c, a16) => {
            // skip call if condition does not hold
            let op: &str = transpile_cond_jump(c.not());
            let cond_jump =
                single(format!("{op} .after_call_{a16:02x}"));
            let (call_instrs, call_res) =
                transpile_instr_preserve_c_flag(
                    CALL_a16(a16),
                    mem_reg,
                    pc,
                );

            cond_jump
                .into_iter()
                .chain(call_instrs)
                .chain(single(format!(".after_call_{a16:02x}"))) // label
                .collect()
        }
        CALL_a16(a16) => {
            [
                // the call part
                format!("dec {:#?}", g16(SP)), // need to wrap at 1<<16
                format!("dec {:#?}", g16(SP)), // so cant lea
                // push the address of next instruction, 3 bytes from here
                // the jump part
                format!("mov WORD PTR [{:#?}], {:#?}", g64(SP), pc + 3),
                format!("jmp .sm83_{a16:02x}"),
            ]
            .map(Amd64Instr::new)
            .into()
        }
        RST_vector(vec) => {
            return transpile_instr_preserve_c_flag(
                CALL_a16(vec as u16),
                mem_reg,
                pc - 2, // fix addr calc
            );
        }
    };
    (instrs, res)
}

fn transpile_cond_jump(c: crate::sm83::Condition) -> &'static str {
    use crate::sm83::Condition::*;
    match c {
        NZ => {
            "je";
            todo!() // where is the zero flag?!
        }
        Z => {
            "jne";
            todo!()
        }
        NC => "jc",
        C => "jnc",
    }
}
