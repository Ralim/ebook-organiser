use std::path::Path;

use mp4ameta::{Data, DataIdent, Tag};

use crate::{
    parsers::metadata::FileMetadata,
    prompt::{prompt, prompt_select_other},
};
fn get_title(meta: &Tag, file_path: &Path) -> String {
    if let Some(title) = meta.title() {
        if !title.is_empty() {
            return title.to_owned();
        }
    }
    let fourcc_sti = mp4ameta::Fourcc([b'@', b's', b't', b'i']);
    let fourcc_alb = mp4ameta::Fourcc([0xA9, b'a', b'l', b'b']);
    // Try for metadata
    for (meta_name, meta_data) in meta.data() {
        match meta_name {
            DataIdent::Fourcc(fourcc) => {
                if fourcc == &fourcc_sti || fourcc == &fourcc_alb {
                    if let Data::Utf8(title) = meta_data {
                        return title.to_owned();
                    }
                }
            }
            DataIdent::Freeform { mean, name } => {
                if mean.is_empty() || name.is_empty() {
                    // Freeform data without mean or name, skip
                    continue;
                }
                if name == "SUBTITLE" || name == "TITLE" {
                    if let Data::Utf8(title) = meta_data {
                        return title.to_owned();
                    }
                }
            }
        }
    }
    // Fall through to asking
    prompt(&format!(
        "No title found for {}, please enter one:",
        file_path.display()
    ))
}

pub fn parse_audiobook(file_path: &Path) -> Result<FileMetadata, String> {
    // Open the m4b file and parse its metadata
    match mp4ameta::Tag::read_from_path(file_path) {
        Ok(file_meta) => {
            // Extract title from mobi metadata
            let title = get_title(&file_meta, file_path);

            let author = file_meta.artist();
            let artists: Vec<&str> = file_meta.artists().collect();
            let composer = file_meta.composer();
            let composers: Vec<&str> = file_meta.composers().collect();
            let artist_options: Vec<String> = artists
                .iter()
                .chain(composers.iter())
                .map(|&s| s.to_owned())
                .collect();

            let selected_author = if let Some(author) = author {
                author.to_owned()
            } else if let Some(author) = composer {
                author.to_owned()
            } else {
                prompt_select_other(
                    &format!(
                        "No author found for {}, please enter one:",
                        file_path.display()
                    ),
                    &artist_options,
                )
            };

            Ok(FileMetadata {
                title: title.trim().to_owned(),
                main_author: selected_author.trim().to_owned(),
            })
        }
        Err(e) => {
            println!("Failed to parse audiobook file: {:?}", e);
            Err(e.to_string())
        }
    }
}
