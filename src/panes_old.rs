use color_eyre::eyre::{self, Ok, OptionExt};
use ratatui::layout;

use crate::{AdjustDirection, cycle_index};

#[derive(Clone)]
pub enum Pane {
    Leaf {
        image_id: usize,
    },
    Split {
        direction: layout::Direction,
        pct: u8,
        first: Box<Pane>,
        second: Box<Pane>,
    },
}

impl Default for Pane {
    fn default() -> Self {
        Pane::new_leaf()
    }
}

macro_rules! impl_get_node_at {
    ($self:ident, $path:ident, $method:ident) => {
        if $path.is_empty() {
            Ok($self)
        } else {
            match $self {
                Pane::Leaf { .. } => Err(eyre::eyre!("Path leads beyond a leaf node")),
                Pane::Split { first, second, .. } => {
                    let child = if $path[0] { first } else { second };
                    child.$method(&$path[1..])
                }
            }
        }
    };
}

impl Pane {
    pub fn new_leaf() -> Self {
        Pane::Leaf { image_id: 0 }
    }

    pub fn new_split(direction: layout::Direction) -> Pane {
        Pane::Split {
            direction,
            pct: 50,
            first: Box::new(Self::new_leaf()),
            second: Box::new(Self::new_leaf()),
        }
    }

    fn collect_leaf_paths_impl(
        &self,
        current_path: &mut Vec<bool>,
        all_paths: &mut Vec<Vec<bool>>,
    ) {
        match self {
            Pane::Leaf { .. } => {
                all_paths.push(current_path.clone());
            }
            Pane::Split { first, second, .. } => {
                current_path.push(true);
                first.collect_leaf_paths_impl(current_path, all_paths);
                current_path.pop();

                current_path.push(false);
                second.collect_leaf_paths_impl(current_path, all_paths);
                current_path.pop();
            }
        }
    }

    pub fn collect_leaf_paths(&self) -> Vec<Vec<bool>> {
        let mut all_paths = Vec::new();
        let mut current_path = Vec::new();
        self.collect_leaf_paths_impl(&mut current_path, &mut all_paths);
        all_paths
    }

    pub fn get_node_at_mut(&mut self, path: &[bool]) -> eyre::Result<&mut Pane> {
        impl_get_node_at!(self, path, get_node_at_mut)
    }

    pub fn get_node_at(&self, path: &[bool]) -> eyre::Result<&Pane> {
        impl_get_node_at!(self, path, get_node_at)
    }

    pub fn split_leaf_at(
        &mut self,
        path: &[bool],
        direction: layout::Direction,
    ) -> eyre::Result<Vec<bool>> {
        let pane = self.get_node_at_mut(path)?;
        match pane {
            Pane::Leaf { .. } => {
                *pane = Self::new_split(direction);
                let mut result = path.to_vec();
                result.push(false);
                Ok(result)
            }
            Pane::Split { .. } => Err(eyre::eyre!("Can only split a leaf node")),
        }
    }

    pub fn remove_leaf_at(&mut self, path: &[bool]) -> eyre::Result<Vec<bool>> {
        let to_remove = self.get_node_at(path)?;
        match to_remove {
            Pane::Leaf { .. } => {}
            Pane::Split { .. } => {
                return Err(eyre::eyre!("Not allowed to remove a split node"));
            }
        }

        let mut parent_path = path.to_vec();
        let removing_first_child = match parent_path.pop() {
            Some(boolval) => boolval,
            _ => {
                let root = self.get_node_at_mut(path)?;
                *root = Pane::new_leaf();
                return Ok(vec![]);
            }
        };

        let parent = self.get_node_at_mut(&parent_path)?;

        match parent {
            Pane::Leaf { .. } => Err(eyre::eyre!("Parent does not seem to be a split node!")),
            Pane::Split { first, second, .. } => {
                let sibling = if removing_first_child {
                    std::mem::take(second)
                } else {
                    std::mem::take(first)
                };
                *parent = *sibling;
                match parent {
                    Pane::Leaf { .. } => Ok(parent_path),
                    Pane::Split { .. } => Ok(path.to_vec()),
                }
            }
        }
    }

    pub fn cycle(&self, path: &[bool], direction: AdjustDirection) -> eyre::Result<Vec<bool>> {
        let leaf_paths = self.collect_leaf_paths();
        let current_pos = leaf_paths
            .iter()
            .position(|x| x == path)
            .ok_or_eyre("Could not find current path in leaf paths")?;
        let cycled_index = cycle_index(current_pos, leaf_paths.len(), direction);
        let cycled_path = leaf_paths
            .get(cycled_index)
            .ok_or_eyre("Could not find cycled path")?;
        Ok(cycled_path.clone())
    }

    pub fn resize_leaf_at(
        &mut self,
        path: &[bool],
        direction: layout::Direction,
        first_pane_delta: i8,
    ) -> eyre::Result<()> {
        if path.is_empty() {
            return Ok(());
        }
        let (parent_path, _) = match path.split_at_checked(path.len().saturating_sub(1)) {
            Some(val) => val,
            _ => {
                return Ok(());
            }
        };
        let parent_node = self.get_node_at_mut(parent_path)?;
        if let Pane::Split {
            direction: splitdir,
            pct,
            ..
        } = parent_node
        {
            if *splitdir == direction {
                *pct = (*pct as i8 + first_pane_delta).clamp(5, 95) as u8;
            } else {
                self.resize_leaf_at(parent_path, direction, first_pane_delta)?;
            }
        }
        Ok(())
    }

    pub fn cycle_image(
        &mut self,
        path: &[bool],
        size: usize,
        direction: AdjustDirection,
    ) -> eyre::Result<()> {
        let node = self.get_node_at_mut(path)?;

        match node {
            Pane::Split { .. } => Err(eyre::eyre!("Cannot change image on a split node")),
            Pane::Leaf { image_id } => {
                *image_id = crate::cycle_index(*image_id, size, direction);
                Ok(())
            }
        }
    }

    pub fn navigate(
        &self,
        path: &[bool],
        layout_direction: layout::Direction,
        adjust_direction: AdjustDirection,
    ) -> eyre::Result<Vec<bool>> {
        let leaf_paths = self.collect_leaf_paths();

        let leaf_index = leaf_paths
            .iter()
            .position(|x| x == path)
            .ok_or_eyre("Could not find current path in leaf paths")?;

        let (prev, thisplusnext) = leaf_paths.split_at(leaf_index);
        let (_, next) = thisplusnext.split_at(1);
        let previter = prev.iter().rev().collect::<Vec<_>>();
        let nextiter = next.iter().collect::<Vec<_>>();

        let search_array = match adjust_direction {
            AdjustDirection::Next => nextiter,
            AdjustDirection::Previous => previter,
        };

        for candidate in search_array {
            let mut common_parent = Vec::new();
            for (a, b) in path.iter().zip(candidate.iter()) {
                if a == b {
                    common_parent.push(*a);
                } else {
                    break;
                }
            }
            let common_parent_node = self.get_node_at(&common_parent)?;
            if let Pane::Split { direction, .. } = common_parent_node
                && *direction == layout_direction
            {
                return Ok(candidate.clone());
            }
        }

        Ok(path.to_vec())
    }
}
