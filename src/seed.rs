use std::env;
use std::fs;
use std::io;
use std::io::Error;
use std::path::PathBuf;
use dirs;
use tracing::{info, error};

use once_cell::sync::Lazy;

// The default seed file name if none is provided
const DEFAULT_SEED_FILE_NAME: &str = ".tp2.seed";
// Environment variable will override the default seed file name if set
pub const FILE_NAME_ENVIRONMENT_VAR: &str = "TP2_SEED_FILE_NAME";

// Default global context for legacy code
static DEFAULT_CONTEXT: Lazy<PathBuf> = Lazy::new(|| {
    seed_file_name()
});


// Returns the default file name as a path in the user's home directory
fn default_file_name() -> PathBuf {
    let mut buf = match dirs::home_dir() {
        Some(path) => path,
        _ => PathBuf::from("./"),
    };

    buf.push(DEFAULT_SEED_FILE_NAME);

    buf
}

fn seed_file_name() -> PathBuf {
   let env_name = env::var(FILE_NAME_ENVIRONMENT_VAR);

   match env_name {
       Ok(ref env_name) => PathBuf::from(env_name),
       _ => default_file_name(),
   }
}

pub fn create_seed_context(name: &str) -> PathBuf {
    PathBuf::from(name)
}


pub fn get_from_context(context: &PathBuf) -> Result<String, io::Error> {
    fs::read_to_string(context.as_path())
}

pub fn get_from_context_or_null(context: &PathBuf) -> String {
    match fs::read_to_string(context.as_path()) {
        Ok(s) => s,
        _ => String::new(),
    }
}


pub fn set_from_context(context: &PathBuf, seed: &str) -> Result<(), Error> {
    info!("Writing {} to {}", seed, context.display());
    fs::write(context.as_path(), seed)?;
    Ok(())
}


pub fn clear_from_context(context: &PathBuf) {
    match fs::remove_file(context.as_path()) {
        Err(result) => error!("Unable to clear the seed: {}", result.to_string()),
        _ => {},
    }
}

pub fn get() -> Result<String, io::Error> {
    get_from_context(&DEFAULT_CONTEXT)
}

pub fn get_or_null() -> String {
    get_from_context_or_null(&DEFAULT_CONTEXT)
}


pub fn set(seed: &str) -> Result<(), Error> {
    set_from_context(&DEFAULT_CONTEXT, seed)
}


pub fn clear() {
    clear_from_context(&DEFAULT_CONTEXT)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test run in parallel, each needs its own variable
    #[test]
    fn fails_if_not_set() {
        let context = create_seed_context(".__fails_if_not_set");
        clear_from_context(&context); // In case the last test crashed and left a file on disk
        assert!(get_from_context(&context).is_err());
        let _ = clear_from_context(&context);
    }

    #[test]
    fn seed_round_trip() {
        let context = create_seed_context(".__seed_round_trip");
        clear_from_context(&context); // In case the last test crashed and left a file on disk
        assert!(!set_from_context(&context, "example").is_err());
        assert_eq!(get_from_context(&context).unwrap(), "example");
        assert!(get_from_context(&context).is_ok());
        let _ = clear_from_context(&context);
    }
}
