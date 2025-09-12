pub struct BoolPathOperations;

impl BoolPathOperations {
    pub fn get_nearest_promoted_leaf(all_paths: &[Vec<bool>], remove_path: &[bool]) -> Vec<bool> {
        let extended = vec![None]
            .into_iter()
            .chain(all_paths.iter().map(Some))
            .chain(vec![None])
            .collect::<Vec<Option<&Vec<bool>>>>();

        let (before, after) = extended
            .windows(3)
            .find(|x| x[1].is_some_and(|f| f == remove_path))
            .map(|x| (x[0], x[2]))
            .unwrap();

        let mut parent_path = remove_path.to_vec();
        parent_path.pop();

        let child = if before.is_some_and(|f| f.starts_with(&parent_path)) {
            before.unwrap()
        } else if after.is_some() {
            after.unwrap()
        } else {
            return vec![];
        };

        let mut result = child.clone();
        result.remove(parent_path.len());
        result
    }
}
