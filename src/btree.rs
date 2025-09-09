use std::slice::Iter;

use color_eyre::eyre::{self, Ok, OptionExt};

pub struct BTreeSpec<L = (), B = ()> {
    pub leaf_paths: Vec<Vec<bool>>,
    pub leaf_data: Vec<L>,
    pub branch_data: Vec<B>,
}

#[derive(Debug)]
pub enum BTreeNode<L = (), B = ()> {
    Leaf(L),
    Branch {
        first: Box<Self>,
        second: Box<Self>,
        data: B,
    },
}

impl<L, B> BTreeNode<L, B> {
    fn default_leaf() -> Self
    where
        L: Default,
    {
        Self::Leaf(L::default())
    }

    fn default_branch() -> Self
    where
        L: Default,
        B: Default,
    {
        Self::Branch {
            first: Box::new(Self::default_leaf()),
            second: Box::new(Self::default_leaf()),
            data: B::default(),
        }
    }

    fn default_from_path(node: &mut Self, path: &[bool])
    where
        L: Clone + Default,
        B: Clone + Default,
    {
        match node {
            Self::Leaf(_) if path.is_empty() => {}
            Self::Leaf(_) => {
                *node = Self::default_branch();
            }
            Self::Branch { first, second, .. } => {
                let child = if path[0] { first } else { second };
                Self::default_from_path(child, &path[1..])
            }
        }
    }

    fn assign_leaf_data(node: &mut Self, data_iter: &mut Iter<L>) -> eyre::Result<()>
    where
        L: Clone,
    {
        match node {
            Self::Leaf(data) => {
                let newdata = data_iter
                    .next()
                    .ok_or_eyre("Prematurely exhausted leaf data")?;
                *data = newdata.clone();
            }
            Self::Branch { first, second, .. } => {
                Self::assign_leaf_data(first, data_iter)?;
                Self::assign_leaf_data(second, data_iter)?;
            }
        }
        Ok(())
    }

    fn assign_branch_data(node: &mut Self, data_iter: &mut Iter<B>) -> eyre::Result<()>
    where
        B: Clone,
    {
        if let Self::Branch {
            first,
            second,
            data,
        } = node
        {
            let newdata = data_iter
                .next()
                .ok_or_eyre("Prematurely exhausted branch data")?;
            *data = newdata.clone();
            Self::assign_branch_data(first, data_iter)?;
            Self::assign_branch_data(second, data_iter)?;
        }
        Ok(())
    }

    pub fn from_spec(spec: &BTreeSpec<L, B>) -> eyre::Result<Self>
    where
        L: Clone + Default,
        B: Clone + Default,
    {
        let BTreeSpec {
            leaf_paths,
            leaf_data,
            branch_data,
        } = spec;

        let mut tree = Self::Leaf(L::default());
        for path in leaf_paths {
            Self::default_from_path(&mut tree, path);
        }
        if tree.collect_paths() != *leaf_paths {
            return Err(eyre::eyre!("Invalid path spec"));
        }

        let mut leaf_data_iter = leaf_data.iter();
        Self::assign_leaf_data(&mut tree, &mut leaf_data_iter)?;

        let mut branch_data_iter = branch_data.iter();
        Self::assign_branch_data(&mut tree, &mut branch_data_iter)?;

        if leaf_data_iter.next().is_some() {
            return Err(eyre::eyre!("Remaining unused leaf data"));
        }
        if branch_data_iter.next().is_some() {
            return Err(eyre::eyre!("Remaining unused branch data"));
        }

        Ok(tree)
    }

    fn get_leaf_at_impl<'a>(node: &'a Self, path: &[bool]) -> eyre::Result<&'a Self> {
        match node {
            Self::Leaf(_) if path.is_empty() => Ok(node),
            Self::Branch { first, second, .. } if !path.is_empty() => {
                let child = if path[0] { first } else { second };
                Self::get_leaf_at_impl(child, &path[1..])
            }
            _ => Err(eyre::eyre!("Could not find leaf at specified path")),
        }
    }

    pub fn get_leaf_at(&self, path: &[bool]) -> eyre::Result<&Self> {
        Self::get_leaf_at_impl(self, path)
    }

    pub fn collect_paths(&self) -> Vec<Vec<bool>> {
        let mut all_paths = Vec::new();
        Self::collect_paths_impl(self, &mut Vec::new(), &mut all_paths);
        all_paths
    }

    fn collect_paths_impl(
        node: &Self,
        current_path: &mut Vec<bool>,
        all_paths: &mut Vec<Vec<bool>>,
    ) {
        match node {
            Self::Leaf(_) => all_paths.push(current_path.to_vec()),
            Self::Branch { first, second, .. } => {
                for (child, bool) in [(first, true), (second, false)] {
                    current_path.push(bool);
                    Self::collect_paths_impl(child, current_path, all_paths);
                    current_path.pop();
                }
            }
        }
    }

    pub fn collect_leaf_data(&self) -> Vec<L>
    where
        L: Clone,
    {
        let mut leaf_data = Vec::new();
        Self::collect_leaf_data_impl(self, &mut leaf_data);
        leaf_data
    }

    fn collect_leaf_data_impl(node: &Self, leaf_data: &mut Vec<L>)
    where
        L: Clone,
    {
        match node {
            Self::Leaf(data) => leaf_data.push(data.clone()),
            Self::Branch { first, second, .. } => {
                Self::collect_leaf_data_impl(first, leaf_data);
                Self::collect_leaf_data_impl(second, leaf_data);
            }
        }
    }

    pub fn collect_branch_data(&self) -> Vec<B>
    where
        B: Clone,
    {
        let mut branch_data = Vec::new();
        Self::collect_branch_data_impl(self, &mut branch_data);
        branch_data
    }

    fn collect_branch_data_impl(node: &Self, branch_data: &mut Vec<B>)
    where
        B: Clone,
    {
        match node {
            Self::Leaf(_) => {}
            Self::Branch {
                first,
                second,
                data,
            } => {
                branch_data.push(data.clone());
                Self::collect_branch_data_impl(first, branch_data);
                Self::collect_branch_data_impl(second, branch_data);
            }
        }
    }
}

#[cfg(test)]
mod tests;
