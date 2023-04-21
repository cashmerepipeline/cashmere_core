use async_trait::async_trait;
use bson::{doc, Document};
use linked_hash_map::LinkedHashMap;
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;
use property_field::{FieldDataType, PropertyField};

use tonic::{Request, Response, Status};
use view;

#[async_trait]
pub trait HandleNewSchemaField {
    /// 新建管理属性
    async fn handle_new_schema_field(
        &self,
        request: Request<NewSchemaFieldRequest>,
    ) -> Result<Response<NewSchemaFieldResponse>, Status> {
        let (account_id, _groups, role_group) =
            request_account_context(&request.metadata());

        if !view::can_manage_write(&account_id, &role_group, &MANAGES_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let manage_id = request.get_ref().manage_id;
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
