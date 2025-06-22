use crate::parsers::parse_file;
use crate::prompt::prompt_bool;
use formatx::formatx;
use std::fs::read_dir;
use std::path::Path;

pub struct Sorter<'a> {
    sort_pattern: &'a str,
}

impl<'a> Sorter<'a> {
    pub fn new(sort_pattern: &'a str) -> Self {
        Sorter { sort_pattern }
    }

    pub fn sort_recursively(&self, folder: &Path, library_root_folder: &Path) {
        if folder.is_dir() {
            if let Ok(dir_entries) = read_dir(folder) {
                for entry in dir_entries.flatten() {
                    self.sort_recursively(&entry.path(), library_root_folder);
                }
            }
        } else if let Some(ext) = folder.extension() {
            if ext == "epub" || ext == "pdf" || ext == "mobi" {
                // We only want to sort epub, pdf and mobi files
                self.sort(folder, library_root_folder);
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
                    if new_file_path != file_path {
                        println!(
                            "Want to move file from {:?} to {:?}",
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
                                    eprintln!(
                                        "Failed to create directory {}: {}",
                                        parent.display(),
                                        e
                                    );
                                    return;
                                }
                            }
                        }
                        // If the new file path is different from the original, we move the file
                        if let Err(e) = std::fs::rename(file_path, &new_file_path) {
                            if std::fs::copy(file_path, new_file_path).is_ok() {
                                if let Err(e) = std::fs::remove_file(file_path) {
                                    eprintln!("Failed to remove original file: {}", e);
                                }
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
                    } else {
                        println!("File {} already has the correct name.", file_path.display());
                    }
                }
            }
        }
    }
}
