use crate::lang::ast::{PrimOp, Ty};
use petgraph::graph::NodeIndex;
use petgraph::prelude::Graph;
use std::collections::HashMap;

pub type TreeId = String;
pub type TreeTy = Ty;
pub type TreeIx = NodeIndex;
pub type TreeGraph = Graph<TreeNode, TreeEdge>;
pub type TreeCtx = HashMap<TreeId, TreeIx>;

#[derive(Clone, Debug)]
pub enum TreeOp {
    Input,
    Prim(PrimOp),
}

#[derive(Default, Clone, Debug)]
pub struct TreeEdge;

#[derive(Clone, Debug)]
pub struct TreeNode {
    pub id: TreeId,
    pub ty: TreeTy,
    pub op: TreeOp,
}

#[derive(Clone, Debug)]
pub struct Tree {
    pub graph: TreeGraph,
    pub ctx: TreeCtx,
}
