use dependencies_sync::tonic::Status;

use crate::can_collection_read;

async fn validate_collection_can_read(
    manage_id: &i32,
    role_group: &String,
) -> Result<(), Status> {
    if !can_collection_read(&manage_id.to_string(), role_group).await {
        return Err(Status::unauthenticated(t!("用户不具有集合可写权限")));
    }

    Ok(())
}
