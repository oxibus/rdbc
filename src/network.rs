use super::node::Node;
use std::collections::HashMap;

pub struct Network {
    /// nodes (BU)
    ///
    /// - key: node name
    /// - value: node
    pub nodes: HashMap<String, Node>,
}
