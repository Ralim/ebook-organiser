use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::process;

mod config;
mod parsers;
mod prompt;
mod sorter;

/// Ebook Organiser - A tool to automatically organise your ebook collection
///
/// This application allows you to sort and organise ebook files (epub, pdf, mobi)
/// by extracting metadata and moving them to a structured library folder.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the config file
    ///
    /// If not specified, will use the default location at:
    /// - Linux/macOS: ~/.config/ebook-organiser.toml
    /// - Windows: %APPDATA%\ebook-organiser.toml
    #[arg(short, long, value_name = "FILE", global = true)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Sort and organise ebook files (default command)
    ///
    /// Reads ebook files from the source directory and organises them
    /// in the library directory according to the format template.
    #[clap(visible_alias = "organise")]
    Sort {
        /// Path to source directory containing ebooks to organise
        ///
        /// This overrides the source path from the config file.
        /// If neither this argument nor a config file source path is specified,
        /// defaults to "./input"
        #[arg(value_name = "SOURCE_PATH")]
        source_path: Option<PathBuf>,

        /// Copy files instead of moving them (overrides config)
        #[arg(long, conflicts_with = "move")]
        copy: bool,

        /// Move files instead of copying them (overrides config)
        #[arg(long, conflicts_with = "copy")]
        r#move: bool,
    },

    /// Save the default configuration file
    ///
    /// Creates a default configuration file at the system's default config location
    /// that you can then edit according to your preferences. The path where the file
    /// was saved will be displayed.
    SaveConfig,
}

/// Get the default configuration file path based on the operating system
fn get_default_config_path() -> PathBuf {
    dirs::config_dir().unwrap().join("ebook-organiser.toml")
}

/// Run the sort operation with the specified config and source path
fn run_sort(config_path: Option<PathBuf>, source_path: Option<PathBuf>, copy_flag: Option<bool>) {
    // Load configuration from specified path or default path
    let config_path = config_path.unwrap_or_else(get_default_config_path);

    let mut config = match config::Config::load(&config_path) {
        Ok(config) => {
            println!("Configuration loaded from {}", config_path.display());
            config
        }
        Err(e) => {
            println!(
                "Notice: Failed to load configuration from {}: {}",
                config_path.display(),
                e
            );
            println!("Using default configuration values instead.");
            config::Config::default()
        }
    };

    // Override config copy value if a command line flag was provided
    if let Some(copy_value) = copy_flag {
        if config.input_path == config.library_path && copy_value {
            eprintln!(
                "Error: Copying files is not recommended when the input path is the same as the library path."
            );
            return;
        }
        config.copy = copy_value;
    }

    // Use provided source path or fall back to config
    let source_path = match source_path {
        Some(path) => path,
        None => PathBuf::from(&config.input_path),
    };

    let library_path = Path::new(&config.library_path);
    let audio_book_library_path = Path::new(&config.audiobook_library_path);
    let sorter = sorter::Sorter::new(&config.format_template, config.copy);

    println!(
        "Starting organisation process: sorting ebooks from {} into {}",
        source_path.display(),
        library_path.display()
    );
    println!("Using format template: {}", &config.format_template);
    println!(
        "Copy mode: {}",
        if config.copy { "enabled" } else { "disabled" }
    );

    sorter.sort_recursively(&source_path, library_path, audio_book_library_path);

    println!("Organisation complete!");
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::SaveConfig) => {
            // Create and save the default configuration
            let config = config::Config::default();
            let config_path = get_default_config_path();

            if let Err(e) = config.save(&config_path) {
                eprintln!("Error: Failed to save configuration: {e}");
                process::exit(1);
            }

            println!("Success: Default config saved to {}", config_path.display());
            println!("You can now edit this file to customize your ebook organisation.");
        }
        Some(Commands::Sort {
            source_path,
            copy,
            r#move,
        }) => {
            // Determine whether to override the copy flag from config
            let copy_flag = if *copy {
                Some(true)
            } else if *r#move {
                Some(false)
            } else {
                None
            };

            run_sort(cli.config.clone(), source_path.clone(), copy_flag);
        }
        None => {
            // If no command is specified, default to Sort with no source path
            run_sort(cli.config, None, None);
        }
    }
}
