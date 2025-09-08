use color_eyre::eyre::{self, Ok};

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
        node: &mut BTreeNode<L, B>,
        mut path: &mut [bool],
        leaf_data: &mut [L],
        branch_data: &mut [B],
    ) -> eyre::Result<()>
    where
        L: Clone,
        B: Clone,
    {
        if !path.is_empty()
            && let BTreeNode::Leaf(_) = node
        {
            let current = *path.split_off_first_mut().unwrap();
            let mut first = BTreeNode::Leaf(leaf_data[0].clone());
            let mut second = BTreeNode::Leaf(leaf_data[0].clone());
            if current {
                Self::build_path(&mut first, path, leaf_data, branch_data)?;
            } else {
                Self::build_path(&mut second, path, leaf_data, branch_data)?;
            }
            *node = BTreeNode::Branch {
                first: Box::new(first),
                second: Box::new(second),
                data: branch_data[0].clone(),
            };
        };
        Ok(())
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

        let mut tree = Self::Leaf(leaf_data.first().unwrap().clone());
        println!("Added leaf at tree");
        for path in leaf_paths {
            Self::build_path(
                &mut tree,
                &mut path.clone(),
                &mut leaf_data.clone(),
                &mut branch_data.clone(),
            )?;
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
