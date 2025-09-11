use color_eyre::Result;

use crate::btree::{BTreeNode, BTreeSpec};

#[derive(Default)]
pub struct Pane(BTreeNode<PaneData, SplitData>);

impl Pane {
    pub fn get_spec(&self) -> PaneSpec {
        self.0.get_spec()
    }

    pub fn vsplit(&mut self, path: &[bool]) -> Result<()> {
        self.0.split_leaf_at(path, SplitData::default_vsplit())
    }

    pub fn hsplit(&mut self, path: &[bool]) -> Result<()> {
        self.0.split_leaf_at(path, SplitData::default_hsplit())
    }
}

#[derive(Clone, Default)]
pub struct PaneData {
    pub imagefile: String,
}

#[derive(Clone)]
pub struct SplitData {
    pub direction: SplitDirection,
    pub pct: u8,
}

#[derive(Clone)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

impl Default for SplitData {
    fn default() -> Self {
        Self::default_vsplit()
    }
}

impl SplitData {
    fn default_vsplit() -> Self {
        Self {
            direction: SplitDirection::Horizontal,
            pct: 50,
        }
    }

    fn default_hsplit() -> Self {
        Self {
            direction: SplitDirection::Vertical,
            pct: 50,
        }
    }
}

type PaneSpec = BTreeSpec<PaneData, SplitData>;
