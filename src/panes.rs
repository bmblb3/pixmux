use color_eyre::Result;

use crate::btree::{BTreeNode, BTreeSpec};

pub type PaneType = BTreeNode<PaneData, SplitData>;

#[derive(Default)]
pub struct Pane(pub PaneType);

impl Pane {
    pub fn inner(&self) -> &BTreeNode<PaneData, SplitData> {
        &self.0
    }

    pub fn get_spec(&self) -> PaneSpec {
        self.0.get_spec()
    }

    pub fn vsplit(&mut self, path: &[bool]) -> Result<()> {
        self.0.split_leaf_at(path, SplitData::default_vsplit())
    }

    pub fn hsplit(&mut self, path: &[bool]) -> Result<()> {
        self.0.split_leaf_at(path, SplitData::default_hsplit())
    }

    pub fn remove(&mut self, path: &[bool]) -> Result<()> {
        self.0.remove_leaf_at(path)
    }

    pub fn next(&mut self, path: &Option<Vec<bool>>) -> Result<Vec<bool>> {
        self.0.next_path(path)
    }

    pub fn prev(&mut self, path: &Option<Vec<bool>>) -> Result<Vec<bool>> {
        self.0.prev_path(path)
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
            direction: SplitDirection::Vertical,
            pct: 50,
        }
    }

    fn default_hsplit() -> Self {
        Self {
            direction: SplitDirection::Horizontal,
            pct: 50,
        }
    }
}

type PaneSpec = BTreeSpec<PaneData, SplitData>;
