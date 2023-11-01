use dependencies_sync::bson::{self, Bson, Document};

use dependencies_sync::futures::TryFutureExt;

use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::{Request, Response, Status};


use dependencies_sync::tonic::async_trait;


use majordomo::get_majordomo;
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::ManagerTrait;
use request_utils::request_account_context;




#[async_trait]
pub trait HandleNewTemplate {
    // 实体模板
    async fn handle_new_entity_template(
        &self,
        request: Request<NewTemplateRequest>,
    ) -> Result<Response<NewTemplateResponse>, Status> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_new_entity_template)
            .await
    }
}

async fn validate_view_rules(
    request: Request<NewTemplateRequest>,
) -> Result<Request<NewTemplateRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = ENTITIY_TEMPLATES_MANAGE_ID;
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
    request: Request<NewTemplateRequest>,
) -> Result<Request<NewTemplateRequest>, Status> {
    Ok(request)
}

async fn handle_new_entity_template(
    request: Request<NewTemplateRequest>,
) -> Result<Response<NewTemplateResponse>, Status> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata());

    let manage_id = &request.get_ref().manage_id;
    let fields = &request.get_ref().fields;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc
        .get_manager_by_id(TEMPLATES_MANAGE_ID)
        .unwrap();

    if let Err(e) = manager.validate_data_fields(fields).await {
        return Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        )));
    }

    let fields: Vec<Bson> = fields
        .iter()
        .map(|x| {
            let b = x.clone();
            let d = Document::from_reader(&mut b.as_slice()).unwrap();
            bson::to_bson(&d).unwrap()
        })
        .collect();

    let new_id = if let Some(r) = manager.get_new_entity_id(&account_id).await{
        r
    }else{
        return Err(Status::aborted(format!("{}: {}", t!("获取新ID失败"), "new_comment")));
    };

    let mut new_doc = Document::new();
    new_doc.insert("_id", new_id);
    new_doc.insert(ID_FIELD_ID.to_string(), new_id);
    new_doc.insert(TEMPLATES_MANAGE_ID_FIELD_ID.to_string(), manage_id);
    new_doc.insert(TEMPLATES_PRESETS_FIELD_ID.to_string(), fields);

    let result = manager
        .sink_entity(&mut new_doc, &account_id, &role_group)
        .await;

    match result {
        Ok(_r) => Ok(Response::new(NewTemplateResponse {
            result: "ok".to_string(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
