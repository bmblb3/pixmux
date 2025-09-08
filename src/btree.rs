use std::slice::Iter;

use color_eyre::eyre::{self, Ok};

pub struct BTreeSpec<L = (), B = ()> {
    pub leaf_paths: Vec<Vec<bool>>,
    pub leaf_data: Vec<L>,
    pub branch_data: Vec<B>,
}

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

    fn default_from_path(node: &mut Self, mut path: &mut [bool])
    where
        L: Clone + Default,
        B: Clone + Default,
    {
        let build_first_child = match path.split_off_first_mut() {
            None => return,
            Some(v) => *v,
        };

        if let Self::Leaf(_) = node {
            *node = Self::default_branch();
        };

        if let Self::Branch { first, second, .. } = node {
            if build_first_child {
                Self::default_from_path(first, path);
            } else {
                Self::default_from_path(second, path);
            }
        }
    }
    fn assign_data(node: &mut Self, leaf_data_iter: &mut Iter<L>, branch_data_iter: &mut Iter<B>)
    where
        L: Clone,
        B: Clone,
    {
        match node {
            Self::Leaf(data) => {
                let newdata = leaf_data_iter.next().unwrap();
                *data = newdata.clone();
            }
            Self::Branch {
                first,
                second,
                data,
            } => {
                let newdata = branch_data_iter.next().unwrap();
                *data = newdata.clone();
                Self::assign_data(first, leaf_data_iter, branch_data_iter);
                Self::assign_data(second, leaf_data_iter, branch_data_iter);
            }
        }
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
            Self::default_from_path(&mut tree, &mut path.clone());
        }

        Self::assign_data(&mut tree, &mut leaf_data.iter(), &mut branch_data.iter());

        Ok(tree)
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
