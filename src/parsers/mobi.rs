use std::path::Path;

use mobi::Mobi;

use crate::{parsers::metadata::FileMetadata, prompt::prompt};

pub fn parse_mobi(file_path: &Path) -> Result<FileMetadata, String> {
    // Open the MOBI file and parse its metadata
    match Mobi::from_path(file_path) {
        Ok(mobi) => {
            // Extract title from mobi metadata
            let title = mobi.title();
            let author = mobi.author();
            let contributors = mobi.contributor();
            let selected_author = if let Some(author) = author {
                author
            } else if let Some(contributor) = contributors {
                contributor
            } else {
                prompt(&format!(
                    "No author found for {}, please enter one:",
                    file_path.display()
                ))
            };

            Ok(FileMetadata {
                title: title.trim().to_owned(),
                main_author: selected_author.trim().to_owned(),
            })
        }
        Err(e) => {
            println!("Failed to parse mobi file: {:?}", e);
            Err(e.to_string())
        }
    }
}
