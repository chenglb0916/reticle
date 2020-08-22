pub mod dag;
pub mod partition;

use crate::backend::arch::ultrascale::Ultrascale;
use crate::backend::target::{Descriptor, Target, Tile};
use crate::lang::ast::Prog;
use crate::passes::map::dag::Dag;
use crate::passes::map::partition::tree::{Tree, TreeGraph, TreeIx, TreeNode};
use crate::passes::map::partition::Partition;
use petgraph::visit::{Bfs, DfsPostOrder};
use petgraph::Direction;
use std::collections::HashSet;

fn tree_node_stack(graph: TreeGraph, start: TreeIx) -> Vec<TreeNode> {
    let mut stack: Vec<TreeNode> = Vec::new();
    let mut visit = Bfs::new(&graph, start);
    while let Some(ix) = visit.next(&graph) {
        if let Some(node) = graph.node_weight(ix) {
            stack.push(node.clone());
        }
    }
    stack
}

fn tree_matches_index(pattern: Tree, input: Tree, input_index: TreeIx) -> Vec<TreeIx> {
    let mut update: Vec<TreeIx> = Vec::new();
    let pattern_index = pattern.root_index().unwrap();
    let pstack = tree_node_stack(pattern.graph().clone(), pattern_index);
    let mut pstack_iter = pstack.iter();
    let mut visit = Bfs::new(&input.graph, input_index);
    let mut discard: HashSet<TreeIx> = HashSet::new();
    while let Some(ix) = visit.next(&input.graph) {
        if !discard.contains(&ix) {
            if let Some(pnode) = pstack_iter.next() {
                if !pnode.is_input() {
                    update.push(ix);
                }
                // discard childs if parent node in pattern is input
                if pnode.is_input() {
                    let childs = input.graph.neighbors_directed(ix, Direction::Outgoing);
                    for c in childs {
                        discard.insert(c);
                    }
                }
            }
        }
        if !pstack_iter.len() == 0 {
            break;
        }
    }
    update
}

fn tree_match(pattern: Tree, input: Tree, input_index: TreeIx) -> bool {
    let mut is_match: bool = true;
    let pattern_index = pattern.root_index().unwrap();
    let pstack = tree_node_stack(pattern.graph().clone(), pattern_index);
    let mut pstack_iter = pstack.iter();
    let mut visit = Bfs::new(&input.graph, input_index);
    let mut discard: HashSet<TreeIx> = HashSet::new();
    while let Some(ix) = visit.next(&input.graph) {
        if !discard.contains(&ix) {
            if let Some(pnode) = pstack_iter.next() {
                if let Some(inode) = input.graph.node_weight(ix) {
                    if pnode.ty() != inode.ty() {
                        is_match = false;
                    }
                    if !pnode.is_input() && pnode.op() != inode.op() {
                        is_match = false;
                    }
                    // discard childs if parent node in pattern is input
                    if pnode.is_input() {
                        let childs = input.graph.neighbors_directed(ix, Direction::Outgoing);
                        for c in childs {
                            discard.insert(c);
                        }
                    }
                }
            }
        }
        if !is_match || !pstack_iter.len() == 0 {
            break;
        }
    }
    is_match && pstack_iter.len() == 0
}

fn tree_update(input: Tree, index: TreeIx, tile: Tile) -> Tree {
    let mut output = input;
    if let Some(node) = output.graph.node_weight_mut(index) {
        node.set_cost(tile.pattern.estimate_cost());
        node.set_instr(tile.instr);
    }
    output
}

fn tree_reset(pattern: Tree, input: Tree, input_index: TreeIx) -> Tree {
    let mut output = input.clone();
    let matches = tree_matches_index(pattern, input, input_index);
    for index in matches.iter() {
        if let Some(node) = output.graph.node_weight_mut(*index) {
            node.clear_instr();
            node.set_cost(0.0);
        }
    }
    output
}

fn tree_selection(descriptor: Descriptor, input: Tree) -> Tree {
    let mut output = input.clone();
    let start = input.root_index().unwrap();
    let mut dfs = DfsPostOrder::new(&input.graph(), start);
    while let Some(ix) = dfs.next(&input.graph) {
        if let Some(node) = input.graph.node_weight(ix) {
            if !node.is_input() {
                for tile in descriptor.tiles.iter() {
                    if tree_match(tile.pattern.clone(), input.clone(), ix) {
                        let pat_cost = tile.pattern.estimate_cost();
                        let cur_cost = input.estimate_cost_from_index(ix);
                        if pat_cost < cur_cost {
                            output = tree_reset(tile.pattern.clone(), output.clone(), ix);
                            output = tree_update(output.clone(), ix, tile.clone());
                        }
                    }
                }
            }
        }
    }
    output
}

fn tree_codegen(input: Tree) {
    let root_index = input.root_index().unwrap();
    let graph = input.graph;
    let mut visit = DfsPostOrder::new(&graph, root_index);
    while let Some(ix) = visit.next(&graph) {
        if let Some(node) = graph.node_weight(ix) {
            if let Some(instr) = node.instr() {
                println!("{}", instr);
            }
        }
    }
}

pub fn example(prog: Prog) {
    let descriptor = Ultrascale::default().to_descriptor();
    let dag = Dag::from(prog);
    let input = Partition::from(dag);
    let mut output = Partition::new();
    for (id, tree) in input.iter() {
        println!("\n{}", tree);
        output.insert(
            id.to_string(),
            tree_selection(descriptor.clone(), tree.clone()),
        );
    }
    for (id, tree) in output.iter() {
        println!("\ncodegen ---> {}", id);
        tree_codegen(tree.clone());
    }
}
