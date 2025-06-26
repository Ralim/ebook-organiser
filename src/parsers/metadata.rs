use std::path::Path;

use crate::{
    parsers::{audiobook::parse_audiobook, epub::parse_epub, mobi::parse_mobi},
    prompt::prompt,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FileMetadata {
    pub title: String,
    pub main_author: String,
}

pub fn parse_file(file_path: &Path) -> Option<FileMetadata> {
    // If file is an epub, we can parse it
    let ext = file_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or_default();

    let metadata: Option<FileMetadata> = if ext == "epub" {
        parse_epub(file_path).ok()
    } else if ext == "mobi" {
        parse_mobi(file_path).ok()
    } else if ext == "m4a" || ext == "m4b" {
        parse_audiobook(file_path).ok()
    } else {
        None
    };
    if let Some(meta) = metadata {
        if !meta.title.is_empty() && !meta.main_author.is_empty() {
            return Some(meta);
        }
    }
    println!("Failed to parse file metadata for: {file_path:?} ",);
    let title = prompt("Enter title");
    let main_author = prompt("Enter main author");
    if title.is_empty() || main_author.is_empty() {
        println!("Title or main author cannot be empty.");
        return None;
    }
    Some(FileMetadata { title, main_author })
}
