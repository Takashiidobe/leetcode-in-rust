use std::{cell::RefCell, rc::Rc};

use crate::GraphNode;

pub fn clone_graph(node: Option<Rc<RefCell<GraphNode>>>) -> GraphNode {
    let mut nodes = vec![GraphNode::default(); 101];

    fn traverse(node: &Rc<GraphNode>) {}

    GraphNode::default()
}
