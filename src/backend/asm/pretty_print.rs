use crate::backend::asm::ast::*;
use crate::util::pretty_print::{PrettyPrint, PRETTY_INDENT};
use pretty::RcDoc;

impl PrettyPrint for Expr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Expr::Ref(n, _) => RcDoc::as_string(n),
        }
    }
}

impl PrettyPrint for LocExpr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            LocExpr::Hole => RcDoc::text("??"),
            LocExpr::Var(n) => RcDoc::as_string(n),
            LocExpr::Lit(n) => RcDoc::as_string(n),
        }
    }
}

impl PrettyPrint for Loc {
    fn to_doc(&self) -> RcDoc<()> {
        self.ty
            .to_doc()
            .append(RcDoc::text("("))
            .append(self.x.to_doc())
            .append(RcDoc::text(","))
            .append(RcDoc::space())
            .append(self.y.to_doc())
            .append(RcDoc::text(")"))
    }
}

impl PrettyPrint for Instr {
    fn to_doc(&self) -> RcDoc<()> {
        let dst_doc = RcDoc::as_string(self.dst())
            .append(RcDoc::space())
            .append(RcDoc::text(":"))
            .append(RcDoc::space())
            .append(self.ty.to_doc())
            .append(RcDoc::space())
            .append(RcDoc::text("="))
            .append(RcDoc::space());
        let params_doc = if self.params.is_empty() {
            RcDoc::nil()
        } else {
            RcDoc::text("(")
                .append(RcDoc::intersperse(
                    self.params.iter().map(|p| p.to_doc()),
                    RcDoc::text(",").append(RcDoc::space()),
                ))
                .append(RcDoc::text(")"))
        };
        let op_doc = RcDoc::as_string(&self.op);
        let loc_doc = RcDoc::space()
            .append(RcDoc::text("@"))
            .append(self.loc.to_doc());
        dst_doc.append(op_doc).append(params_doc).append(loc_doc)
    }
}

impl PrettyPrint for Prog {
    fn to_doc(&self) -> RcDoc<()> {
        let inputs_doc = RcDoc::intersperse(
            self.sig.inputs().iter().map(|i| i.to_doc()),
            RcDoc::text(",").append(RcDoc::space()),
        );
        let outputs_doc = RcDoc::intersperse(
            self.sig.outputs().iter().map(|o| o.to_doc()),
            RcDoc::text(",").append(RcDoc::space()),
        );
        let mut body_doc = RcDoc::nil();
        for instr in self.body().iter() {
            body_doc = body_doc
                .append(RcDoc::hardline())
                .append(instr.to_doc())
                .append(RcDoc::text(";"));
        }
        body_doc = body_doc.nest(PRETTY_INDENT).group();
        RcDoc::text("def")
            .append(RcDoc::space())
            .append(RcDoc::as_string(self.sig.id()))
            .append(RcDoc::text("("))
            .append(inputs_doc)
            .append(RcDoc::text(")"))
            .append(RcDoc::space())
            .append(RcDoc::text("->"))
            .append(RcDoc::space())
            .append(RcDoc::text("("))
            .append(outputs_doc)
            .append(RcDoc::text(")"))
            .append(RcDoc::space())
            .append(RcDoc::text("{"))
            .append(body_doc)
            .append(RcDoc::hardline())
            .append(RcDoc::text("}"))
    }
}
