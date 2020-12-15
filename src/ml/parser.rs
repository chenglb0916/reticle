use crate::ml::ast::*;
use crate::util::file::read_to_string;
use pest_consume::{match_nodes, Error, Parser};
use std::path::Path;
use std::rc::Rc;
use std::str::FromStr;

pub type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

const _GRAMMAR: &str = include_str!("syntax.pest");

#[derive(Parser)]
#[grammar = "ml/syntax.pest"]
pub struct MLParser;

#[pest_consume::parser]
impl MLParser {
    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }

    fn id(input: Node) -> Result<Id> {
        Ok(input.as_str().to_string())
    }

    fn op_coord(input: Node) -> Result<OpCoord> {
        let op = OpCoord::from_str(input.as_str());
        match op {
            Ok(e) => Ok(e),
            Err(m) => panic!("{}", m),
        }
    }

    fn coord(input: Node) -> Result<ExprCoord> {
        let expr = ExprCoord::from_str(input.as_str());
        match expr {
            Ok(e) => Ok(e),
            Err(m) => panic!("{}", m),
        }
    }

    fn expr_coord(input: Node) -> Result<ExprCoord> {
        Ok(match_nodes!(
            input.into_children();
            [coord(coord)] => coord,
            [coord(lhs), op_coord(op), coord(rhs)] => ExprCoord::Bin(op, Rc::new(lhs), Rc::new(rhs)),
        ))
    }

    fn bel(input: Node) -> Result<Bel> {
        let bel = Bel::from_str(input.as_str());
        match bel {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn loc(input: Node) -> Result<Loc> {
        Ok(match_nodes!(
            input.into_children();
            [bel(bel), expr_coord(x), expr_coord(y)] => Loc {
                bel: Some(bel),
                x,
                y,
            },
            [expr_coord(x), expr_coord(y)] => Loc {
                bel: None,
                x,
                y,
            },
        ))
    }

    fn op_mach(input: Node) -> Result<OpMach> {
        let op = OpMach::from_str(input.as_str());
        match op {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn op_wire(input: Node) -> Result<OpWire> {
        let op = OpWire::from_str(input.as_str());
        match op {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn instr_mach(input: Node) -> Result<Instr> {
        Ok(match_nodes!(
            input.into_children();
            [op_mach(op), loc(loc)] => Instr::from(
                InstrMach {
                    op,
                    opt: OptMap::new(),
                    dst: Expr::default(),
                    arg: Expr::default(),
                    loc
                }
            )
        ))
    }

    fn instr_wire(input: Node) -> Result<Instr> {
        Ok(match_nodes!(
            input.into_children();
            [op_wire(op)] => Instr::from(
                InstrWire {
                    op,
                    dst: Expr::default(),
                    attr: Expr::default(),
                    arg: Expr::default(),
                }
            )
        ))
    }

    fn instr(input: Node) -> Result<Instr> {
        Ok(match_nodes!(
            input.into_children();
            [instr_mach(instr)] => instr,
            [instr_wire(instr)] => instr,
        ))
    }

    fn file(input: Node) -> Result<Instr> {
        Ok(match_nodes!(
            input.into_children();
            [instr(instr), _] => instr,
        ))
    }
}

impl MLParser {
    pub fn parse_from_str(input_str: &str) -> Result<Instr> {
        let inputs = MLParser::parse(Rule::file, input_str)?;
        let input = inputs.single()?;
        Ok(MLParser::file(input)?)
    }
    pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Result<Instr> {
        let content = read_to_string(path);
        MLParser::parse_from_str(&content)
    }
}
