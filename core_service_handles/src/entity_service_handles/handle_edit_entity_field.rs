use dependencies_sync::bson;
use dependencies_sync::bson::{doc, Document};
use dependencies_sync::tonic::async_trait;
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::prost::bytes::Buf;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;

use manage_define::general_field_ids::*;

use managers::manager_trait::ManagerTrait;
use request_utils::request_account_context;



use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleEditEntityField {
    /// 编辑修改实体属性
    async fn handle_edit_entity_field(
        &self,
        request: Request<EditEntityFieldRequest>,
    ) -> UnaryResponseResult<EditEntityFieldResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_edit_entity_field)
            .await
    }
}

async fn validate_view_rules(
    request: Request<EditEntityFieldRequest>,
) -> Result<Request<EditEntityFieldRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = &request.get_ref().manage_id;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if let Err(e) =
            view::validates::validate_collection_can_write(&manage_id, &role_group).await
        {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<EditEntityFieldRequest>,
) -> Result<Request<EditEntityFieldRequest>, Status> {
    let manage_id = &request.get_ref().manage_id;
    let entity_id = &request.get_ref().entity_id;
    let field_id = &request.get_ref().field_id;
    // bson bytes {field_id:new_value}
    let new_value = &request.get_ref().new_value;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(*manage_id).unwrap();
    // 实体是否存在
    if manager.entity_exists(&doc! {
        "_id": entity_id,
    }).await.is_none(){
        return Err(Status::invalid_argument(format!(
            "{}: {}-{}, {}",
            t!("实体不存在"),
            manage_id,
            entity_id,
            "edit_entity_field"
        )));
    }

    // 描写属性是否存在
    let fields = manager.get_manage_schema().await;
    if !manager.has_schema_field(field_id).await{
        return Err(Status::invalid_argument(format!(
            "{}: {}-{}, {}",
            t!("属性不存在"),
            manage_id,
            field_id,
            "edit_entity_field"
        )));
    }

    // 验证新值
    let new_value_doc: Document = if let Ok(r) = bson::from_slice(new_value){
        r
    }else {
        return Err(Status::invalid_argument(format!(
            "{}:  {}",
            t!("新值反序列化失败"),
            "edit_entity_field"
        )));
    };
    // 只能有一个field
    if new_value_doc.keys().count() != 1{
        return Err(Status::invalid_argument(format!(
            "{}:  {}",
            t!("新值只能有一个field"),
            "edit_entity_field"
        )))
    }
    let new_field =
        new_value_doc.keys().next().expect(t!("取得新值field失败").as_str());
    // 新值field必须与目标field一致
    if field_id != new_field{
        return Err(Status::invalid_argument(format!(
            "{}:  {}",
            t!("新值field必须与目标field一致"),
            "edit_entity_field"
        )));
    }

    let new_value_bson = new_value_doc.get(field_id).unwrap();
    let new_t = new_value_bson.element_type();
    let f_t = fields.iter().find(|f| &f.id.to_string() == field_id).unwrap().get_element_type();
    // field类型需要匹配
    if new_t != f_t{
        return Err(Status::invalid_argument(format!(
            "{}:  {}",
            t!("新值field类型需要与目标field类型一致"),
            "edit_entity_field"
        )));
    }

    Ok(request)
}

async fn handle_edit_entity_field(
    request: Request<EditEntityFieldRequest>,
) -> UnaryResponseResult<EditEntityFieldResponse> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata());

    let manage_id = &request.get_ref().manage_id;
    let entity_id = &request.get_ref().entity_id;
    let field_id = &request.get_ref().field_id;
    // bson bytes {field_id:new_value}
    let new_value = &request.get_ref().new_value;

    let mut modify_doc = match Document::from_reader(new_value.reader()) {
        Ok(v) => {
            let t_v = v.get(field_id);
            if t_v.is_some() {
                v
            } else {
                return Err(Status::data_loss("新值不能为空"));
            }
        }
        Err(_) => return Err(Status::data_loss("新值不能为空")),
    };


    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(*manage_id).unwrap();
    let query_doc = doc! {
        ID_FIELD_ID.to_string():entity_id,
    };

    let result = manager
        .update_entity_field(query_doc, &mut modify_doc, &account_id)
        .await;

    match result {
        Ok(_r) => Ok(Response::new(EditEntityFieldResponse {
            result: "ok".to_string(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
