use pyo3::prelude::*;
use crate::{flags, seed};

#[pyfunction]
#[pyo3(signature = (step, salt=None))]
fn format_flag(step: &str, salt: Option<String>) -> String {
    flags::format_flag(step, salt)
}

#[pyfunction]
#[pyo3(signature = (context, step, salt=None))]
fn format_flag_from_string_context(context: &str, step: &str, salt: Option<String>) -> String {
    flags::format_flag_from_string_context(context, step, salt)
}

#[pyfunction]
fn get_seed_or_null() -> String {
    seed::get_or_null()
}

#[pymodule]
fn ctflags(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(format_flag, m)?)?;
    m.add_function(wrap_pyfunction!(format_flag_from_string_context, m)?)?;
    m.add_function(wrap_pyfunction!(get_seed_or_null, m)?)?;
    Ok(())
}
