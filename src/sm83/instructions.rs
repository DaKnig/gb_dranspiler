#![allow(dead_code)]

use crate::sm83::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Condition {
    NZ = 0,
    Z = 1,
    NC = 2,
    C = 3,
}

impl Condition {
    pub fn not(self) -> Self {
	use Condition::*;
        match self {
            NZ => Z,
            Z => NZ,
            NC => C,
            C => NC,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PrefixOp {
    RLC,
    RRC,
    RL,
    RR,
    SLA,
    SRA,
    SWAP,
    SRL,
    BIT(u8),
    RES(u8),
    SET(u8),
}

#[derive(Clone, Copy, Debug)]
pub enum AluBlockOp {
    // for all the 02xx instructions and 036x
    ADD = 0o00,
    ADC = 0o10,
    SUB = 0o20,
    SBC = 0o30,
    AND = 0o40,
    XOR = 0o50,
    OR = 0o60,
    CP = 0o70,
}

#[derive(Clone, Copy, Debug)]
pub enum RegOrNum {
    // useful for the 02xx instructions
    Reg(Reg),
    Num(u8),
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    // I use octal because it alligns better for gb instructions
    // mnemonic                         // opcode
    NOP,                                  // 000
    LD_pa16_SP(u16),                      // 010
    STOP(u8),                             // 020
    JR_r8(i8),                            // 030
    JR_c_r8(Condition, i8),               // 040, 050, 060, 070
    LD_rr_d16(RegPair, u16),              // 0e1 e for evens
    ADD_HL_rr(RegPair),                   // 0o1 o for odds
    LD_prr_A(RegPair),                    // 002, 022; for bc, de
    LD_A_prr(RegPair),                    // 012, 032; for bc, de
    LD_pHLi_A,                            // 042
    LD_A_pHLi,                            // 052
    LD_pHLd_A,                            // 062
    LD_A_pHLd,                            // 072
    INC_rr(RegPair),                      // 0e3
    DEC_rr(RegPair),                      // 0o3
    INC_r(Reg),                           // 0?4
    DEC_r(Reg),                           // 0?5
    LD_r_d8(Reg, u8),                     // 0?6
    RLCA,                                 //
    RRCA,                                 //
    RLA,                                  //
    RRA,                                  //
    DAA,                                  //
    CPL,                                  //
    SCF,                                  //
    CCF,                                  // 0?7
    HALT,                                 // 166
    LD_r_r(Reg, Reg),                     // 1??
    Alu_A_RegOrNum(AluBlockOp, RegOrNum), // 20?, 2?6
    RET_c(Condition),                     // 300, 310, 320, 330
    LDH_pa8_A(u8),                        // 340
    ADD_SP_r8(i8),                        // 350
    LDH_A_pa8(u8),                        // 360
    LD_HL_SP_r8(i8),                      // 370
    POP_rr(RegPair),                      // 3e1 e for evens
    RET,                                  // 311
    RETI,                                 // 331
    JP_HL,                                // 351
    LD_SP_HL,                             // 371
    JP_c_a16(Condition, u16),             // 302, 312, 322, 332
    LDH_pC_A,                             // 342
    LD_pa16_A(u16),                       // 352
    LDH_A_pC,                             // 362
    LD_A_pa16(u16),                       // 372
    JP_a16(u16),                          // 303
    Prefix(PrefixOp, Reg),                // 0xcb
    Invalid,                              // all invalid ops
    DI,                                   // 363
    EI,                                   // 373
    CALL_c_a16(Condition, u16),           // 304, 314, 324, 334
    PUSH_rr(RegPair),                     // 3e5 e for evens
    CALL_a16(u16),                        // 315
    RST_vector(u8),                       // 3?7
                                          // u8 is in [0x00, 0x08, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38]
}

impl Instruction {
    pub fn len(&self) -> u16 {
        use Instruction::*;
        match self {
            LD_pa16_SP(_) => 3,
            _ => unimplemented!(),
        }
    }
}
