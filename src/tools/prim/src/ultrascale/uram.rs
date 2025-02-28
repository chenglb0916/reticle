use crate::{Param, ParamSet, Port, PortSet, Prim, ToPrim};
use derive_more::{Deref, DerefMut, Display, From};

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub enum CascadeOrder {
    #[display(fmt = "FIRST")]
    First,
    #[display(fmt = "MIDDLE")]
    Middle,
    #[display(fmt = "LAST")]
    Last,
    #[display(fmt = "NONE")]
    None,
}

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub enum BwMode {
    #[display(fmt = "PARITY_INTERLEAVED")]
    Interleaved,
    #[display(fmt = "PARITY_INDEPENDENT")]
    Independent,
}

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub enum RstMode {
    #[display(fmt = "SYNC")]
    Sync,
    #[display(fmt = "ASYNC")]
    Async,
}

#[derive(Clone, Debug, From, Eq, Display)]
pub enum UramParam {
    CascadeOrder(CascadeOrder),
    BwMode(BwMode),
    RstMode(RstMode),
    Bool(bool),
    I64(i64),
}

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct Uram(Prim<UramParam>);

#[derive(Clone, Debug, Default)]
struct UramPrim;

impl PartialEq for UramParam {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (UramParam::CascadeOrder(_), UramParam::CascadeOrder(_)) => true,
            (UramParam::BwMode(_), UramParam::BwMode(_)) => true,
            (UramParam::RstMode(_), UramParam::RstMode(_)) => true,
            (UramParam::Bool(_), UramParam::Bool(_)) => true,
            (UramParam::I64(_), UramParam::I64(_)) => true,
            (_, _) => false,
        }
    }
}

