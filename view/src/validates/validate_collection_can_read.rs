use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::Status;

use crate::can_collection_read;

pub async fn validate_collection_can_read(
    manage_id: &str,
    role_group: &str,
) -> Result<(), Status> {
    if !can_collection_read(manage_id, role_group).await {
        return Err(Status::unauthenticated(t!("用户不具有集合可写权限")));
    }

    Ok(())
}
