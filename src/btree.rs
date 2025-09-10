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

    fn default_from_path(node: &mut Self, path: &[bool]) -> eyre::Result<()>
    where
        L: Clone + Default,
        B: Clone + Default,
    {
        match node {
            Self::Leaf(_) if path.is_empty() => {}
            Self::Leaf(_) => {
                *node = Self::default_branch();
            }
            Self::Branch { .. } if path.is_empty() => {
                return Err(eyre::eyre!("Non-canonical/invalid path spec"));
            }
            Self::Branch { first, second, .. } => {
                let child = if path[0] { first } else { second };
                Self::default_from_path(child, &path[1..])?
            }
        };
        Ok(())
    }

    fn assign_leaf_data(&mut self, data_iter: &mut Iter<L>) -> eyre::Result<()>
    where
        L: Clone,
    {
        match self {
            Self::Leaf(data) => {
                let new_data = data_iter
                    .next()
                    .ok_or_eyre("Prematurely exhausted leaf data")?;
                *data = new_data.clone();
            }
            Self::Branch { first, second, .. } => {
                first.assign_leaf_data(data_iter)?;
                second.assign_leaf_data(data_iter)?;
            }
        }
        Ok(())
    }

    fn assign_branch_data(&mut self, data_iter: &mut Iter<B>) -> eyre::Result<()>
    where
        B: Clone,
    {
        if let Self::Branch {
            first,
            second,
            data,
        } = self
        {
            let new_data = data_iter
                .next()
                .ok_or_eyre("Prematurely exhausted branch data")?;
            *data = new_data.clone();
            first.assign_branch_data(data_iter)?;
            second.assign_branch_data(data_iter)?;
        }
        Ok(())
    }

    pub fn from_spec(spec: &BTreeSpec<L, B>) -> eyre::Result<Self>
    where
        L: Clone + Default,
        B: Clone + Default,
    {
        let mut tree = Self::Leaf(L::default());
        for path in &spec.leaf_paths {
            Self::default_from_path(&mut tree, path)?;
        }
        if tree.collect_paths() != spec.leaf_paths {
            return Err(eyre::eyre!("Non-canonical/invalid path spec"));
        }

        let mut leaf_data_iter = spec.leaf_data.iter();
        tree.assign_leaf_data(&mut leaf_data_iter)?;
        if leaf_data_iter.next().is_some() {
            return Err(eyre::eyre!("Remaining unused leaf data"));
        }

        let mut branch_data_iter = spec.branch_data.iter();
        tree.assign_branch_data(&mut branch_data_iter)?;
        if branch_data_iter.next().is_some() {
            return Err(eyre::eyre!("Remaining unused branch data"));
        }

        Ok(tree)
    }

    pub fn get_leaf_at(&self, path: &[bool]) -> eyre::Result<&Self> {
        match (self, path) {
            (Self::Leaf(_), []) => Ok(self),
            (Self::Branch { first, second, .. }, [head, tail @ ..]) => {
                let child = if *head { first } else { second };
                child.get_leaf_at(tail)
            }
            _ => Err(eyre::eyre!("Could not find leaf at specified path")),
        }
    }

    fn get_leaf_data_at(&self, path: &[bool]) -> eyre::Result<&L> {
        match (self, path) {
            (Self::Leaf(data), []) => Ok(data),
            (Self::Branch { first, second, .. }, [head, tail @ ..]) => {
                let child = if *head { first } else { second };
                child.get_leaf_data_at(tail)
            }
            _ => Err(eyre::eyre!("Could not find leaf at specified path")),
        }
    }

    pub fn get_leaf_at_mut(&mut self, path: &[bool]) -> eyre::Result<&mut Self> {
        match path {
            [] => match self {
                Self::Leaf(_) => Ok(self),
                _ => Err(eyre::eyre!("Could not find leaf at specified path")),
            },
            [head, tail @ ..] => match self {
                Self::Branch { first, second, .. } => {
                    let child = if *head { first } else { second };
                    child.get_leaf_at_mut(tail)
                }
                _ => Err(eyre::eyre!("Could not find leaf at specified path")),
            },
        }
    }

    pub fn collect_paths(&self) -> Vec<Vec<bool>> {
        let mut all_paths = Vec::new();
        self.collect_paths_impl(&mut Vec::new(), &mut all_paths);
        all_paths
    }

    fn collect_paths_impl(&self, current_path: &mut Vec<bool>, all_paths: &mut Vec<Vec<bool>>) {
        match self {
            Self::Leaf(_) => all_paths.push(current_path.to_vec()),
            Self::Branch { first, second, .. } => {
                for (child, is_first) in [(first, true), (second, false)] {
                    current_path.push(is_first);
                    child.collect_paths_impl(current_path, all_paths);
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
        self.collect_leaf_data_impl(&mut leaf_data);
        leaf_data
    }

    fn collect_leaf_data_impl(&self, leaf_data: &mut Vec<L>)
    where
        L: Clone,
    {
        match self {
            Self::Leaf(data) => leaf_data.push(data.clone()),
            Self::Branch { first, second, .. } => {
                first.collect_leaf_data_impl(leaf_data);
                second.collect_leaf_data_impl(leaf_data);
            }
        }
    }

    pub fn collect_branch_data(&self) -> Vec<B>
    where
        B: Clone,
    {
        let mut branch_data = Vec::new();
        self.collect_branch_data_impl(&mut branch_data);
        branch_data
    }

    fn collect_branch_data_impl(&self, branch_data: &mut Vec<B>)
    where
        B: Clone,
    {
        match self {
            Self::Leaf(_) => {}
            Self::Branch {
                first,
                second,
                data,
            } => {
                branch_data.push(data.clone());
                first.collect_branch_data_impl(branch_data);
                second.collect_branch_data_impl(branch_data);
            }
        }
    }

    pub fn split_leaf_at(&mut self, path: &mut &Vec<bool>, branch_data: B) -> eyre::Result<()>
    where
        L: Default + Clone,
        B: Default + Clone,
    {
        let data = self.get_leaf_data_at(path)?.clone();
        let leaf_mut = self.get_leaf_at_mut(path)?;
        *leaf_mut = Self::default_branch();
        leaf_mut.assign_leaf_data(&mut vec![data.clone(); 2].iter())?;
        leaf_mut.assign_branch_data(&mut vec![branch_data; 1].iter())?;
        Ok(())
    }

    pub fn remove_leaf_at(&mut self, _path: &mut &Vec<bool>) -> eyre::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests;
