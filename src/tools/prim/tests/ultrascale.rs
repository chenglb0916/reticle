use anyhow::Result;
use prim::{Param, ParamSet, PortSet, Prim};
use std::fmt;

fn test_name<T: Eq + fmt::Debug + fmt::Display>(prim: &Prim<T>, exp: &str) {
    let res = prim.name();
    assert_eq!(res, exp);
}

fn test_param<T: Eq + fmt::Debug + fmt::Display>(prim: &Prim<T>, exp: &ParamSet<T>) {
    let res = prim.param();
    let inter = res.symmetric_difference(exp);
    assert_eq!(inter.count(), 0);
}

fn test_input<T: Eq + fmt::Debug + fmt::Display>(prim: &Prim<T>, exp: &[(&str, u32)]) {
    let res = prim.input();
    let exp = PortSet::from(exp);
    assert_eq!(*res, exp);
}

fn test_output<T: Eq + fmt::Debug + fmt::Display>(prim: &Prim<T>, exp: &[(&str, u32)]) {
    let res = prim.output();
    let exp = PortSet::from(exp);
    assert_eq!(*res, exp);
}

mod test_carry {
    use super::*;
    use prim::ultrascale::carry::{Carry, CarryParam, CarryType};

    #[test]
    fn name() {
        let prim = Carry::default();
        test_name(&prim, "CARRY8");
    }

    #[test]
    fn param() {
        let prim = Carry::default();
        let mut param = ParamSet::<CarryParam>::new();
        param.insert(Param {
            name: "CARRY_TYPE".to_string(),
            width: None,
            value: CarryType::Single.into(),
        });
        test_param(&prim, &param);
    }

    #[test]
    fn input() {
        let prim = Carry::default();
        let input = [("DI", 8), ("S", 8), ("CI", 1), ("CI_TOP", 1)];
        test_input(&prim, &input);
    }

    #[test]
    fn output() {
        let prim = Carry::default();
        let output = [("O", 8), ("CO", 8)];
        test_output(&prim, &output);
    }

    #[test]
    fn set_param() -> Result<()> {
        let mut prim = Carry::default();
        prim.set_param("CARRY_TYPE", CarryType::Dual)?;
        Ok(())
    }
}

mod test_gnd {
    use super::*;
    use prim::ultrascale::gnd::Gnd;

    #[test]
    fn name() {
        let prim = Gnd::default();
        test_name(&prim, "GND");
    }

    #[test]
    fn param() {
        let prim = Gnd::default();
        let param: ParamSet<_> = ParamSet::new();
        test_param(&prim, &param);
    }

    #[test]
    fn input() {
        let prim = Gnd::default();
        let input = [];
        test_input(&prim, &input);
    }

    #[test]
    fn output() {
        let prim = Gnd::default();
        let output = [("G", 1)];
        test_output(&prim, &output);
    }
}

mod test_bram {
    use super::*;
    use prim::ultrascale::bram::*;

    #[test]
    fn name() {
        let prim = Bram::default();
        test_name(&prim, "RAMB18E2");
    }

