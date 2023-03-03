use crate::account_service_handles::get_account_entity_doc::get_account_entity_doc;
use crate::{ChangeOwnPasswordRequest, ChangeOwnPasswordResponse, UnaryResponseResult};
use async_trait::async_trait;
use bson::{doc, Document};
use majordomo::get_majordomo;
use manage_define::cashmere::Name;
use manage_define::field_ids::{
    ACCOUNTS_PASSWORD_FIELD_ID, ACCOUNTS_PHONE_AREA_CODE_FIELD_ID, ACCOUNTS_PHONE_FIELD_ID,
};
use manage_define::general_field_ids::{
    COMMENTS_FIELD_ID, DATAS_FIELD_ID, DATAS_REMOVED_FIELD_ID, ENTITY_REMOVED_FIELD_ID,
    ID_FIELD_ID, NAME_MAP_FIELD_ID,
};
use manage_define::manage_ids::ACCOUNTS_MANAGE_ID;
use managers::traits::ManagerTrait;
use managers::utils::make_new_entity_document;
use tonic::{Request, Response, Status};

#[async_trait]
pub trait HandleChangeOwnPassword {
    async fn handle_change_own_password(
        &self,
        request: Request<ChangeOwnPasswordRequest>,
    ) -> UnaryResponseResult<ChangeOwnPasswordResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let old_password = &request.get_ref().old_password;
        let new_password = &request.get_ref().new_password;

        let manage_id = ACCOUNTS_MANAGE_ID;
        // 实体可写检查
        if !view::can_entity_write(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated(t!("用户不具有修改实体权限")));
        }
        // 字段可写检查
        if !view::can_field_write(&account_id, &role_group, &manage_id.to_string(), &ACCOUNTS_PASSWORD_FIELD_ID.to_string()).await {
            return Err(Status::unauthenticated(t!("用户不具有修改密码权限")));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(ACCOUNTS_MANAGE_ID)
            .await
            .unwrap();

        // 帐号存在性检查
        let query_doc = doc! {
            ID_FIELD_ID.to_string():account_id.clone()
        };
        if !manager.entity_exists(&query_doc).await {
            return Err(Status::data_loss(format!(
                "{}: {}",
                t!("帐号不存在"),
                account_id
            )));
        }

        let account_doc = match get_account_entity_doc(&account_id).await {
            Ok(d) => d,
            Err(e) => {
                return Err(Status::data_loss(format!(
                    "{} {} {}",
                    t!("取得账户数据错误"),
                    e.operation(),
                    e.details()
                )));
            }
        };

        // 验证旧密码
        let password_hash = match account::get_account_passwd_hash(&account_doc) {
            Some(d) => d,
            None => return Err(Status::data_loss(t!("取得账户密码失败"))),
        };
        let pw_ok = (auth::jwt::verify_passwd(old_password, &password_hash).await).unwrap_or(false);
        if !pw_ok {
            return Err(Status::permission_denied(t!("旧密码错误")));
        }

        let encrypt_password = auth::jwt::hash_password(new_password).await.unwrap();

        let query_doc = doc! {
            ID_FIELD_ID.to_string(): account_id.clone()
        };
        let mut modify_doc = doc! {};
        modify_doc.insert(ACCOUNTS_PASSWORD_FIELD_ID.to_string(), encrypt_password);

        let result = manager
            .update_entity_field(query_doc, modify_doc, &account_id)
            .await;

        match result {
            Ok(_r) => {
                info!("{}: {}", t!("更新帐号密码成功"), account_id);
                Ok(Response::new(ChangeOwnPasswordResponse {
                    result: "ok".to_string(),
                }))
            }
            Err(e) => {
                error!("更新帐号密码发生错误--{}", account_id);
                Err(Status::aborted(format!(
                    "{} {}",
                    e.operation(),
                    e.details()
                )))
            }
        }
    }
}
