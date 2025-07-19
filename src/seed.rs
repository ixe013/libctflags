use std::env;
use std::fs;
use std::io;
use std::io::Error;
use std::path::PathBuf;
use dirs;
use tracing::{info, error};

const DEFAULT_SEED_FILE_NAME: &str = ".tp2.seed";
pub const FILE_NAME_ENVIRONMENT_VAR: &str = "TP2_SEED_FILE_NAME";


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


pub fn get() -> Result<String, io::Error> {
    fs::read_to_string(seed_file_name().as_path())
}

pub fn get_or_null() -> String {
    match fs::read_to_string(seed_file_name().as_path()) {
        Ok(s) => s,
        _ => String::new(),
    }
}


pub fn set(seed: &str) -> Result<(), Error> {
    info!("Writing {} to {}", seed, seed_file_name().display());
    fs::write(seed_file_name().as_path(), seed)?;
    Ok(())
}


pub fn clear() {
    match fs::remove_file(seed_file_name().as_path()) {
        Err(result) => error!("Unable to clear the seed: {}", result.to_string()),
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test with an inexiting file
    fn set_test_seed_file_name() {
        env::set_var(FILE_NAME_ENVIRONMENT_VAR, "__missing__.__missing__");
    }

    #[test]
    fn fails_if_not_set() {
        set_test_seed_file_name();
        assert!(get().is_err());
        let _ = clear();
    }

    #[test]
    fn seed_round_trip() {
        set_test_seed_file_name();
        assert!(!set("segg1545").is_err());
        assert_eq!(get().unwrap(), "segg1545");
        assert!(get().is_err());
    }
}
