use bson::Document;

use cash_result::OperationResult;

use manage_define::manage_ids::ACCOUNTS_MANAGE_ID;
use managers::traits::ManagerTrait;

// 取得账户密钥加密串
pub async fn get_account_entity_doc(account_id: &String) -> Result<Document, OperationResult> {
    let majordomo_lock_arc = majordomo::get_majordomo().await;
    let manager_arc = majordomo_lock_arc
        .get_manager_by_id(ACCOUNTS_MANAGE_ID)
        .await
        .unwrap();

    manager_arc.get_entity_by_id(account_id).await
}
