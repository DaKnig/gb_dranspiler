pub mod decode;
pub mod instructions;
pub mod regs;
//pub use decode::Instruction;

pub use decode::*;
pub use instructions::{
    AluBlockOp, Condition, Instruction, PrefixOp, RegOrNum,
};
pub use regs::{Reg, RegPair};

//pub use Instruction::*;
//pub type Instr = Instruction;
