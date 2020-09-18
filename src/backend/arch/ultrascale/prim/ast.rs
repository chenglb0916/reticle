use std::collections::HashMap;

pub type Id = String;
pub type PortMap = HashMap<String, Expr>;
pub type ParamMap = HashMap<String, i64>;
pub type AttrMap = HashMap<String, String>;

#[derive(Clone, Debug)]
pub enum Expr {
    Ref(String),
    Index(String, u32),
}

#[derive(Clone, Debug)]
pub struct Slice {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Debug)]
pub enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Clone, Debug)]
pub enum BelTy {
    A6LUT,
    FF,
}

#[derive(Clone, Debug)]
pub struct Bel {
    pub letter: Letter,
    pub ty: BelTy,
}

#[derive(Clone, Debug)]
pub struct Loc {
    pub slice: Slice,
    pub bel: Bel,
}

#[derive(Clone, Debug)]
pub enum LutTy {
    Lut1,
    Lut2,
    Lut3,
    Lut4,
    Lut5,
    Lut6,
}

#[derive(Clone, Debug)]
pub struct Lut {
    pub ty: LutTy,
    pub id: Id,
    pub attrs: AttrMap,
    pub inputs: PortMap,
    pub outputs: PortMap,
    pub loc: Option<Loc>,
}

#[derive(Clone, Debug)]
pub enum RegTy {
    Fdre,
    Fdse,
}

#[derive(Clone, Debug)]
pub struct Reg {
    pub ty: RegTy,
    pub id: Id,
    pub inputs: PortMap,
    pub outputs: PortMap,
    pub loc: Option<Loc>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DspFusedOp {
    Mul,
    RegMul,
    MulReg,
    RegMulReg,
    MulAdd,
    RegMulAdd,
    MulRegAdd,
    MulRegAddReg,
    RegMulAddReg,
    RegMulRegAdd,
    RegMulRegAddReg,
    MulAddReg,
}

#[derive(Clone, Debug)]
pub struct DspFused {
    pub op: DspFusedOp,
    pub id: Id,
    pub params: ParamMap,
    pub inputs: PortMap,
    pub outputs: PortMap,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DspVectorOp {
    Add,
    Sub,
    And,
    Or,
    Xor,
    Nand,
    Nor,
    Xnor,
}

#[derive(Clone, Debug)]
pub struct DspVector {
    pub op: DspVectorOp,
    pub id: Id,
    pub params: ParamMap,
    pub inputs: PortMap,
    pub outputs: PortMap,
}

#[derive(Clone, Debug)]
pub struct Vcc {
    pub id: Id,
    pub outputs: PortMap,
}

#[derive(Clone, Debug)]
pub struct Gnd {
    pub id: Id,
    pub outputs: PortMap,
}

#[derive(Clone, Debug)]
pub struct Const {
    pub id: Id,
    pub params: ParamMap,
    pub inputs: PortMap,
}

#[derive(Clone, Debug)]
pub struct Carry {
    pub id: Id,
    pub inputs: PortMap,
    pub outputs: PortMap,
}
