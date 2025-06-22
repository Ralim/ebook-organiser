use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub library_path: String,
    pub audiobook_library_path: String,
    pub input_path: String,
    pub format_template: String,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            library_path: String::from("/tmp/library"),
            input_path: String::from("/tmp/input"),
            format_template: String::from("{author}/{title}.{ext}"),
            audiobook_library_path: String::from("/tmp/audiobook_library"),
        }
    }
}
impl Config {
    pub fn load(path: &Path) -> io::Result<Self> {
        let contents = fs::read_to_string(path)?;
        let config: Config =
            toml::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(config)
    }

    pub fn save(&self, path: &Path) -> io::Result<()> {
        let serialized = toml::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let path = Path::new(path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = fs::File::create(path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;

    fn get_temp_file_path() -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "ebook_organiser_config_test_{}.toml",
            rand::random::<u64>()
        ));
        path
    }

    #[test]
    fn test_load_config() -> io::Result<()> {
        let config = Config::default();

        let temp_path = get_temp_file_path();

        // Create a TOML config file
        let toml_content = r#"
        library_path = "/tmp/library"
        audiobook_library_path = "/tmp/audiobook_library"
        input_path = "/tmp/input"
        format_template = "{author}/{title}.{ext}"
        "#;

        let mut file = fs::File::create(&temp_path)?;
        file.write_all(toml_content.as_bytes())?;

        // Load the config
        let loaded_config = Config::load(&temp_path)?;

        // Verify the loaded config matches the expected values
        assert_eq!(loaded_config, config);

        // Cleanup
        fs::remove_file(&temp_path)?;

        Ok(())
    }

    #[test]
    fn test_save_and_load_config() -> io::Result<()> {
        let config = Config::default();

        let temp_path = get_temp_file_path();

        // Save the config
        config.save(&temp_path)?;

        // Load the config
        let loaded_config = Config::load(&temp_path)?;

        // Verify the loaded config matches the original
        assert_eq!(loaded_config, config);

        // Cleanup
        fs::remove_file(&temp_path)?;

        Ok(())
    }

    #[test]
    fn test_default_config() {
        let default_config = Config::default();

        assert_eq!(default_config.library_path, "/tmp/library");
        assert_eq!(default_config.input_path, "/tmp/input");
        assert_eq!(default_config.format_template, "{author}/{title}.{ext}");
        assert_eq!(
            default_config.audiobook_library_path,
            "/tmp/audiobook_library"
        );
    }
}
