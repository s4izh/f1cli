use std::env;
use std::fs;
use std::path::Path;

pub mod web;

pub fn ensure_cache_dir(year: Option<i32>) -> Result<String, std::io::Error> {
    let cache_home = match env::var("XDG_CACHE_HOME") {
        Ok(val) => val,
        Err(_) => {
            // If XDG_CACHE_HOME is not set, use the default ~/.cache
            let home_dir = match dirs::home_dir() {
                Some(path) => path,
                None => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Cannot determine the home directory.",
                    ));
                }
            };
            home_dir.join(".cache").to_string_lossy().to_string()
        }
    };

    // Create the cache directory if it doesn't exist
    let cache_dir = Path::new(&cache_home);
    if !cache_dir.exists() {
        if let Err(err) = fs::create_dir_all(cache_dir) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error creating cache directory: {}", err),
            ));
        }
    }

    // Create the 'cli' subdirectory inside the cache directory
    let cli_dir = cache_dir.join("f1rs");
    if !cli_dir.exists() {
        if let Err(err) = fs::create_dir(&cli_dir) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error creating cache directory: {}", err),
            ));
        }
    }

    let cache_year_dir = match year {
        Some(year) => cli_dir.join(year.to_string()),
        None => return Ok(String::from(cli_dir.to_str().unwrap())),
    };

    if !cache_year_dir.exists() {
        if let Err(err) = fs::create_dir(&cache_year_dir) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Error creating cache directory for year {}: {}",
                    year.unwrap(),
                    err
                ),
            ));
        }
    }
    Ok(String::from(cli_dir.to_str().unwrap()))
}
