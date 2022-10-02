use tonic::metadata::MetadataMap;

use crate::ROLE_GROUP_NAME;

pub fn get_current_role(metadata: &MetadataMap) -> Option<String> {
    match metadata.get(ROLE_GROUP_NAME) {
        Some(role) => {
            let role = role.to_str().unwrap().to_string();
            Some(role)
        }
        None => None,
    }
}
