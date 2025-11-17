use std::env;
use std::path::PathBuf;

use crate::seed;

// Environment variable to define to set a global salt
// defaults to no salt
const GLOBAL_SALT_ENV_VAR: &str = "FLAG_GLOBAL_SALT";


pub fn compute_flag_from_context(context: &PathBuf, step: &str, salt: Option<String>) -> String {
    let mut digest = md5::Context::new();

    digest.consume(step);

    match env::var(GLOBAL_SALT_ENV_VAR) {
        Ok(s) => digest.consume(s),
        _ => (),
    }

    match salt {
        Some(s) => digest.consume(s),
        None => digest.consume(seed::get_from_context_or_null(context)),
    }
    format!("{:x}", digest.finalize())
}

pub fn compute_flag(step: &str, salt: Option<String>) -> String {
    let mut digest = md5::Context::new();

    digest.consume(step);

    match env::var(GLOBAL_SALT_ENV_VAR) {
        Ok(s) => digest.consume(s),
        _ => (),
    }

    match salt {
        Some(s) => digest.consume(s),
        None => digest.consume(seed::get_or_null()),
    }
    format!("{:x}", digest.finalize())
}


pub fn format_flag(step: &str, salt: Option<String>) -> String {
    format!("flag({step}).{}", compute_flag(step, salt))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_flag() {
        let context = seed::create_seed_context(".__example_flag");
        assert!(!seed::set_from_context(&context, "segg1545").is_err());

        assert_eq!(compute_flag_from_context(&context, "example", None), "5f1b958992ca66c09c0ac9170fce85de");

        let _ = seed::clear_from_context(&context);
    }
}
