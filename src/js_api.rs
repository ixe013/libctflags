use napi_derive::napi;
use crate::{flags, seed};

#[napi(js_name = "formatFlag")]
pub fn format_flag(step: String, salt: Option<String>) -> String {
    flags::format_flag(&step, salt)
}

#[napi(js_name = "formatFlagFromContext")]
pub fn format_flag_from_context(context: String, step: String, salt: Option<String>) -> String {
    flags::format_flag_from_context(&context, &step, salt)
}

#[napi(js_name = "getSeedOrNull")]
pub fn get_seed_or_null() -> String {
    seed::get_or_null()
}
