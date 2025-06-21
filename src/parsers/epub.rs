use std::path::Path;

use epub::doc::EpubDoc;

use crate::parsers::{metadata::FileMetadata, misc::flip_comma_split};

pub fn parse_epub(file_path: &Path) -> Result<FileMetadata, String> {
    // This function would contain the logic to parse the EPUB file
    // For now, we will just return Ok to simulate successful parsing
    println!("Parsing EPUB file: {:?}", file_path);
    match EpubDoc::new(file_path) {
        Ok(doc) => {
            println!("Successfully parsed EPUB file: {:?}", file_path);
            // Now we can parse out the metdata

            let title = doc
                .metadata
                .get("title")
                .and_then(|titles| titles.first())
                .cloned()
                .unwrap_or_else(|| "".to_string());

            let creators = doc
                .metadata
                .get("creator")
                .cloned()
                .unwrap_or_else(|| vec![]);
            let file_as = doc
                .metadata
                .get("file-as")
                .cloned()
                .unwrap_or_else(|| vec![]);
            let main_author = if creators.len() == 1 {
                flip_comma_split(creators[0].clone())
            } else if file_as.len() > 1 {
                // If there are multiple authors, we can use the first one
                // or format them in a specific way
                flip_comma_split(file_as[0].clone())
            } else {
                // No or many results found; so we are potentially not dertministic.
                println!(
                    "No main author found, using first creator as fallback. {:?} {:?}",
                    creators, file_as
                );
                "".to_string()
            };

            Ok(FileMetadata {
                title: title.trim().to_owned(),
                main_author: main_author.trim().to_owned(),
            })
        }
        Err(e) => {
            println!("Failed to parse EPUB file: {}", e);
            Err(e.to_string())
        }
    }
}
