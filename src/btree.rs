use color_eyre::eyre::{Ok, Result, bail};

pub struct BTreeSpec<L = (), B = ()> {
    pub leaf_paths: Vec<Vec<bool>>,
    pub leaf_data: Vec<L>,
    pub branch_data: Vec<B>,
}

#[derive(Debug, Clone)]
pub enum BTreeNode<L = (), B = ()> {
    Leaf(L),
    Branch {
        first: Box<Self>,
        second: Box<Self>,
        data: B,
    },
}

impl<L, B> BTreeNode<L, B> {
    pub fn from_spec(spec: &BTreeSpec<L, B>) -> Result<Self>
    where
        L: Clone + Default,
        B: Clone + Default,
    {
        let mut tree = Self::Leaf(L::default());
        for path in &spec.leaf_paths {
            Self::default_from_path(&mut tree, path)?;
        }
        if tree.collect_paths() != spec.leaf_paths {
            bail!("Non-canonical/invalid path spec")
        }

        tree.assign_leaf_data(&spec.leaf_data)?;
        tree.assign_branch_data(&spec.branch_data)?;

        Ok(tree)
    }

    pub fn get_spec(&self) -> BTreeSpec<L, B>
    where
        L: Clone,
        B: Clone,
    {
        BTreeSpec {
            leaf_paths: self.collect_paths(),
            leaf_data: self.collect_leaf_data(),
            branch_data: self.collect_branch_data(),
        }
    }

    pub fn split_leaf_at(&mut self, path: &mut &Vec<bool>, branch_data: B) -> Result<()>
    where
        L: Default + Clone,
        B: Default + Clone,
    {
        let data = self.get_leaf_data_at(path)?.clone();
        let leaf_mut = self.get_leaf_at_mut(path)?;
        *leaf_mut = Self::default_branch();
        leaf_mut.assign_leaf_data(&vec![data.clone(); 2])?;
        leaf_mut.assign_branch_data(&vec![branch_data; 1])?;
        Ok(())
    }

    pub fn remove_leaf_at(&mut self, path: &mut &Vec<bool>) -> Result<()>
    where
        L: Default + Clone,
        B: Default + Clone,
    {
        match path.as_slice() {
            [] => Ok(()),
            [is_this_first] => {
                if let Self::Branch { first, second, .. } = self {
                    let promoted_child = if *is_this_first {
                        second.clone()
                    } else {
                        first.clone()
                    };
                    *self = *promoted_child;
                }
                Ok(())
            }
            [head @ .., is_this_first] => {
                let parent_mut = self.get_branch_at_mut(head)?;
                if let Self::Branch { first, second, .. } = parent_mut {
                    let promoted_child = if *is_this_first {
                        second.clone()
                    } else {
                        first.clone()
                    };
                    *parent_mut = *promoted_child;
                }
                Ok(())
            }
        }
    }
}

impl<L, B> BTreeNode<L, B> {
    fn assign_branch_data(&mut self, new_data: &[B]) -> Result<()>
    where
        B: Clone,
    {
        let mut existing_data_mut = self.collect_branch_data_mut();
        if new_data.len() > existing_data_mut.len() {
            bail!("Remaining unused branch data")
        } else if new_data.len() < existing_data_mut.len() {
            bail!("Prematurely exhausted branch data")
        } else {
            for (src, dst) in new_data.iter().zip(&mut existing_data_mut) {
                **dst = src.clone();
            }
            Ok(())
        }
    }

    fn assign_leaf_data(&mut self, new_data: &[L]) -> Result<()>
    where
        L: Clone,
    {
        let mut existing_data_mut = self.collect_leaf_data_mut();
        if new_data.len() > existing_data_mut.len() {
            bail!("Remaining unused leaf data")
        } else if new_data.len() < existing_data_mut.len() {
            bail!("Prematurely exhausted leaf data")
        } else {
            for (src, dst) in new_data.iter().zip(&mut existing_data_mut) {
                **dst = src.clone();
            }
            Ok(())
        }
    }

    fn collect_branch_data(&self) -> Vec<B>
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

    fn collect_branch_data_mut(&mut self) -> Vec<&mut B> {
        let mut branch_data = Vec::new();
        self.collect_branch_data_mut_impl(&mut branch_data);
        branch_data
    }

    fn collect_branch_data_mut_impl<'a>(&'a mut self, branch_data: &mut Vec<&'a mut B>) {
        if let Self::Branch {
            first,
            second,
            data,
        } = self
        {
            branch_data.push(data);
            first.collect_branch_data_mut_impl(branch_data);
            second.collect_branch_data_mut_impl(branch_data);
        }
    }

    fn collect_leaf_data(&self) -> Vec<L>
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

    fn collect_leaf_data_mut(&mut self) -> Vec<&mut L> {
        let mut leaf_data = Vec::new();
        self.collect_leaf_data_mut_impl(&mut leaf_data);
        leaf_data
    }

    fn collect_leaf_data_mut_impl<'a>(&'a mut self, leaf_data: &mut Vec<&'a mut L>) {
        match self {
            Self::Leaf(data) => leaf_data.push(data),
            Self::Branch { first, second, .. } => {
                first.collect_leaf_data_mut_impl(leaf_data);
                second.collect_leaf_data_mut_impl(leaf_data);
            }
        }
    }

    fn collect_paths(&self) -> Vec<Vec<bool>> {
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

    fn default_from_path(node: &mut Self, path: &[bool]) -> Result<()>
    where
        L: Default,
        B: Default,
    {
        if path.is_empty() {
            if let Self::Branch { .. } = node {
                bail!("Non-canonical/invalid path spec")
            } else {
                return Ok(());
            }
        }

        match node {
            Self::Leaf(_) => {
                *node = Self::default_branch();
                Ok(())
            }
            Self::Branch { first, second, .. } => {
                let child = if path[0] { first } else { second };
                Self::default_from_path(child, &path[1..])
            }
        }
    }

    fn default_leaf() -> Self
    where
        L: Default,
    {
        Self::Leaf(L::default())
    }

    fn get_branch_at_mut(&mut self, path: &[bool]) -> Result<&mut Self> {
        match path {
            [] => {
                if let Self::Branch { .. } = self {
                    Ok(self)
                } else {
                    bail!("Did not end up at a branch!")
                }
            }
            [head, tail @ ..] => {
                if let Self::Branch { first, second, .. } = self {
                    let child = if *head { first } else { second };
                    child.get_branch_at_mut(tail)
                } else {
                    bail!("Reached a leaf before end of path")
                }
            }
        }
    }

    fn get_leaf_at_mut(&mut self, path: &[bool]) -> Result<&mut Self> {
        match path {
            [] => match self {
                Self::Leaf(_) => Ok(self),
                _ => bail!("Could not find leaf at specified path"),
            },
            [head, tail @ ..] => match self {
                Self::Branch { first, second, .. } => {
                    let child = if *head { first } else { second };
                    child.get_leaf_at_mut(tail)
                }
                _ => bail!("Could not find leaf at specified path"),
            },
        }
    }

    fn get_leaf_data_at(&self, path: &[bool]) -> Result<&L> {
        match (self, path) {
            (Self::Leaf(data), []) => Ok(data),
            (Self::Branch { first, second, .. }, [head, tail @ ..]) => {
                let child = if *head { first } else { second };
                child.get_leaf_data_at(tail)
            }
            _ => bail!("Could not find leaf at specified path"),
        }
    }
}
