use super::node::Node;
use super::node_id::NodeId;
use std::collections::HashMap;

pub struct NodeManager {
    node_map: HashMap<NodeId, Box<dyn Node>>,
}

impl NodeManager {}
