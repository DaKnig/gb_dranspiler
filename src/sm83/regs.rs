#![allow(dead_code)]

type Amd64 = iced_x86::Register;

use Reg::*;
use RegPair::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RegPair {
    BC = 0,
    DE = 1,
    HL = 2,
    SP = 3,
    AF = 4,
}

impl RegPair {
    pub const fn parts(self) -> Option<(Reg, Reg)> {
        use Reg::*;
        match self {
            SP => None,
            AF => Some((A, F)),
            BC => Some((B, C)),
            DE => Some((D, E)),
            HL => Some((H, L)),
        }
    }
    pub const fn by_num_group_hl_sp(n: u8) -> Option<RegPair> {
        use RegPair::*;
        match n {
            0 => Some(BC),
            1 => Some(DE),
            2 => Some(HL),
            3 => Some(SP),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Reg {
    B = 0,
    C = 1,
    D = 2,
    E = 3,
    H = 4,
    L = 5,
    HL_ = 6, // pseudo-reg
    A = 7,
    F,
}

impl Reg {
    pub const fn pair(self) -> Option<RegPair> {
        match self {
            HL_ => None,
            A | F => Some(AF),
            B | C => Some(BC),
            D | E => Some(DE),
            H | L => Some(HL),
        }
    }
    pub const fn by_num(n: u8) -> Option<Reg> {
        match n {
            0 => Some(B),
            1 => Some(C),
            2 => Some(D),
            3 => Some(E),
            4 => Some(H),
            5 => Some(L),
            6 => Some(HL_),
            7 => Some(A),
            _ => None,
        }
    }
    pub const fn name(self) -> &'static str {
        use Reg::*;
        match self {
            B => "B",
            C => "C",
            D => "D",
            E => "E",
            H => "H",
            L => "L",
            HL_ => "[HL]",
            A => "A",
            F => "F",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum HalfPair {
    None,
    Top,
    Bottom,
}
