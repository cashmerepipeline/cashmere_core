use dependencies_sync::once_cell;
use dependencies_sync::rust_i18n::{self, i18n, t};
i18n!("locales");

pub use init_log_dir::*;
pub use wait_for_terminate_signal::*;
pub use get_tls_configs::*;
pub use init_managers::*;

mod init_log_dir;
mod wait_for_terminate_signal;
mod get_tls_configs;
mod init_managers;