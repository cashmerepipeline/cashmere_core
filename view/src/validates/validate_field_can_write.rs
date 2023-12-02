use dependencies_sync::tonic::Status;

use crate::can_field_read;

pub async fn validate_field_can_write(
    manage_id: &i32,
    field_id: &String,
    role_group: &String,
) -> Result<(), Status> {
    if !can_field_read(&manage_id, field_id, role_group).await {
        return Err(Status::unauthenticated(t!("用户不具有集合可写权限")));
    }

    Ok(())
}
