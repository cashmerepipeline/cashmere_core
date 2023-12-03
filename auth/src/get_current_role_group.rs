use dependencies_sync::{tonic::metadata::MetadataMap, log::debug};
use dependencies_sync::rust_i18n::{self, t};

use crate::ROLE_GROUP_NAME;

pub fn get_current_role(metadata: &MetadataMap) -> Option<String> {
    debug!("{}: {:?}", t!("取得角色"), metadata);
    match metadata.get(ROLE_GROUP_NAME) {
        Some(role) => {
            #[cfg(debug_assertions)]
            debug!("{}", t!("取得角色组"));

            let role = role.to_str().unwrap().to_string();
            Some(role)
        }
        None => None,
    }
}
