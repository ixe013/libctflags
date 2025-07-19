use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// On importe la fonction Rust originale qu'on veut exposer
use crate::flags::format_flag;

#[unsafe(no_mangle)]
pub extern "C" fn ctflags_format_flag(step: *const c_char, salt: *const c_char) -> *const c_char {
    unsafe {
        let step_str = CStr::from_ptr(step).to_str().expect("step contains invalid UTF-8");

        // Conversion du `salt` (optionnel)
        // Si le pointeur C est nul, on le traite comme `None` en Rust.
        let salt_option = if salt.is_null() {
            None
        } else {
            let salt_str = CStr::from_ptr(salt).to_str().expect("salt contains invalid UTF-8");
            Some(salt_str.to_string())
        };

        // Appel de la fonction Rust originale
        let result_string = format_flag(step_str, salt_option);

        // Conversion du résultat pour le retour en C++
        let c_result = CString::new(result_string).unwrap();
        c_result.into_raw()
    }
}
