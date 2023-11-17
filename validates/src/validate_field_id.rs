use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::Status;

use majordomo::get_majordomo;
use managers::ManagerTrait;

/// zh: 验证管理字段是否存在
pub async fn validate_field_id(manage_id: &i32, field_id: &String) -> Result<(), Status> {
    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(*manage_id).unwrap();

    let field_id = if let Ok(r) = field_id.parse::<i32>(){
        r
    }else{
        return Err(Status::invalid_argument(format!(
            "{}: {}-{}, {}",
            t!("属性ID格式错误"),
            manage_id,
            field_id,
            "validate_field_id"
        )));
    };

    if !manager.has_schema_field(field_id).await{
        return Err(Status::invalid_argument(format!(
            "{}: {}-{}, {}",
            t!("属性不存在"),
            manage_id,
            field_id,
            "validate_field_id"
        )));
    }

    Ok(())
}
