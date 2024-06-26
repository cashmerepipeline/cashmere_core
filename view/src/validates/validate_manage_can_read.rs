use dependencies_sync::tonic::Status;
use dependencies_sync::rust_i18n::{self, t};

use crate::can_manage_read;

pub async fn validate_manage_can_read(
    manage_id: &str,
    role_group: &String,
) -> Result<(), Status>{

    if !can_manage_read(
        &manage_id.to_string(),
        role_group,
    )
    .await
    {
        return Err(Status::unauthenticated(t!("用户不具有集合可写权限")));
    }

    Ok(())
}
