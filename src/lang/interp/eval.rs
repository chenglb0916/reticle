use crate::lang::ast::*;
use crate::lang::interp::state::{State, Value};

pub trait Eval {
    fn eval_current(&self, state: &State) -> Value;
}

impl Eval for Instr {
    fn eval_current(&self, state: &State) -> Value {
        match self {
            Instr::Std {
                id: _,
                ty: _,
                op: StdOp::Identity,
                attrs: _,
                params,
            } => state.get_value(&params[0].id()),
            _ => unimplemented!("Prim instr not supported yet"),
        }
    }
}
