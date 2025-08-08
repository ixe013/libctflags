use std::env;

use crate::seed;

// Environment variable to define to set a global salt
// defaults to no salt
const GLOBAL_SALT_ENV_VAR: &str = "FLAG_GLOBAL_SALT";


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
        unsafe { //Because POSIX makes this not thread safe
            env::set_var(seed::FILE_NAME_ENVIRONMENT_VAR, ".__example_flag");
            env::set_var(GLOBAL_SALT_ENV_VAR, "Hiver2025");
        }

        let _ = seed::set("example");

        assert_eq!(compute_flag("example", None), "70a72d19ef800c3e0941ff41bd0f4db7");

        let _ = seed::clear();
    }
}
