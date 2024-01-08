use dependencies_sync::once_cell;
use dependencies_sync::rust_i18n::{self, i18n};
i18n!("locales");

pub use init_log_dir::*;

pub use wait_for_terminate_signal::*;
pub use shutdown_cancelation_token::*;

pub use set_tls_configs::*;
pub use init_managers::*;
pub use build_runtime::*;

mod init_log_dir;
mod set_tls_configs;

mod init_managers;
mod build_runtime;

mod wait_for_terminate_signal;
mod shutdown_cancelation_token;