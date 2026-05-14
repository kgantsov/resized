use std::path::PathBuf;
use walkdir::WalkDir;

const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff", "tif"];

pub fn collect_image_paths(dir: &PathBuf) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for entry in WalkDir::new(dir) {
        if let Ok(entry) = entry {
            if entry.file_type().is_file() {
                let is_image = entry
                    .path()
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|e| IMAGE_EXTENSIONS.contains(&e.to_lowercase().as_str()))
                    .unwrap_or(false);
                if is_image {
                    paths.push(entry.path().to_path_buf());
                }
            }
        }
    }
    paths
}
