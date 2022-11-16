use async_trait::async_trait;
use bson::{doc, Document};
use linked_hash_map::LinkedHashMap;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use property_field::{FieldDataType, PropertyField};
use view;

#[async_trait]
pub trait HandleNewSchemaField {
    /// 新建管理属性
    async fn handle_new_schema_field(
        &self,
        request: Request<NewSchemaFieldRequest>,
    ) -> Result<Response<NewSchemaFieldResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        if !view::can_manage_write(&account_id, &role_group, &MANAGES_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let manage_id: i32 = request.get_ref().manage_id.parse().unwrap();
        let field: &SchemaField = request.get_ref().field.as_ref().unwrap();

        let name_bytes = field.name_map.clone();
        let name_doc = Document::from_reader(&mut name_bytes.as_slice()).unwrap();
        let name: LinkedHashMap<String, String> = bson::from_document(name_doc).unwrap();

        let new_field: PropertyField = PropertyField {
            id: field.id,
            name_map: name,
            data_type: FieldDataType::from(field.data_type.clone()),
            removed: false,
        };

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc.get_manager_by_id(manage_id).await.unwrap();
        let result = manager.new_schema_field(new_field, &account_id).await;

        match result {
            Ok(_r) => Ok(Response::new(NewSchemaFieldResponse {
                result: "ok".to_string(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
