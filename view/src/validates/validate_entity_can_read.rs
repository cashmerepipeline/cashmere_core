use dependencies_sync::tonic::Status;

use crate::can_entity_read;

pub async fn validate_entity_can_read(
    manage_id: &str,
    role_group: &String,
) -> Result<(), Status>{

    if !can_entity_read(
        &manage_id.to_string(),
        role_group,
    )
    .await
    {
        return Err(Status::unauthenticated(t!("用户不具有集合可写权限")));
    }

    Ok(())
}
