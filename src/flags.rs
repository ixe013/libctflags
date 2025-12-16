use std::env;
use std::path::PathBuf;

use crate::seed;

// Environment variable to define to set a global salt
// defaults to no salt
const GLOBAL_SALT_ENV_VAR: &str = "FLAG_GLOBAL_SALT";


pub fn compute_flag_from_context(context: &PathBuf, step: &str, salt: Option<String>) -> String {
    let mut digest = md5::Context::new();

    digest.consume(step);

    digest.consume(seed::get_from_context_or_null(context));

    if let Some(s) = salt {
        digest.consume(s);
    }

    format!("{:x}", digest.finalize())
}

pub fn compute_flag_from_string_context(context: &str, step: &str, salt: Option<String>) -> String {
    let mut digest = md5::Context::new();

    digest.consume(step);

    digest.consume(context);

    if let Some(s) = salt {
        digest.consume(s);
    }

    format!("{:x}", digest.finalize())
}


pub fn compute_flag(step: &str, salt: Option<String>) -> String {
    let mut digest = md5::Context::new();

    digest.consume(step);

    if let Ok(global_salt) = env::var(GLOBAL_SALT_ENV_VAR) {
        digest.consume(global_salt);
    }

    if let Some(app_salt) = salt {
        digest.consume(app_salt);
    }

    digest.consume(seed::get_or_null());

    format!("{:x}", digest.finalize())
}


pub fn format_flag(step: &str, salt: Option<String>) -> String {
    format!("flag({step}).{}", compute_flag(step, salt))
}

pub fn format_flag_from_string_context(context: &str, step: &str, salt: Option<String>) -> String {
    format!("flag({step}).{}", compute_flag_from_string_context(context, step, salt))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_environment_based_flag() {
        assert_eq!(compute_flag("example", None),
            "1a79a4d60de6718e8e5b326e338ae533");
        assert_eq!(compute_flag("example", Some("app noise".to_string())),
            "5251b4290fb05756da94df1ec637b5a7");
    }

    #[test]
    fn same_flag_increasing_noise() {
        // Create a context file for this test (to avoid collision with other tests)
        let context = seed::create_seed_context(".__example1");
        // Save a seed to that context file
        assert!(!seed::set_from_context(&context, "segg1545").is_err());

        assert_eq!(compute_flag_from_context(&context, "example", None),
            "5f1b958992ca66c09c0ac9170fce85de");

        assert_eq!(compute_flag_from_context(&context, "example", Some("app noise".to_string())),
            "98bf92ea5a1438ed465490c9c2396409");

        let _ = seed::clear_from_context(&context);
    }

    #[test]
    fn example_flag() {
        let context = seed::create_seed_context(".__example2");
        assert!(!seed::set_from_context(&context, "segg1545").is_err());

        assert_eq!(compute_flag_from_context(&context, "example", None), "5f1b958992ca66c09c0ac9170fce85de");

        let _ = seed::clear_from_context(&context);
    }

    #[test]
    // Equivalent to existing example_flag test
    fn example_flag_with_string_context() {
        assert_eq!(compute_flag_from_string_context("segg1545", "example", None), "5f1b958992ca66c09c0ac9170fce85de");
    }
}
