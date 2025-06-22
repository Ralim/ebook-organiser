use std::{collections::HashSet, path::Path};

use epub::doc::EpubDoc;

use crate::{
    parsers::{metadata::FileMetadata, misc::flip_comma_split},
    prompt::prompt_select_other,
};

pub fn parse_epub(file_path: &Path) -> Result<FileMetadata, String> {
    // This function would contain the logic to parse the EPUB file
    // For now, we will just return Ok to simulate successful parsing
    match EpubDoc::new(file_path) {
        Ok(doc) => {
            // Now we can parse out the metdata

            let title = doc
                .metadata
                .get("title")
                .and_then(|x| x.iter().find(|s| s.to_lowercase() != "unknown"))
                .cloned()
                .unwrap_or_else(|| "".to_string());

            let creators = doc
                .metadata
                .get("creator")
                .cloned()
                .unwrap_or_else(std::vec::Vec::new);
            let file_as = doc
                .metadata
                .get("file-as")
                .cloned()
                .unwrap_or_else(std::vec::Vec::new);

            let main_author = if creators.len() == 1 {
                flip_comma_split(creators[0].clone())
            } else if file_as.len() > 1 {
                // If there are multiple authors, we can use the first one
                // or format them in a specific way
                flip_comma_split(file_as[0].clone())
            } else {
                // Sometimes we get a bunch of names as CSV, or we get a single name with a comma and flipped order.
                // For these cases we can prompt the user to pick

                let options = creators.into_iter().chain(file_as).collect::<Vec<String>>();

                let options_split1: Vec<String> = options
                    .iter()
                    .map(|s| flip_comma_split(s.to_owned()))
                    .collect();
                let options_split2: Vec<String> = options
                    .iter()
                    .map(|s| s.split(',').map(str::trim).collect::<Vec<_>>())
                    .to_owned()
                    .flatten()
                    .map(|x| x.to_owned())
                    .collect();

                // Merge all threee options
                let final_options_set: HashSet<String> = options
                    .into_iter()
                    .chain(options_split2)
                    .chain(options_split1)
                    .map(|x| x.trim().replace("  ", " "))
                    .collect::<HashSet<_>>();
                let mut final_options: Vec<String> = final_options_set.into_iter().collect();
                final_options.sort();
                prompt_select_other(
                    &format!("No main author found for {file_path:?}"),
                    &final_options,
                )
            };

            Ok(FileMetadata {
                title: title.trim().to_owned(),
                main_author: main_author.trim().to_owned(),
            })
        }
        Err(e) => {
            println!("Failed to parse EPUB file: {:?}", e);
            Err(e.to_string())
        }
    }
}
