use crate::sm83::regs::*;
use iced_x86::{
    code_asm::{
        registers::{
            gpr64::get_gpr64, gpr16::get_gpr16, gpr8::get_gpr8,
        },
        AsmRegister16, AsmRegister64, AsmRegister8,
    },
    Register,
};

type Amd64 = iced_x86::Register;

pub fn g64(rr: RegPair) -> AsmRegister64 {
    get_gpr64(rr.map()).unwrap()
}

pub fn g16(rr: RegPair) -> AsmRegister16 {
    use RegPair::*;
    get_gpr16(rr.map()).unwrap()
}

pub fn g8(r: Reg) -> AsmRegister8 {
    get_gpr8(r.map()).unwrap()
}

trait Mapped {
    fn map(self) -> Amd64;
}

impl Mapped for RegPair {
    fn map(self) -> Amd64 {
        use RegPair::*;
        match self {
            SP => Amd64::DI,
            _ => unreachable!(),
        }
    }
}

impl Mapped for Reg {
    fn map(self) -> Amd64 {
        use iced_x86::Register::*;
        use Reg::*;
        match self {
            A => AL,
            F => AH,
            B => BH,
            C => BL,
            D => CH,
            E => CL,
            H => DH,
            L => DL,
            _ => unreachable!(),
        }
    }
}
