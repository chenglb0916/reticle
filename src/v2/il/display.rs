use crate::util::pretty_print::PrettyPrint;
use crate::v2::il::ast::*;
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

display!(Sig);
display!(Def);
display!(Prog);