impl ToPrim<UramParam> for UramPrim {
    fn to_name(&self) -> String {
        String::from("URAM288")
    }
    fn to_param(&self) -> ParamSet<UramParam> {
        let mut param = ParamSet::new();
        // TODO: range for this param is 3-15
        // but there is no special types for this
        param.insert(Param {
            name: "AUTO_SLEEP_LATENCY".into(),
            width: None,
            value: 8i64.into(),
        });
        // TODO: range for this param is 10-10000
        // but there is no special types for this
        param.insert(Param {
            name: "AVG_CONS_INACTIVE_CYCLES".into(),
            width: None,
            value: 10i64.into(),
        });
        param.insert(Param {
            name: "BWE_MODE_A".into(),
            width: None,
            value: BwMode::Interleaved.into(),
        });
        param.insert(Param {
            name: "BWE_MODE_B".into(),
            width: None,
            value: BwMode::Interleaved.into(),
        });
        param.insert(Param {
            name: "CASCADE_ORDER_A".into(),
            width: None,
            value: CascadeOrder::None.into(),
        });
        param.insert(Param {
            name: "CASCADE_ORDER_B".into(),
            width: None,
            value: CascadeOrder::None.into(),
        });
        param.insert(Param {
            name: "EN_AUTO_SLEEP_MODE".into(),
            width: None,
            value: false.into(),
        });
        param.insert(Param {
            name: "EN_ECC_RD_A".into(),
            width: None,
            value: false.into(),
        });
        param.insert(Param {
            name: "EN_ECC_RD_B".into(),
            width: None,
            value: false.into(),
        });
        param.insert(Param {
            name: "EN_ECC_WR_A".into(),
            width: None,
            value: false.into(),
        });
        param.insert(Param {
            name: "EN_ECC_WR_B".into(),
            width: None,
            value: false.into(),
        });
        param.insert(Param {
            name: "IREG_PRE_A".into(),
            width: None,
            value: false.into(),
        });
        param.insert(Param {
            name: "IREG_PRE_B".into(),
            width: None,
            value: false.into(),
        });
        param.insert(Param {
            name: "IS_CLK_INVERTED".into(),
            width: Some(1),
            value: false.into(),
        });
        param.insert(Param {
            name: "IS_EN_A_INVERTED".into(),
            width: Some(1),
            value: false.into(),
        });
        param.insert(Param {
            name: "IS_EN_B_INVERTED".into(),
            width: Some(1),
            value: false.into(),
        });
        param.insert(Param {
            name: "IS_RDB_WR_A_INVERTED".into(),
            width: Some(1),
            value: false.into(),
        });
        param.insert(Param {
            name: "IS_RDB_WR_B_INVERTED".into(),
            width: Some(1),
            value: false.into(),
        });
        param.insert(Param {
            name: "IS_RST_A_INVERTED".into(),
            width: Some(1),
            value: false.into(),
        });
        param.insert(Param {
            name: "IS_RST_B_INVERTED".into(),
            width: Some(1),
            value: false.into(),
        });
        param.insert(Param {
            name: "OREG_A".into(),
            width: None,
            value: false.into(),
        });
        param.insert(Param {
            name: "OREG_B".into(),
            width: None,
            value: false.into(),
        });
        param.insert(Param {
            name: "OREG_ECC_A".into(),
            width: None,
            value: false.into(),
        });
        param.insert(Param {
            name: "OREG_ECC_B".into(),
            width: None,
            value: false.into(),
        });
        param.insert(Param {
            name: "REG_CAS_A".into(),
            width: None,
            value: false.into(),
        });
        param.insert(Param {
            name: "REG_CAS_B".into(),
            width: None,
            value: false.into(),
        });
        param.insert(Param {
            name: "RST_MODE_A".into(),
            width: None,
            value: RstMode::Sync.into(),
        });
        param.insert(Param {
            name: "RST_MODE_B".into(),
            width: None,
            value: RstMode::Sync.into(),
        });
        param.insert(Param {
            name: "SELF_ADDR_A".into(),
            width: Some(11),
            value: 0i64.into(),
        });
        param.insert(Param {
            name: "SELF_ADDR_B".into(),
            width: Some(11),
            value: 0i64.into(),
        });
        param.insert(Param {
            name: "SELF_MASK_A".into(),
            width: Some(11),
            value: 2047i64.into(),
        });
        param.insert(Param {
            name: "SELF_MASK_B".into(),
            width: Some(11),
            value: 2047i64.into(),
        });
        param.insert(Param {
            name: "USE_EXT_CE_A".into(),
            width: None,
            value: false.into(),
        });
        param.insert(Param {
            name: "USE_EXT_CE_B".into(),
            width: None,
            value: false.into(),
        });
        param
    }
    fn to_input(&self) -> PortSet {
        let mut port = PortSet::new();
        port.insert(Port::new("ADDR_A", 23));
        port.insert(Port::new("ADDR_B", 23));
        port.insert(Port::new("BWE_A", 9));
        port.insert(Port::new("BWE_B", 9));
        port.insert(Port::new("CAS_IN_ADDR_A", 23));
        port.insert(Port::new("CAS_IN_ADDR_B", 23));
        port.insert(Port::new("CAS_IN_BWE_A", 9));
        port.insert(Port::new("CAS_IN_BWE_B", 9));
        port.insert(Port::new("CAS_IN_DBITERR_A", 1));
        port.insert(Port::new("CAS_IN_DBITERR_B", 1));
        port.insert(Port::new("CAS_IN_DIN_A", 72));
        port.insert(Port::new("CAS_IN_DIN_B", 72));
        port.insert(Port::new("CAS_IN_DOUT_A", 72));
        port.insert(Port::new("CAS_IN_DOUT_B", 72));
        port.insert(Port::new("CAS_IN_EN_A", 1));
        port.insert(Port::new("CAS_IN_EN_B", 1));
        port.insert(Port::new("CAS_IN_RDACCESS_A", 1));
        port.insert(Port::new("CAS_IN_RDACCESS_B", 1));
        port.insert(Port::new("CAS_IN_RDB_WR_A", 1));
        port.insert(Port::new("CAS_IN_RDB_WR_B", 1));
        port.insert(Port::new("CAS_IN_SBITERR_A", 1));
        port.insert(Port::new("CAS_IN_SBITERR_B", 1));
        port.insert(Port::new("CLK", 1));
        port.insert(Port::new("DIN_A", 72));
        port.insert(Port::new("DIN_B", 72));
        port.insert(Port::new("EN_A", 1));
        port.insert(Port::new("EN_B", 1));
        port.insert(Port::new("INJECT_DBITERR_A", 1));
        port.insert(Port::new("INJECT_DBITERR_B", 1));
        port.insert(Port::new("INJECT_SBITERR_A", 1));
        port.insert(Port::new("INJECT_SBITERR_B", 1));
        port.insert(Port::new("OREG_CE_A", 1));
        port.insert(Port::new("OREG_CE_B", 1));
        port.insert(Port::new("OREG_ECC_CE_A", 1));
        port.insert(Port::new("OREG_ECC_CE_B", 1));
        port.insert(Port::new("RDB_WR_A", 1));
        port.insert(Port::new("RDB_WR_B", 1));
        port.insert(Port::new("RST_A", 1));
        port.insert(Port::new("RST_B", 1));
        port.insert(Port::new("SLEEP", 1));
        port
    }
    fn to_output(&self) -> PortSet {
        let mut port = PortSet::new();
        port.insert(Port::new("CAS_OUT_ADDR_A", 23));
        port.insert(Port::new("CAS_OUT_ADDR_B", 23));
        port.insert(Port::new("CAS_OUT_BWE_A", 9));
        port.insert(Port::new("CAS_OUT_BWE_B", 9));
        port.insert(Port::new("CAS_OUT_DBITERR_A", 1));
        port.insert(Port::new("CAS_OUT_DBITERR_B", 1));
        port.insert(Port::new("CAS_OUT_DIN_A", 72));
        port.insert(Port::new("CAS_OUT_DIN_B", 72));
        port.insert(Port::new("CAS_OUT_DOUT_A", 72));
        port.insert(Port::new("CAS_OUT_DOUT_B", 72));
        port.insert(Port::new("CAS_OUT_EN_A", 1));
        port.insert(Port::new("CAS_OUT_EN_B", 1));
        port.insert(Port::new("CAS_OUT_RDACCESS_A", 1));
        port.insert(Port::new("CAS_OUT_RDACCESS_B", 1));
        port.insert(Port::new("CAS_OUT_RDB_WR_A", 1));
        port.insert(Port::new("CAS_OUT_RDB_WR_B", 1));
        port.insert(Port::new("CAS_OUT_SBITERR_A", 1));
        port.insert(Port::new("CAS_OUT_SBITERR_B", 1));
        port.insert(Port::new("DBITERR_A", 1));
        port.insert(Port::new("DBITERR_B", 1));
        port.insert(Port::new("DOUT_A", 72));
        port.insert(Port::new("DOUT_B", 72));
        port.insert(Port::new("RDACCESS_A", 1));
        port.insert(Port::new("RDACCESS_B", 1));
        port.insert(Port::new("SBITERR_A", 1));
        port.insert(Port::new("SBITERR_B", 1));
        port
    }
}

impl Default for Uram {
    fn default() -> Self {
        let ram = UramPrim;
        Uram(ram.to_prim())
    }
}
