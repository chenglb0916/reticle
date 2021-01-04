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

    fn val(input: Node) -> Result<ExprTerm> {
        let val = input.as_str().parse::<i64>();
        match val {
            Ok(v) => Ok(ExprTerm::Val(v)),
            Err(_) => panic!("Error: parsing {} as i64", input.as_str()),
        }
    }

    fn ty(input: Node) -> Result<Ty> {
        let ty = Ty::from_str(input.as_str());
        match ty {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
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
                bel,
                x,
                y,
            },
        ))
    }

    fn var(input: Node) -> Result<ExprTerm> {
        Ok(match_nodes!(
            input.into_children();
            [id(id), ty(ty)] => ExprTerm::Var(id, ty),
            [id(id)] => ExprTerm::Var(id, Ty::Any),
            [] => ExprTerm::Any,
        ))
    }

    fn tup_var(input: Node) -> Result<ExprTup> {
        Ok(match_nodes!(
            input.into_children();
            [var(vars)..] => ExprTup{ term: vars.collect() },
        ))
    }

    fn tup_val(input: Node) -> Result<ExprTup> {
        Ok(match_nodes!(
            input.into_children();
            [val(vals)..] => ExprTup{ term: vals.collect() },
        ))
    }

    fn io(input: Node) -> Result<Expr> {
        Ok(match_nodes!(
            input.into_children();
            [var(var)] => Expr::from(var),
            [tup_var(tup)] => Expr::from(tup),
        ))
    }

    fn op_mach(input: Node) -> Result<OpMach> {
        let op = OpMach::from_str(input.as_str());
        match op {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn op_basc(input: Node) -> Result<OpBasc> {
        let op = OpBasc::from_str(input.as_str());
        match op {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn opt_key(input: Node) -> Result<Opt> {
        let opt = Opt::from_str(input.as_str());
        match opt {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn opt_num(input: Node) -> Result<OptVal> {
        let val = input.as_str().parse::<u64>();
        match val {
            Ok(v) => Ok(OptVal::UInt(v)),
            Err(_) => panic!("Error: parsing {} as u64", input.as_str()),
        }
    }

    fn opt_op(input: Node) -> Result<OptVal> {
        let op = OpDsp::from_str(input.as_str());
        match op {
            Ok(v) => Ok(OptVal::Op(v)),
            Err(_) => panic!("Error: parsing {} as u64", input.as_str()),
        }
    }

    fn opt_tup(input: Node) -> Result<(Opt, OptVal)> {
        Ok(match_nodes!(
            input.into_children();
            [opt_key(key), opt_num(val)] => (key, val),
            [opt_key(key), opt_op(val)] => (key, val)
        ))
    }

    fn opt(input: Node) -> Result<OptMap> {
        Ok(match_nodes!(
            input.into_children();
            [opt_tup(tup)..] => {
                let mut map = OptMap::new();
                for (key, val) in tup {
                    map.insert(key, val);
                }
                map
            }
        ))
    }

    fn instr_mach(input: Node) -> Result<InstrMach> {
        Ok(match_nodes!(
            input.into_children();
            [io(dst), op_mach(op), io(arg)] => InstrMach {
                op,
                opt: OptMap::new(),
                dst,
                arg,
                loc: None,
            },
            [io(dst), op_mach(op), opt(opt), io(arg)] => InstrMach {
                op,
                opt,
                dst,
                arg,
                loc: None,
            },
            [io(dst), op_mach(op), io(arg), loc(loc)] => InstrMach {
                op,
                opt: OptMap::new(),
                dst,
                arg,
                loc: Some(loc),
            },
            [io(dst), op_mach(op), opt(opt), io(arg), loc(loc)] => InstrMach {
                op,
                opt,
                dst,
                arg,
                loc: Some(loc),
            }
        ))
    }

    fn instr_basc(input: Node) -> Result<InstrBasc> {
        Ok(match_nodes!(
            input.into_children();
            [io(dst), op_basc(op), tup_val(attr)] => InstrBasc {
                op,
                dst,
                attr: Expr::from(attr),
                arg: Expr::default(),
            },
            [io(dst), op_basc(op), io(arg)] => InstrBasc {
                op,
                dst,
                attr: Expr::default(),
                arg,
            },
            [io(dst), op_basc(op), tup_val(attr), io(arg)] => InstrBasc {
                op,
                dst,
                attr: Expr::from(attr),
                arg,
            }
        ))
    }

    fn instr(input: Node) -> Result<Instr> {
        Ok(match_nodes!(
            input.into_children();
            [instr_mach(instr)] => Instr::from(instr),
            [instr_basc(instr)] => Instr::from(instr),
        ))
    }

    fn body(input: Node) -> Result<Vec<Instr>> {
        Ok(match_nodes!(
            input.into_children();
            [instr(instr)..] => instr.collect(),
        ))
    }

    fn sig(input: Node) -> Result<Sig> {
        Ok(match_nodes!(
            input.into_children();
            [id(id), io(output)] => Sig {
                id,
                input: Expr::default(),
                output,
            },
            [id(id), io(input), io(output)] => Sig {
                id,
                input,
                output,
            },
        ))
    }

    fn prog(input: Node) -> Result<Prog> {
        Ok(match_nodes!(
            input.into_children();
            [sig(sig), body(body)] => Prog {
                sig,
                body,
            },
        ))
    }

    fn file(input: Node) -> Result<Prog> {
        Ok(match_nodes!(
            input.into_children();
            [prog(prog), _] => prog,
        ))
    }
}

impl MLParser {
    pub fn parse_from_str(input_str: &str) -> Result<Prog> {
        let inputs = MLParser::parse(Rule::file, input_str)?;
        let input = inputs.single()?;
        Ok(MLParser::file(input)?)
    }
    pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Result<Prog> {
        let content = read_to_string(path);
        MLParser::parse_from_str(&content)
    }
}
