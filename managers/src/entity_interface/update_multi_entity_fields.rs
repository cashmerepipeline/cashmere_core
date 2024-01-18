use cash_result::OperationResult;
use manage_define::cashmere::EntityFieldEdit;

pub async fn update_multi_entity_fields(
  edits: &Vec<EntityFieldEdit>,
  account_id: &str,
) -> Result<OperationResult, OperationResult> {
  entity::update_multi_entity_fields(edits, account_id).await
}