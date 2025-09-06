pub enum BTreeNode<L = ()> {
    Leaf(L),
}
impl BTreeNode {
    pub fn get_paths(&self) -> Vec<Vec<bool>> {
        let paths = vec![vec![]];
        paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btree_with_leaf_at_root_returns_computed_paths() {
        let tree = BTreeNode::Leaf(());

        let paths = tree.get_paths();

        assert_eq!(paths, [[]]);
    }
}
