use crate::seed::get_or_null; // <--- On importe la fonction nécessaire
use std::ffi::CString;
use std::os::raw::c_char;


#[unsafe(no_mangle)]
pub extern "C" fn ctflags_get_seed_or_null() -> *const c_char {
    let rust_string = get_or_null();

    // Si la chaîne est vide, on retourne un pointeur nul,
    // ce qui est une convention courante en C.
    if rust_string.is_empty() {
        return std::ptr::null_mut();
    }

    // Convertit la String Rust en CString (qui garantit une terminaison par un nul).
    // .unwrap() est sûr ici si vous savez que votre chaîne Rust ne contient pas de `\0` au milieu.
    let c_string = CString::new(rust_string).unwrap();

    // Transfère la possession de la mémoire de Rust vers le C.
    // Rust "oublie" cette allocation, empêchant une double libération.
    c_string.into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn ctflags_free_string(ptr: *const c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _c_string = CString::from_raw(ptr as *mut c_char);
    }
}

