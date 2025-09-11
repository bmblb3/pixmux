use crate::btree::{BTreeNode, BTreeSpec};

pub struct Pane(BTreeNode<(), ()>);

impl Default for Pane {
    fn default() -> Self {
        Self(BTreeNode::Leaf(()))
    }
}

impl Pane {
    pub fn get_spec(&self) -> BTreeSpec {
        self.0.get_spec()
    }
}
