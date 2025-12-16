use std::env;

use crate::seed;

// Environment variable to define to set a global salt
// defaults to no salt
const GLOBAL_SALT_ENV_VAR: &str = "FLAG_GLOBAL_SALT";


pub fn compute_flag_from_context(context: &str, step: &str, salt: Option<String>) -> String {
    let mut digest = md5::Context::new();

    digest.consume(step);

    if let Ok(global_salt) = env::var(GLOBAL_SALT_ENV_VAR) {
        digest.consume(global_salt);
    }

    if let Some(s) = salt {
        digest.consume(s);
    }

    digest.consume(context);

    format!("{:x}", digest.finalize())
}


pub fn compute_flag(step: &str, salt: Option<String>) -> String {
    compute_flag_from_context(&seed::get_or_null(), step, salt)
}


pub fn format_flag(step: &str, salt: Option<String>) -> String {
    format!("flag({step}).{}", compute_flag(step, salt))
}

pub fn format_flag_from_context(context: &str, step: &str, salt: Option<String>) -> String {
    format!("flag({step}).{}", compute_flag_from_context(context, step, salt))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_environment_based_flag() {
        assert_eq!(compute_flag("example", None),
            "5f1b958992ca66c09c0ac9170fce85de");
        assert_eq!(compute_flag("example", Some("app noise".to_string())),
            "c37b2bf9e83b0c886c166bbb7e28c8fe");
    }

    #[test]
    fn same_flag_with_noise() {
        assert_eq!(compute_flag_from_context("segg1545", "example", Some("not a flag".to_string())),
            "54ed3a6a399869c751820aee6046668b");
    }

    #[test]
    fn example_flag() {
        assert_eq!(compute_flag_from_context("segg1545", "example", None), "5f1b958992ca66c09c0ac9170fce85de");
    }
}
