#![allow(dead_code)]

use crate::{Reg, RegPair, Instruction, AluBlockOp, RegOrNum};

pub fn decode_instr(inst: [u8; 3]) -> Instruction {
    use Instruction::*;
    use RegPair::*;
    let d8: u8 = inst[1]; //
    let d16: u16 = inst[1] as u16 + (inst[2] as u16) << 8; // little endian
    let r8 = inst[1] as i8;

    // static REGS: [&Reg; 8] = [&B, &C, &D, &E, &H, &L, &HL_, &A];

    // we can safely unwrap since for arguments 0..7 it's always Some
    let r1 = Reg::by_num((inst[0] >> 3) & 7).unwrap();
    let r2 = Reg::by_num(inst[0] & 7).unwrap();
    // we can safely unwrap since for arguments 0..3 it's always Some
    let rr = RegPair::by_num_group_hl_sp((inst[0] >> 4) & 3).unwrap();
    let c = match (inst[0] >> 3) & 3 {
        0 => crate::Condition::NZ,
        1 => crate::Condition::Z,
        2 => crate::Condition::NC,
        3 => crate::Condition::C,
        _ => unreachable!(),
    };

    match inst[0] {
        0o000 => NOP,
        0o010 => LD_pa16_SP(d16),
        0o020 => STOP(d8),
        0o030 => JR_r8(r8),
        0o040 | 0o050 | 0o060 | 0o070 => JR_c_r8(c, r8),
        0o001 | 0o021 | 0o041 | 0o061 => LD_rr_d16(rr, d16),
        0o011 | 0o031 | 0o051 | 0o071 => ADD_HL_rr(rr),
        0o002 | 0o022 => LD_prr_A(rr),
        0o012 | 0o032 => LD_A_prr(rr),
        0o042 => LD_pHLi_A,
        0o052 => LD_A_pHLi,
        0o062 => LD_pHLd_A,
        0o072 => LD_A_pHLd,
        0o003 | 0o023 | 0o043 | 0o063 => INC_rr(rr),
        0o013 | 0o033 | 0o053 | 0o073 => DEC_rr(rr),
        0o004 | 0o014 | 0o024 | 0o034 | 0o044 | 0o054 | 0o064
        | 0o074 => INC_r(r1),
        0o005 | 0o015 | 0o025 | 0o035 | 0o045 | 0o055 | 0o065
        | 0o075 => DEC_r(r1),
        0o006 | 0o016 | 0o026 | 0o036 | 0o046 | 0o056 | 0o066
        | 0o076 => LD_r_d8(r1, d8),
        0o007 => RLCA,
        0o017 => RRCA,
        0o027 => RLA,
        0o037 => RRA,
        0o047 => DAA,
        0o057 => CPL,
        0o067 => SCF,
        0o077 => CCF,
        0o166 => HALT,
        0o100..=0o177 => LD_r_r(r1, r2),
        0o200..=0o207 => Alu_A_RegOrNum(AluBlockOp::ADD, RegOrNum::Reg(r2)),
        0o210..=0o217 => Alu_A_RegOrNum(AluBlockOp::ADC, RegOrNum::Reg(r2)),
        0o220..=0o227 => Alu_A_RegOrNum(AluBlockOp::SUB, RegOrNum::Reg(r2)),
        0o230..=0o237 => Alu_A_RegOrNum(AluBlockOp::SBC, RegOrNum::Reg(r2)),
        0o240..=0o247 => Alu_A_RegOrNum(AluBlockOp::AND, RegOrNum::Reg(r2)),
        0o250..=0o257 => Alu_A_RegOrNum(AluBlockOp::XOR, RegOrNum::Reg(r2)),
        0o260..=0o267 => Alu_A_RegOrNum(AluBlockOp::OR, RegOrNum::Reg(r2)),
        0o270..=0o277 => Alu_A_RegOrNum(AluBlockOp::CP, RegOrNum::Reg(r2)),
        0o300 | 0o310 | 0o320 | 0o330 => RET_c(c),
        0o340 => LDH_pa8_A(d8),
        0o350 => ADD_SP_r8(r8),
        0o360 => LDH_A_pa8(d8),
        0o370 => LD_HL_SP_r8(r8),
        0o301 | 0o321 | 0o341 => POP_rr(rr),
        0o361 => POP_rr(AF),
        0o311 => RET,
        0o331 => RETI,
        0o351 => JP_HL,
        0o371 => LD_SP_HL,
        0o302 | 0o312 | 0o322 | 0o332 => JP_c_a16(c, d16),
        0o342 => LDH_pC_A,
        0o352 => LD_pa16_A(d16),
        0o362 => LDH_A_pC,
        0o372 => LD_A_pa16(d16),
        0o303 => JP_a16(d16),
        0xCB => prefix(inst[1]),
        0o323 | 0o333 | 0o343 | 0o353 | 0o344 | 0o354 | 0o364
        | 0o374 | 0o335 | 0o355 | 0o375 => Invalid,
        0o363 => DI,
        0o373 => EI,
        0o304 | 0o314 | 0o324 | 0o334 => CALL_c_a16(c, d16),
        0o305 | 0o325 | 0o345 => POP_rr(rr),
        0o365 => PUSH_rr(AF),
        0o315 => CALL_a16(d16),
        0o306 => Alu_A_RegOrNum(AluBlockOp::ADD, RegOrNum::Num(d8)),
        0o316 => Alu_A_RegOrNum(AluBlockOp::ADC, RegOrNum::Num(d8)),
        0o326 => Alu_A_RegOrNum(AluBlockOp::SUB, RegOrNum::Num(d8)),
        0o336 => Alu_A_RegOrNum(AluBlockOp::SBC, RegOrNum::Num(d8)),
        0o346 => Alu_A_RegOrNum(AluBlockOp::AND, RegOrNum::Num(d8)),
        0o356 => Alu_A_RegOrNum(AluBlockOp::XOR, RegOrNum::Num(d8)),
        0o366 => Alu_A_RegOrNum(AluBlockOp::OR, RegOrNum::Num(d8)),
        0o376 => Alu_A_RegOrNum(AluBlockOp::CP, RegOrNum::Num(d8)),
        0o307 | 0o317 | 0o327 | 0o337 | 0o347 | 0o357 | 0o367
        | 0o377 => RST_vector(inst[0] & 0o070),
    }
}

fn prefix(inst: u8) -> Instruction {
    use crate::PrefixOp::*;

    let r = Reg::by_num(inst & 7).unwrap();
    let bit = (inst >> 3) & 7;

    let op = match inst {
        0o00..=0o07 => RLC,
        0o10..=0o17 => RRC,
        0o20..=0o27 => RL,
        0o30..=0o37 => RR,
        0o40..=0o47 => SLA,
        0o50..=0o57 => SRA,
        0o60..=0o67 => SWAP,
        0o70..=0o77 => SRL,
        0o100..=0o177 => BIT(bit),
        0o200..=0o277 => RES(bit),
        0o300..=0o377 => SET(bit),
    };

    Instruction::Prefix(op, r)
}
