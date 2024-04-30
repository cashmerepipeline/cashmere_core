use dependencies_sync::rust_i18n::{self, i18n};
i18n!("locales");

pub mod types;

pub use send_stream_response::*;
pub use send_stream_error::*;
pub use get_constants::*;
pub use hard_codes_utils_macro::*;

mod get_constants;
mod send_stream_response;
mod send_stream_error;
mod hard_codes_utils_macro;