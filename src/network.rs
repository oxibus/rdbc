use std::collections::HashMap;

use super::node::Node;

pub struct Network {
    /// nodes (BU)
    ///
    /// - key: node name
    /// - value: node
    pub nodes: HashMap<String, Node>,
}
