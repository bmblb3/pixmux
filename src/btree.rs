use color_eyre::eyre::{self, Ok, OptionExt};

pub struct BTreeSpec<L = (), B = ()> {
    pub leaf_paths: Vec<Vec<bool>>,
    pub leaf_data: Vec<L>,
    pub branch_data: Vec<B>,
}

pub enum BTreeNode<L = (), B = ()> {
    Leaf(L),
    Branch {
        first: Box<BTreeNode<L, B>>,
        second: Box<BTreeNode<L, B>>,
        data: B,
    },
}

impl<L, B> BTreeNode<L, B> {
    fn build_path(
        _node: &mut BTreeNode<L, B>,
        path: &[bool],
        leaf_data: &[L],
        branch_data: &[B],
    ) -> eyre::Result<()>
    where
        L: Clone,
        B: Clone,
    {
        match path.split_first() {
            None => {
                let (this_leaf_data, _rest_leaf_data) = match leaf_data.split_first() {
                    Some(v) => v,
                    None => return Err(eyre::eyre!("Prematurely exhausted leaf_data!")),
                };
                *_node = Self::Leaf(this_leaf_data.clone());
                Ok(())
            }
            Some((_first, _rest)) => {
                let (this_branch_data, _rest_branch_data) = match branch_data.split_first() {
                    Some(v) => v,
                    None => return Err(eyre::eyre!("Prematurely exhausted branch_data!")),
                };
                *_node = Self::Branch {
                    first: Box::new(Self::Leaf(
                        leaf_data
                            .first()
                            .ok_or_eyre("Prematurely exhausted leaf data!")?
                            .clone(),
                    )),
                    second: Box::new(Self::Leaf(
                        leaf_data
                            .first()
                            .ok_or_eyre("Prematurely exhausted leaf data!")?
                            .clone(),
                    )),
                    data: this_branch_data.clone(),
                };
                Ok(())
            }
        }
    }

    pub fn from_spec(spec: &BTreeSpec<L, B>) -> eyre::Result<Self>
    where
        L: Clone,
        B: Clone,
    {
        let BTreeSpec {
            leaf_paths,
            leaf_data,
            branch_data,
        } = spec;

        let data0 = leaf_data.first().ok_or_eyre("Leaf data is empty!")?;
        let mut tree = Self::Leaf(data0.clone());
        for path in leaf_paths {
            Self::build_path(&mut tree, path, leaf_data, branch_data)?;
        }

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
}

#[cfg(test)]
mod tests;
