use crate::btree::{BTreeNode, BTreeSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaneData {
    pub imagefile: String,
}

type PaneSpec = BTreeSpec<PaneData, ()>;

#[derive(Default)]
pub struct Pane(BTreeNode<PaneData, ()>);

impl Pane {
    pub fn get_spec(&self) -> PaneSpec {
        self.0.get_spec()
    }
}