    #[test]
    fn param() {
        let prim = Bram::default();
        let mut param = ParamSet::<BramParam>::new();
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
            name: "CLOCK_DOMAINS".into(),
            width: None,
            value: ClockDomains::Independent.into(),
        });
        param.insert(Param {
            name: "SIM_COLLISION_CHECK".into(),
            width: None,
            value: CollisionCheck::All.into(),
        });
        param.insert(Param {
            name: "DOA_REG".into(),
            width: None,
            value: false.into(),
        });
        param.insert(Param {
            name: "DOB_REG".into(),
            width: None,
            value: false.into(),
        });
        param.insert(Param {
            name: "ENADDRENA".into(),
            width: None,
            value: false.into(),
        });
        param.insert(Param {
            name: "ENADDRENB".into(),
            width: None,
            value: false.into(),
        });
        for i in 0..8 {
            let name = format!("INITP_{:02X}", i);
            param.insert(Param {
                name,
                width: Some(256),
                value: vec![0; 32].into(),
            });
        }
        for i in 0..64 {
            let name = format!("INIT_{:02X}", i);
            param.insert(Param {
                name,
                width: Some(256),
                value: vec![0; 32].into(),
            });
        }
        param.insert(Param {
            name: "INIT_A".into(),
            width: Some(18),
            value: 0i64.into(),
        });
        param.insert(Param {
            name: "INIT_B".into(),
            width: Some(18),
            value: 0i64.into(),
        });
        param.insert(Param {
            name: "INIT_FILE".into(),
            width: None,
            value: FilePath::None.into(),
        });
        param.insert(Param {
            name: "IS_CLKARDCLK_INVERTED".into(),
            width: Some(1),
            value: false.into(),
        });
        param.insert(Param {
            name: "IS_CLKBWRCLK_INVERTED".into(),
            width: Some(1),
            value: false.into(),
        });
        param.insert(Param {
            name: "IS_ENARDEN_INVERTED".into(),
            width: Some(1),
            value: false.into(),
        });
        param.insert(Param {
            name: "IS_ENBWREN_INVERTED".into(),
            width: Some(1),
            value: false.into(),
        });
        param.insert(Param {
            name: "IS_RSTRAMARSTRAM_INVERTED".into(),
            width: Some(1),
            value: false.into(),
        });
        param.insert(Param {
            name: "IS_RSTRAMB_INVERTED".into(),
            width: Some(1),
            value: false.into(),
        });
        param.insert(Param {
            name: "IS_RSTREGARSTREG_INVERTED".into(),
            width: Some(1),
            value: false.into(),
        });
        param.insert(Param {
            name: "IS_RSTREGB_INVERTED".into(),
            width: Some(1),
            value: false.into(),
        });
        param.insert(Param {
            name: "RDADDRCHANGEA".into(),
            width: None,
            value: false.into(),
        });
        param.insert(Param {
            name: "RDADDRCHANGEB".into(),
            width: None,
            value: false.into(),
        });
        param.insert(Param {
            name: "READ_WIDTH_A".into(),
            width: None,
            value: 0i64.into(),
        });
        param.insert(Param {
            name: "READ_WIDTH_B".into(),
            width: None,
            value: 0i64.into(),
        });
        param.insert(Param {
            name: "WRITE_WIDTH_A".into(),
            width: None,
            value: 0i64.into(),
        });
        param.insert(Param {
            name: "WRITE_WIDTH_B".into(),
            width: None,
            value: 0i64.into(),
        });
        param.insert(Param {
            name: "RSTREG_PRIORITY_A".into(),
            width: None,
            value: RstRegPriority::RstReg.into(),
        });
        param.insert(Param {
            name: "RSTREG_PRIORITY_B".into(),
            width: None,
            value: RstRegPriority::RstReg.into(),
        });
        param.insert(Param {
            name: "SRVAL_A".into(),
            width: Some(18),
            value: 0i64.into(),
        });
        param.insert(Param {
            name: "SRVAL_B".into(),
            width: Some(18),
            value: 0i64.into(),
        });
        param.insert(Param {
            name: "SLEEP_ASYNC".into(),
            width: None,
            value: false.into(),
        });
        param.insert(Param {
            name: "WRITE_MODE_A".into(),
            width: None,
            value: WriteMode::NoChange.into(),
        });
        param.insert(Param {
            name: "WRITE_MODE_B".into(),
            width: None,
            value: WriteMode::NoChange.into(),
        });
        test_param(&prim, &param);
    }

    #[test]
    fn input() {
        let prim = Bram::default();
        let input = [
            ("CASDIMUXA", 1),
            ("CASDIMUXB", 1),
            ("CASDINA", 16),
            ("CASDINB", 16),
            ("CASDINPA", 2),
            ("CASDINPB", 2),
            ("CASDOMUXA", 1),
            ("CASDOMUXB", 1),
            ("CASDOMUXEN_A", 1),
            ("CASDOMUXEN_B", 1),
            ("CASOREGIMUXA", 1),
            ("CASOREGIMUXB", 1),
            ("CASOREGIMUXEN_A", 1),
            ("CASOREGIMUXEN_B", 1),
            ("ADDRARDADDR", 14),
            ("ADDRENA", 1),
            ("CLKARDCLK", 1),
            ("ENARDEN", 1),
            ("REGCEAREGCE", 1),
            ("RSTRAMARSTRAM", 1),
            ("RSTREGARSTREG", 1),
            ("WEA", 2),
            ("DINADIN", 16),
            ("DINPADINP", 2),
            ("ADDRBWRADDR", 14),
            ("ADDRENB", 1),
            ("CLKBWRCLK", 1),
            ("ENBWREN", 1),
            ("REGCEB", 1),
            ("RSTRAMB", 1),
            ("RSTREGB", 1),
            ("SLEEP", 1),
            ("WEBWE", 4),
            ("DINBDIN", 16),
            ("DINPBDINP", 2),
        ];
        test_input(&prim, &input);
    }

    #[test]
    fn output() {
        let prim = Bram::default();
        let output = [
            ("CASDOUTA", 16),
            ("CASDOUTB", 16),
            ("CASDOUTPA", 2),
            ("CASDOUTPB", 2),
            ("DOUTADOUT", 16),
            ("DOUTPADOUTP", 2),
            ("DOUTBDOUT", 16),
            ("DOUTPBDOUTP", 2),
        ];
        test_output(&prim, &output);
    }
}

mod test_lram {
    use super::*;
    use prim::ultrascale::lram::*;

    const LUT: [&str; 8] = ["A", "B", "C", "D", "E", "F", "G", "H"];

    #[test]
    fn name() {
        let prim = Lram::default();
        test_name(&prim, "RAM64M8");
    }

    #[test]
    fn param() {
        let prim = Lram::default();
        let mut param = ParamSet::<LramParam>::new();
        for l in LUT.iter() {
            let name = format!("INIT_{}", l);
            param.insert(Param {
                name,
                width: Some(64),
                value: vec![0; 8].into(),
            });
        }
        param.insert(Param {
            name: "IS_WCLK_INVERTED".into(),
            width: Some(1),
            value: false.into(),
        });
        test_param(&prim, &param);
    }

