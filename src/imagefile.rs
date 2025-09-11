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
