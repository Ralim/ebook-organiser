use crate::parsers::parse_file;
use crate::prompt::prompt_bool;
use formatx::formatx;
use std::fs::read_dir;
use std::path::Path;

pub struct Sorter<'a> {
    sort_pattern: &'a str,
    copy: bool,
}

impl<'a> Sorter<'a> {
    pub fn new(sort_pattern: &'a str, copy: bool) -> Self {
        Sorter { sort_pattern, copy }
    }

    pub fn sort_recursively(
        &self,
        folder: &Path,
        library_root_folder: &Path,
        audiobook_root_folder: &Path,
    ) {
        if folder.is_dir() {
            if let Ok(dir_entries) = read_dir(folder) {
                for entry in dir_entries.flatten() {
                    self.sort_recursively(
                        &entry.path(),
                        library_root_folder,
                        audiobook_root_folder,
                    );
                }
            }
        } else if let Some(ext) = folder.extension() {
            if ext == "epub" || ext == "mobi" || ext == "m4b" || ext == "m4a" {
                // Different base folder for audiobooks and regular books
                let base_folder = if ext == "m4b" || ext == "m4a" {
                    audiobook_root_folder
                } else {
                    library_root_folder
                };
                self.sort(folder, base_folder);
            }
        }
    }
    fn act_on_file(&self, file_path: &Path, new_file_path: &Path) {
        if new_file_path == file_path {
            println!("File {} already has the correct name.", file_path.display());
            return;
        }
        let action = if self.copy { "copy" } else { "move" };
        println!(
            "Want to {action} file from {:?} to {:?}",
            file_path, new_file_path
        );
        // Ask user to move
        if !prompt_bool("OK?") {
            return;
        }
        // Need to make folder path if it doesn't exist
        if let Some(parent) = new_file_path.parent() {
            if !parent.exists() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    eprintln!("Failed to create directory {}: {}", parent.display(), e);
                    return;
                }
            }
        }

        // Decide whether to copy or move the file based on the copy flag
        if self.copy {
            // Copy mode - copy file and leave original intact
            if std::fs::copy(file_path, &new_file_path).is_ok() {
                println!(
                    "Copied {} to {}",
                    file_path.display(),
                    new_file_path.display()
                );
            } else {
                eprintln!("Failed to copy file {}", file_path.display());
            }
        } else {
            // Move/rename mode (default behavior)
            if let Err(e) = std::fs::rename(file_path, &new_file_path) {
                // If rename fails, try copy + delete as fallback
                if std::fs::copy(file_path, &new_file_path).is_ok() {
                    if let Err(e) = std::fs::remove_file(file_path) {
                        eprintln!("Failed to remove original file: {}", e);
                    }
                    println!(
                        "Moved (via copy) {} to {}",
                        file_path.display(),
                        new_file_path.display()
                    );
                } else {
                    eprintln!("Failed to copy file: {}", e);
                }
            } else {
                println!(
                    "Moved {} to {}",
                    file_path.display(),
                    new_file_path.display()
                );
            }
        }
    }
    pub fn sort(&self, file_path: &Path, library_root_folder: &Path) {
        // Given a file path, we parse the file metadata, generate a new file name based on the sort pattern,
        // and then move it to that path if its different from the original path.

        if let Some(file_metadata) = parse_file(file_path) {
            //We have file metadata, so we can generate a new file name
            if let Some(ext) = file_path.extension() {
                if let Ok(new_file_name) = formatx!(
                    self.sort_pattern,
                    title = file_metadata.title,
                    author = file_metadata.main_author,
                    ext = ext.to_string_lossy(),
                ) {
                    let new_file_path = library_root_folder.join(new_file_name);
                    self.act_on_file(file_path, &new_file_path);
                }
            }
        }
    }
}