    #[test]
    fn input() {
        let prim = Lram::default();
        let input = [
            ("DIA", 1),
            ("DIB", 1),
            ("DIC", 1),
            ("DID", 1),
            ("DIE", 1),
            ("DIF", 1),
            ("DIG", 1),
            ("DIH", 1),
            ("ADDRA", 6),
            ("ADDRB", 6),
            ("ADDRC", 6),
            ("ADDRD", 6),
            ("ADDRE", 6),
            ("ADDRF", 6),
            ("ADDRG", 6),
            ("ADDRH", 6),
            ("WE", 1),
            ("WCLK", 1),
        ];
        test_input(&prim, &input);
    }

    #[test]
    fn output() {
        let prim = Lram::default();
        let output = [
            ("DOA", 1),
            ("DOB", 1),
            ("DOC", 1),
            ("DOD", 1),
            ("DOE", 1),
            ("DOF", 1),
            ("DOG", 1),
            ("DOH", 1),
        ];
        test_output(&prim, &output);
    }
}

mod test_uram {
    use super::*;
    use prim::ultrascale::uram::*;

    #[test]
    fn name() {
        let prim = Uram::default();
        test_name(&prim, "URAM288");
    }

    #[test]
    fn param() {
        let prim = Uram::default();
        let mut param = ParamSet::new();
        param.insert(Param {
            name: "AUTO_SLEEP_LATENCY".into(),
            width: None,
            value: 8i64.into(),
        });
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
        test_param(&prim, &param);
    }

    #[test]
    fn input() {
        let prim = Uram::default();
        let input = [
            ("ADDR_A", 23),
            ("ADDR_B", 23),
            ("BWE_A", 9),
            ("BWE_B", 9),
            ("CAS_IN_ADDR_A", 23),
            ("CAS_IN_ADDR_B", 23),
            ("CAS_IN_BWE_A", 9),
            ("CAS_IN_BWE_B", 9),
            ("CAS_IN_DBITERR_A", 1),
            ("CAS_IN_DBITERR_B", 1),
            ("CAS_IN_DIN_A", 72),
            ("CAS_IN_DIN_B", 72),
            ("CAS_IN_DOUT_A", 72),
            ("CAS_IN_DOUT_B", 72),
            ("CAS_IN_EN_A", 1),
            ("CAS_IN_EN_B", 1),
            ("CAS_IN_RDACCESS_A", 1),
            ("CAS_IN_RDACCESS_B", 1),
            ("CAS_IN_RDB_WR_A", 1),
            ("CAS_IN_RDB_WR_B", 1),
            ("CAS_IN_SBITERR_A", 1),
            ("CAS_IN_SBITERR_B", 1),
            ("CLK", 1),
            ("DIN_A", 72),
            ("DIN_B", 72),
            ("EN_A", 1),
            ("EN_B", 1),
            ("INJECT_DBITERR_A", 1),
            ("INJECT_DBITERR_B", 1),
            ("INJECT_SBITERR_A", 1),
            ("INJECT_SBITERR_B", 1),
            ("OREG_CE_A", 1),
            ("OREG_CE_B", 1),
            ("OREG_ECC_CE_A", 1),
            ("OREG_ECC_CE_B", 1),
            ("RDB_WR_A", 1),
            ("RDB_WR_B", 1),
            ("RST_A", 1),
            ("RST_B", 1),
            ("SLEEP", 1),
        ];
        test_input(&prim, &input);
    }

    #[test]
    fn output() {
        let prim = Uram::default();
        let output = [
            ("CAS_OUT_ADDR_A", 23),
            ("CAS_OUT_ADDR_B", 23),
            ("CAS_OUT_BWE_A", 9),
            ("CAS_OUT_BWE_B", 9),
            ("CAS_OUT_DBITERR_A", 1),
            ("CAS_OUT_DBITERR_B", 1),
            ("CAS_OUT_DIN_A", 72),
            ("CAS_OUT_DIN_B", 72),
            ("CAS_OUT_DOUT_A", 72),
            ("CAS_OUT_DOUT_B", 72),
            ("CAS_OUT_EN_A", 1),
            ("CAS_OUT_EN_B", 1),
            ("CAS_OUT_RDACCESS_A", 1),
            ("CAS_OUT_RDACCESS_B", 1),
            ("CAS_OUT_RDB_WR_A", 1),
            ("CAS_OUT_RDB_WR_B", 1),
            ("CAS_OUT_SBITERR_A", 1),
            ("CAS_OUT_SBITERR_B", 1),
            ("DBITERR_A", 1),
            ("DBITERR_B", 1),
            ("DOUT_A", 72),
            ("DOUT_B", 72),
            ("RDACCESS_A", 1),
            ("RDACCESS_B", 1),
            ("SBITERR_A", 1),
            ("SBITERR_B", 1),
        ];
        test_output(&prim, &output);
    }
}
