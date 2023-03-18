#![allow(dead_code)]

type Amd64 = iced_x86::Register;
use iced_x86::code_asm::*;

#[derive(Debug, PartialEq, Eq)]
pub struct RegPair {
    pub name: &'static str, // sm83 name
    pub parts: Option<(&'static Reg, &'static Reg)>,
    // bc -> b,c
    pub host_reg: iced_x86::Register,
}

impl RegPair {
    pub fn parts(&self) -> Option<(AsmRegister8, AsmRegister8)> {
        if let Some((upper, lower)) = self.parts {
            Some((upper.g8(), lower.g8()))
        } else {
            None
        }
    }
    pub fn is_mapped_to_reg(&self) -> bool {
        self.host_reg != iced_x86::Register::None
    }
    pub fn g16(&self) -> AsmRegister16 {
        get_gpr16(self.host_reg).unwrap()
    }
    pub fn g64(&self) -> AsmRegister64 {
        get_gpr64(self.host_reg).unwrap()
    }
    pub fn ptr(&self) -> AsmMemoryOperand {
        ptr(get_gpr64(MEM).unwrap() + self.g64())
    }
    const fn new(
        name: &'static str,
        parts: Option<(&'static Reg, &'static Reg)>,
        host_reg: iced_x86::Register,
    ) -> Self {
        let host_reg: iced_x86::Register = if let Some((h, l)) = parts {
            // the sm83 reg names of the "parts"
            let h: iced_x86::Register = h.host_reg;
            let l = l.host_reg as iced_x86::Register;

            use iced_x86::Register::*;
            match (h, l) {
                (AH, AL) => AX,
                (BH, BL) => BX,
                (CH, CL) => CX,
                (DH, DL) => DX,
                _ => Amd64::None,
            }
        } else {
            host_reg
        };
        Self {
            name,
            parts,
            host_reg,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum HalfPair {
    None,
    Top,
    Bottom,
}

use HalfPair::{Bottom, Top};

#[derive(Debug, PartialEq, Eq)]
pub struct Reg {
    name: &'static str,
    number: i8,                       // in sm83 inst encoding
    position: HalfPair,               // in a reg pair...
    pair: &'static RegPair, // Option for the special case of (HL)
    pub host_reg: iced_x86::Register, // host_reg
}

impl Reg {
    const fn new(
        name: &'static str,
        number: i8,
        position: HalfPair,
        pair: &'static RegPair,
        host_reg: iced_x86::Register,
    ) -> Self {
        use iced_x86::Register::*;
        match host_reg {
            None => {} // special casing HL_ aka [HL] pseudo register
            AL | AH | BL | BH | CL | CH | DL | DH | SPL | BPL | SIL
            | DIL | R8L | R9L | R10L | R11L | R12L | R13L | R14L
            | R15L => {}
            _ => panic!("use 8bit regs!"),
        } // would be happy to replace with a better alternative!

        Self {
            name,
            number,
            position,
            pair,
            host_reg,
        }
    }

    pub fn g8(&self) -> AsmRegister8 {
        get_gpr8(self.host_reg).unwrap()
    }
}

pub static BC: RegPair =
    RegPair::new("BC", Some((&B, &C)), Amd64::None);
pub static B: Reg = Reg::new("B", 0, Top, &BC, Amd64::BL); //"BL");
pub static C: Reg = Reg::new("C", 1, Bottom, &BC, Amd64::R9L); //"R9B");
pub static DE: RegPair =
    RegPair::new("DE", Some((&D, &E)), Amd64::None);
pub static D: Reg = Reg::new("D", 2, Top, &DE, Amd64::R8L); //"R8B");
pub static E: Reg = Reg::new("E", 3, Bottom, &DE, Amd64::DL); //"DL");
pub static HL: RegPair =
    RegPair::new("HL", Some((&H, &L)), Amd64::None);
pub static H: Reg = Reg::new("H", 4, Top, &HL, Amd64::CH); //"CH");
pub static L: Reg = Reg::new("L", 5, Bottom, &HL, Amd64::CL); //"CL");

// AF, F are special cased everywhere...
pub static AF: RegPair =
    RegPair::new("HL", Some((&H, &L)), Amd64::None);
pub static A: Reg = Reg::new("A", 7, Top, &AF, Amd64::AL); //"AL");
pub static F: Reg = Reg::new("F", -1, Bottom, &AF, Amd64::AH); //"AH");

// should never be used
pub static INVALID: RegPair =
    RegPair::new("Invalid", None, Amd64::None);
// (HL) is treated almost like a register... would be nice to have it too
pub static HL_: Reg =
    Reg::new("(HL)", 6, HalfPair::None, &INVALID, Amd64::None);

// upper bits of the reg containing SP will be the same as MEM
pub static SP: RegPair = RegPair::new("SP", None, Amd64::DI);

pub static MEM: iced_x86::Register = Amd64::RSI;
