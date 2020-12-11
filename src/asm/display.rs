use crate::asm::ast::*;
use crate::util::pretty_print::PrettyPrint;
use std::fmt;

macro_rules! display {
    ($ty:tt) => {
        impl fmt::Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.to_pretty())
            }
        }
    };
}

display!(ExprCoord);
display!(Loc);
display!(OpAsm);
display!(InstrAsm);
display!(Instr);
display!(Prog);
