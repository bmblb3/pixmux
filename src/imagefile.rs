use std::path;

pub fn collect_basenames(imgdir_paths: &Vec<path::PathBuf>) -> Vec<String> {
    let mut basenames = std::collections::BTreeSet::new();
    let image_extensions = ["jpg", "jpeg", "png", "bmp", "tiff", "webp"];

    for dir_path in imgdir_paths {
        if let Ok(entries) = std::fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file()
                    && let Some(ext) = path.extension()
                    && let Some(ext_str) = ext.to_str()
                    && image_extensions.contains(&ext_str.to_lowercase().as_str())
                    && let Some(basename) = path.file_name()
                    && let Some(basename_str) = basename.to_str()
                {
                    basenames.insert(basename_str.to_string());
                }
            }
        }
    }
    basenames.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_collect_only_image_files() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir_path = temp_dir.path().to_path_buf();
        fs::write(temp_dir_path.join("image1.jpg"), b"").unwrap();
        fs::write(temp_dir_path.join("image2.PNG"), b"").unwrap();
        fs::write(temp_dir_path.join("not_image.txt"), b"").unwrap();

        let imgdir_paths = vec![temp_dir_path];
        let result = collect_basenames(&imgdir_paths);

        assert_eq!(result.len(), 2);
        assert!(result.contains(&"image1.jpg".to_string()));
        assert!(result.contains(&"image2.PNG".to_string()));
        assert!(!result.contains(&"not_an_image.txt".to_string()));
    }

    #[test]
    fn test_dont_collect_deep_images() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir_path = temp_dir.path().to_path_buf();
        fs::create_dir(temp_dir_path.join("subdir")).unwrap();
        fs::write(temp_dir_path.join("subdir").join("image.jpg"), b"").unwrap();

        let imgdir_paths = vec![temp_dir_path];
        let result = collect_basenames(&imgdir_paths);

        assert!(result.is_empty());
    }

    #[test]
    fn test_two_dirs_different_stems_same_ext() {
        let temp_dir1 = tempfile::tempdir().unwrap();
        let temp_dir_path1 = temp_dir1.path().to_path_buf();
        fs::write(temp_dir_path1.join("image1.jpg"), b"").unwrap();

        let temp_dir2 = tempfile::tempdir().unwrap();
        let temp_dir_path2 = temp_dir2.path().to_path_buf();
        fs::write(temp_dir_path2.join("image2.jpg"), b"").unwrap();

        let imgdir_paths = vec![temp_dir_path1, temp_dir_path2];
        let result = collect_basenames(&imgdir_paths);

        assert_eq!(result.len(), 2);
        assert!(result.contains(&"image1.jpg".to_string()));
        assert!(result.contains(&"image2.jpg".to_string()));
    }

    #[test]
    fn test_two_dirs_same_stem_different_ext() {
        let temp_dir1 = tempfile::tempdir().unwrap();
        let temp_dir_path1 = temp_dir1.path().to_path_buf();
        fs::write(temp_dir_path1.join("image1.jpg"), b"").unwrap();

        let temp_dir2 = tempfile::tempdir().unwrap();
        let temp_dir_path2 = temp_dir2.path().to_path_buf();
        fs::write(temp_dir_path2.join("image1.png"), b"").unwrap();

        let imgdir_paths = vec![temp_dir_path1, temp_dir_path2];
        let result = collect_basenames(&imgdir_paths);

        assert_eq!(result.len(), 2);
        assert!(result.contains(&"image1.jpg".to_string()));
        assert!(result.contains(&"image1.png".to_string()));
    }

    #[test]
    fn test_two_dirs_same_stem_same_ext() {
        let temp_dir1 = tempfile::tempdir().unwrap();
        let temp_dir_path1 = temp_dir1.path().to_path_buf();
        fs::write(temp_dir_path1.join("image1.jpg"), b"").unwrap();

        let temp_dir2 = tempfile::tempdir().unwrap();
        let temp_dir_path2 = temp_dir2.path().to_path_buf();
        fs::write(temp_dir_path2.join("image1.jpg"), b"").unwrap();

        let imgdir_paths = vec![temp_dir_path1, temp_dir_path2];
        let result = collect_basenames(&imgdir_paths);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&"image1.jpg".to_string()));
        assert!(result.contains(&"image1.jpg".to_string()));
    }

    #[test]
    fn test_two_dirs_no_images() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir_path = temp_dir.path().to_path_buf();
        fs::write(temp_dir_path.join("not_an_image.txt"), b"").unwrap();

        let imgdir_paths = vec![temp_dir_path];
        let result = collect_basenames(&imgdir_paths);

        assert!(result.is_empty());
    }

    #[test]
    fn test_dont_collect_image_extra_extension() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir_path = temp_dir.path().to_path_buf();
        fs::write(temp_dir_path.join("image.jpg.extra"), b"").unwrap();

        let imgdir_paths = vec![temp_dir_path];
        let result = collect_basenames(&imgdir_paths);

        assert!(result.is_empty());
    }
}
