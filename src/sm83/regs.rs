#![allow(dead_code)]
#[derive(PartialEq, Eq)]
pub struct RegPair {
    name: &'static str, // sm83 name
    parts: Option<(&'static Reg, &'static Reg)>,
    // bc -> b,c
    host_reg: Option<&'static str>,
}

impl RegPair {
    const fn new(
        name: &'static str,
        parts: Option<(&'static Reg, &'static Reg)>,
        host_reg: Option<&'static str>,
    ) -> Self {
        let host_reg: Option<&str> = if let Some((u, d)) = parts {
            // let [u, d]: [&[u8]; 2] =
            //     [u, d].map(|x| x.name.as_bytes());
            let u: &[u8] = u.name.as_bytes();
            let d: &[u8] = d.name.as_bytes();

            if 2 == u.len()
                && 2 == d.len()
                && u[0] == d[0]
                && u[1] == 'H' as u8
                && d[1] == 'L' as u8
            {
                match u[0] as char {
                    'A' => Some("AX"),
                    'B' => Some("BX"),
                    'C' => Some("CX"),
                    'D' => Some("DX"),
                    _ => None,
                }
            } else {
                None
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

#[derive(PartialEq, Eq)]
enum HalfPair {
    None,
    Top,
    Bottom,
}

use HalfPair::{Bottom, Top};

#[derive(PartialEq, Eq)]
pub struct Reg {
    name: &'static str,
    number: i8,                     // in sm83 inst encoding
    position: HalfPair,             // in a reg pair...
    pair: &'static RegPair, // Option for the special case of (HL)
    host_reg: &'static str,         // host_reg
}

impl Reg {
    const fn new(
        name: &'static str,
        number: i8,
        position: HalfPair,
        pair: &'static RegPair,
        host_reg: &'static str,
    ) -> Self {
        Self {
            name,
            number,
            position,
            pair,
            host_reg,
        }
    }
}

pub static BC: RegPair = RegPair::new("BC", Some((&B, &C)), None);
pub static B: Reg = Reg::new("B", 0, Top, &BC, "BL");
pub static C: Reg = Reg::new("C", 1, Bottom, &BC, "R9B");
pub static DE: RegPair = RegPair::new("DE", Some((&D, &E)), None);
pub static D: Reg = Reg::new("D", 2, Top, &DE, "R8B");
pub static E: Reg = Reg::new("E", 3, Bottom, &DE, "DL");
pub static HL: RegPair = RegPair::new("HL", Some((&H, &L)), None);
pub static H: Reg = Reg::new("H", 4, Top, &HL, "CH");
pub static L: Reg = Reg::new("L", 5, Bottom, &HL, "CL");

// AF, F are special cased everywhere...
pub static AF: RegPair = RegPair::new("HL", Some((&H, &L)), None);
pub static A: Reg = Reg::new("A", 7, Top, &AF, "AL");
pub static F: Reg = Reg::new("F", -1, Bottom, &AF, "AH");

// should never be used
pub static INVALID: RegPair = RegPair::new("Invalid", None, None);
// (HL) is treated almost like a register... would be nice to have it too
pub static HL_: Reg = Reg::new("(HL)", 6, HalfPair::None, &INVALID, "");

// upper bits of the reg containing SP will be the same as MEM
pub static SP: RegPair = RegPair::new("SP", None, Some("DI"));
pub static MEM: &str = "SI";
