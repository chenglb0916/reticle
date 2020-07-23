use crate::backend::target::descriptor::Tile;
use crate::passes::select::instr::*;
use petgraph::graph::NodeIndex;
use petgraph::prelude::Graph;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct SDNode {
    pub name: String,
    pub instr: Instr,
    pub tile: Option<Tile>,
}

#[derive(Clone, Debug)]
pub struct SDEdge;

pub type SDGraph = Graph<SDNode, SDEdge>;
pub type SDNodeIx = NodeIndex;
pub type SDId = String;
pub type SDCtx = HashMap<SDId, SDNodeIx>;

#[derive(Clone, Debug)]
pub struct SDag {
    pub graph: SDGraph,
    pub ctx: SDCtx,
}
